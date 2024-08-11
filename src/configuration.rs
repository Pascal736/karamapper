use anyhow::{anyhow, Context as _, Result};
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::keys::Key;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub simple_remaps: SimpleRemaps,
    pub layers: Layers,
    pub layer_assignments: LayerAssignments,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleRemap {
    pub from: Key,
    pub to: Vec<Key>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleRemaps {
    pub remaps: Vec<SimpleRemap>,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Layer {
    pub name: String,
    pub keys: Vec<Key>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layers {
    pub layers: Vec<Layer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Command {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LayerShift {
    pub move_layer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LayerRemap {
    pub to: Vec<Key>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Action {
    Command(Command),
    LayerRemap(LayerRemap),
    LayerShift(LayerShift),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LayerAssignment {
    pub layer: Layer,
    pub key: Key,
    pub action: Action,
    pub next_layer: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct LayerAssignments {
    pub assignments: Vec<LayerAssignment>,
}

impl SimpleRemaps {
    pub fn from_toml(value: &Value) -> Result<Self> {
        let remaps = value
            .as_table()
            .context("Invalid remaps format")?
            .iter()
            .map(|(from_key, to_key)| {
                let from = Key::try_from(from_key.as_str())
                    .with_context(|| format!("Invalid key in remaps: {}", from_key))?;
                let to = match to_key {
                    Value::String(to_str) => vec![Key::try_from(to_str.as_str())
                        .with_context(|| format!("Invalid value in remaps: {}", to_str))?],
                    _ => {
                        return Err(anyhow::anyhow!(
                            "Expected string for remap value, got: {:?}",
                            to_key
                        ))
                    }
                };
                Ok(SimpleRemap { from, to })
            })
            .collect::<Result<Vec<SimpleRemap>>>()?;
        Ok(SimpleRemaps { remaps })
    }
}

impl Layers {
    pub fn from_toml(value: &Value) -> Result<Self> {
        let layers = value
            .as_table()
            .context("Invalid layers format")?
            .iter()
            .map(|(layer_name, layer_keys)| {
                let keys_str = layer_keys.as_str().context(format!(
                    "Expected string for layer keys, got: {:?}",
                    layer_keys
                ))?;

                let keys = keys_str
                    .split('+')
                    .map(|key| {
                        Key::try_from(key).with_context(|| format!("Invalid key in layer: {}", key))
                    })
                    .collect::<Result<Vec<Key>>>()?;

                Ok(Layer {
                    name: layer_name.to_string(),
                    keys,
                })
            })
            .collect::<Result<Vec<Layer>>>()?;
        Ok(Layers { layers })
    }
}

impl Configuration {
    pub fn from_toml(value: &Value) -> Result<Self> {
        let remaps = value
            .get("simple_remaps")
            .context("Missing remaps in configuration")?;
        let remaps = SimpleRemaps::from_toml(remaps)?;

        let layers = value
            .get("layers")
            .context("Missing layers in configuration")?;
        let layers = Layers::from_toml(layers)?;

        let layer_assignments = LayerAssignments::from_toml(value, layers.layers.clone())?;

        Ok(Configuration {
            simple_remaps: remaps,
            layers,
            layer_assignments,
        })
    }
}

impl Action {
    fn from_toml(value: &Value) -> Result<Self> {
        if let Some(command) = value.get("command").and_then(|v| v.as_str()) {
            Ok(Action::Command(Command {
                value: command.to_string(),
            }))
        } else if let Some(remap) = value.get("remap").and_then(|v| v.as_str()) {
            let keys = Self::parse_keys(remap)?;
            Ok(Action::LayerRemap(LayerRemap { to: keys }))
        } else if let Some(move_layer) = value.get("move_layer").and_then(|v| v.as_str()) {
            Ok(Action::LayerShift(LayerShift {
                move_layer: move_layer.to_string(),
            }))
        } else {
            Err(anyhow!("Unknown action type in TOML"))
        }
    }
    fn parse_keys(remap: &str) -> Result<Vec<Key>> {
        remap
            .split('+')
            .map(|s| s.parse::<Key>().map_err(|_| anyhow!("Invalid key: {}", s)))
            .collect()
    }
}

impl LayerAssignment {
    pub fn from_toml(value: &Value, layer: Layer) -> Result<Vec<Self>> {
        let mut assignments = Vec::new();
        let table = value
            .as_table()
            .ok_or_else(|| anyhow!("Invalid TOML format"))?;

        for (key_str, value) in table {
            let key = key_str
                .parse::<Key>()
                .map_err(|_| anyhow!("Invalid key: {}", key_str))?;

            let action = Action::from_toml(value)?;

            let next_layer = value
                .get("next_layer")
                .and_then(|v| v.as_str())
                .map(String::from);

            let description = value
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from);

            assignments.push(LayerAssignment {
                layer: layer.clone(),
                key,
                action,
                next_layer,
                description,
            });
        }

        return Ok(assignments);
    }
}

impl LayerAssignments {
    pub fn from_toml(value: &Value, layers: Vec<Layer>) -> Result<Self> {
        let mut assignments: Vec<LayerAssignment> = vec![];
        for layer in layers.into_iter() {
            let assignment_value = Self::get_assignments_for_layer(&layer.clone().name, value)?;
            let assignments_layer = LayerAssignment::from_toml(&assignment_value, layer)?;
            assignments.extend(assignments_layer);
        }
        Ok(Self { assignments })
    }

    fn get_assignments_for_layer(name: &str, value: &Value) -> Result<Value> {
        let table = value
            .as_table()
            .ok_or_else(|| anyhow!("Expected a table, but found something else"))?;
        if let Some((_, matched_value)) = table.iter().find(|(key, _)| key.starts_with(name)) {
            return Ok(matched_value.clone());
        }
        Err(anyhow!("Layer not found: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_action() -> anyhow::Result<()> {
        let toml_str = r#"
            command = "hello"
            "#;

        let expected = Action::Command(Command {
            value: String::from("hello"),
        });
        let toml_value: Value = toml_str.parse()?;
        let action = Action::from_toml(&toml_value)?;

        assert_eq!(action, expected);

        Ok(())
    }

    #[test]
    fn test_parse_layer_assignment_command() -> anyhow::Result<()> {
        let toml_str = r#"
            h = { command = "hello" }
            "#;

        let layer = Layer {
            name: "layer1".to_string(),
            keys: vec![Key::LeftCommand],
        };

        let expected = vec![LayerAssignment {
            layer: layer.clone(),
            key: Key::H,
            action: Action::Command(Command {
                value: String::from("hello"),
            }),
            next_layer: None,
            description: None,
        }];
        let toml_value: Value = toml_str.parse()?;
        let layer_assignment = LayerAssignment::from_toml(&toml_value, layer)?;

        assert_eq!(layer_assignment, expected);

        Ok(())
    }
    #[test]
    fn test_parse_layer_assignment_remap() -> anyhow::Result<()> {
        let toml_str = r#"
            h = { remap = "j+v" }
            "#;

        let layer = Layer {
            name: "layer1".to_string(),
            keys: vec![Key::LeftControl],
        };

        let expected = vec![LayerAssignment {
            layer: layer.clone(),
            key: Key::H,
            action: Action::LayerRemap(LayerRemap {
                to: vec![Key::J, Key::V],
            }),
            next_layer: None,
            description: None,
        }];
        let toml_value: Value = toml_str.parse()?;
        let layer_assignment = LayerAssignment::from_toml(&toml_value, layer)?;

        assert_eq!(layer_assignment, expected);

        Ok(())
    }

    #[test]
    fn test_get_assignments_for_layer() -> anyhow::Result<()> {
        let toml_str = r#"
            [layer1]
            h = { command = "hello" }
            y = { command = "hello2", target_layer = "base", description = "These arguments are optional" }
            a = { remap =  "left_command+shift+left_arrow"}

            [layer2]
            a = { command = "app_launcher" }
            "#;
        let expected_toml = toml::toml! {
            h = { command = "hello" }
            y = { command = "hello2", target_layer = "base", description = "These arguments are optional" }
            a = { remap =  "left_command+shift+left_arrow"}
        };
        let layer_name = "layer1";
        let toml_value: Value = toml_str.parse()?;

        let layer_assignments =
            LayerAssignments::get_assignments_for_layer(layer_name, &toml_value)?;
        assert_eq!(layer_assignments, expected_toml.into());

        Ok(())
    }
    #[test]
    fn test_parse_layers_assignments() -> anyhow::Result<()> {
        let toml_str = r#"
            [layer1]
            h = { command = "hello" }
            y = { command = "hello2", next_layer= "layer1", description = "These arguments are optional" }

            [layer2]
            a = { command = "app_launcher" }
            "#;

        let layer1 = Layer {
            name: "layer1".to_string(),
            keys: vec![Key::LeftCommand],
        };
        let layer2 = Layer {
            name: "layer2".to_string(),
            keys: vec![Key::A],
        };

        let layers = vec![layer1.clone(), layer2.clone()];

        let toml_value: Value = toml_str.parse()?;
        let layer_assignments = LayerAssignments::from_toml(&toml_value, layers)?;

        let expected = LayerAssignments {
            assignments: vec![
                LayerAssignment {
                    layer: layer1.clone(),
                    key: Key::H,
                    action: Action::Command(Command {
                        value: String::from("hello"),
                    }),
                    next_layer: None,
                    description: None,
                },
                LayerAssignment {
                    layer: layer1.clone(),
                    key: Key::Y,
                    action: Action::Command(Command {
                        value: String::from("hello2"),
                    }),
                    next_layer: Some(layer1.clone().name),
                    description: Some(String::from("These arguments are optional")),
                },
                LayerAssignment {
                    layer: layer2.clone(),
                    key: Key::A,
                    action: Action::Command(Command {
                        value: String::from("app_launcher"),
                    }),
                    next_layer: None,
                    description: None,
                },
            ],
        };
        assert_eq!(layer_assignments, expected);
        Ok(())
    }

    #[test]
    fn test_remaps_from_toml() -> anyhow::Result<()> {
        let toml_str = r#"
        [simple_remaps]
        caps_lock= "left_command"
        v = "escape"
        "#;

        let toml_value: Value = toml_str.parse()?;
        let remaps = SimpleRemaps::from_toml(toml_value.get("simple_remaps").unwrap())?;

        assert_eq!(remaps.remaps.len(), 2);
        assert_eq!(remaps.remaps[0].from, Key::CapsLock);
        assert_eq!(remaps.remaps[0].to[0], Key::LeftCommand);
        assert_eq!(remaps.remaps[1].from, Key::V);
        assert_eq!(remaps.remaps[1].to[0], Key::Escape);
        Ok(())
    }

    #[test]
    fn test_layers_from_toml() -> Result<()> {
        let toml_str = r#"
            [layers]
            layer1 = "left_command"
            layer2 = "left_command+v"
            "#;

        let toml_value: Value = toml_str.parse()?;
        let layers = Layers::from_toml(toml_value.get("layers").unwrap())?;

        assert_eq!(layers.layers.len(), 2);
        assert_eq!(layers.layers[0].name, "layer1".to_string());
        assert_eq!(layers.layers[0].keys[0], Key::LeftCommand);

        assert_eq!(layers.layers[1].name, "layer2".to_string());
        assert_eq!(layers.layers[1].keys[0], Key::LeftCommand);
        assert_eq!(layers.layers[1].keys[1], Key::V);
        Ok(())
    }

    #[test]
    fn test_configuration_from_toml() -> Result<()> {
        let toml_str = r#"
            [simple_remaps]
            caps_lock = "left_command"

            [layers]
            layer1 = "left_command"
            layer2 = "left_command+v"

            [layer1]
            h = { command = "hello" }
            y = { command = "hello2", target_layer = "base", description = "These arguments are optional" }
            a = { remap =  "left_command+left_arrow"}
            n = { remap = "left_command+down_arrow", target_layer = "layer2", description = "These arguments are optional" }
            escape = { move_layer = "base"}

            [layer2]
            a = { command = "app_launcher" }
            "#;

        let toml_value: Value = toml_str.parse()?;
        let _config = Configuration::from_toml(&toml_value)?;

        Ok(())
    }
}
