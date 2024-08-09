use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::keys::Key;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub remaps: Remaps,
    pub commands: Commands,
    pub layers: Layers,
    pub layer_assignments: LayerAssingments,
}

impl Configuration {
    pub fn from_toml(value: &Value) -> Result<Self> {
        let remaps = value
            .get("remaps")
            .map(Remaps::from_toml)
            .unwrap_or_else(|| Ok(Remaps { remaps: vec![] }))?;

        let commands = value
            .get("commands")
            .map(Commands::from_toml)
            .unwrap_or_else(|| Ok(Commands { commands: vec![] }))?;

        let layers = value
            .get("layers")
            .map(Layers::from_toml)
            .unwrap_or_else(|| Ok(Layers { layers: vec![] }))?;

        let layer_assignments = LayerAssingments::from_toml(value, &layers)?;

        Ok(Configuration {
            remaps,
            commands,
            layers,
            layer_assignments,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Remaps {
    pub remaps: Vec<Remap>,
}

impl Remaps {
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
                Ok(Remap { from, to })
            })
            .collect::<Result<Vec<Remap>>>()?;
        Ok(Remaps { remaps })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Remap {
    pub from: Key,
    pub to: Vec<Key>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commands {
    pub commands: Vec<Command>,
}

impl Commands {
    pub fn from_toml(value: &Value) -> Result<Self> {
        let commands = value
            .as_table()
            .context("Invalid commands format")?
            .iter()
            .map(|(_, command_str)| match command_str {
                Value::String(value) => Ok(Command {
                    value: value.to_string(),
                }),
                _ => Err(anyhow::anyhow!(
                    "Expected string for command value, got: {:?}",
                    command_str
                )),
            })
            .collect::<Result<Vec<Command>>>()?;
        Ok(Commands { commands })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layers {
    pub layers: Vec<Layer>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
    name: String,
    keys: Vec<Key>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayerAssingments {
    layer_assingments: Vec<LayerAssingment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayerAssingment {
    layer: Layer,
    command: Command,
    next_layer: Option<Layer>,
    description: Option<String>,
}
impl LayerAssingments {
    pub fn from_toml(value: &Value, layers: &Layers) -> Result<Self> {
        let layer_assignments = value
            .as_table()
            .context("Invalid top-level format")?
            .iter()
            .filter_map(|(layer_name, layer_assignments_value)| {
                // Only consider entries that start with "layer" but aren't the "layers" section itself
                if layer_name.starts_with("layer") && !layer_name.eq("layers") {
                    Some(
                        layer_assignments_value
                            .as_table()
                            .context(format!("Invalid layer assignment for {}", layer_name))
                            .and_then(|layer_table| {
                                let assignments = layer_table
                                    .iter()
                                    .map(|(key, assignment_value)| {
                                        // Lookup the corresponding layer
                                        let layer = layers
                                            .layers
                                            .iter()
                                            .find(|l| &l.name == layer_name)
                                            .context(format!("Layer not found: {}", layer_name))?
                                            .clone();

                                        match assignment_value {
                                            Value::String(command_name) => {
                                                // Simple command assignment
                                                let command = Command {
                                                    value: command_name.clone(),
                                                };
                                                Ok(LayerAssingment {
                                                    layer: layer.clone(),
                                                    command,
                                                    next_layer: None,
                                                    description: None,
                                                })
                                            }
                                            Value::Table(command_table) => {
                                                // Complex command assignment
                                                let command_name = command_table
                                                    .get("command")
                                                    .and_then(|v| v.as_str())
                                                    .context(format!(
                                                        "Invalid command for key: {}",
                                                        key
                                                    ))?;
                                                let command = Command {
                                                    value: command_name.to_string(),
                                                };

                                                let layer_after = command_table
                                                    .get("next_layer")
                                                    .and_then(|v| v.as_str())
                                                    .map(|layer_name| {
                                                        layers
                                                            .layers
                                                            .iter()
                                                            .find(|l| l.name == layer_name)
                                                            .cloned()
                                                    })
                                                    .flatten();

                                                // Optional description
                                                let description = command_table
                                                    .get("description")
                                                    .and_then(|v| v.as_str())
                                                    .map(String::from);

                                                Ok(LayerAssingment {
                                                    layer: layer.clone(),
                                                    command,
                                                    next_layer: layer_after,
                                                    description,
                                                })
                                            }
                                            _ => Err(anyhow::anyhow!(
                                                "Invalid assignment for key: {}",
                                                key
                                            )),
                                        }
                                    })
                                    .collect::<Result<Vec<LayerAssingment>>>()?;
                                Ok(LayerAssingments {
                                    layer_assingments: assignments,
                                })
                            }),
                    )
                } else {
                    None
                }
            })
            .collect::<Result<Vec<LayerAssingments>>>()?;

        let layer_assignments = LayerAssingments {
            layer_assingments: layer_assignments
                .into_iter()
                .flat_map(|x| x.layer_assingments)
                .collect(),
        };

        Ok(layer_assignments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remaps_from_toml() -> anyhow::Result<()> {
        let toml_str = r#"
        [remaps]
        caps_lock= "hyper"
        v = "esc"
        "#;

        let toml_value: Value = toml_str.parse()?;
        let remaps = Remaps::from_toml(toml_value.get("remaps").unwrap())?;

        assert_eq!(remaps.remaps.len(), 2);
        assert_eq!(remaps.remaps[0].from, Key::CapsLock);
        assert_eq!(remaps.remaps[0].to[0], Key::Hyper);
        assert_eq!(remaps.remaps[1].from, Key::V);
        assert_eq!(remaps.remaps[1].to[0], Key::Esc);
        Ok(())
    }

    #[test]
    fn test_commands_from_toml() -> Result<()> {
        let toml_str = r#"
            [commands]
            hello = "sh -c \"echo Hello World\""
            hello2 = "sh -c \"echo Hello World 2\""
            "#;

        let toml_value: Value = toml_str.parse()?;
        let commands = Commands::from_toml(toml_value.get("commands").unwrap())?;

        assert_eq!(commands.commands.len(), 2);
        assert_eq!(
            commands.commands[0].value,
            "sh -c \"echo Hello World\"".to_string()
        );
        assert_eq!(
            commands.commands[1].value,
            "sh -c \"echo Hello World 2\"".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_layers_from_toml() -> Result<()> {
        let toml_str = r#"
            [layers]
            layer1 = "hyper"
            layer2 = "hyper+v"
            "#;

        let toml_value: Value = toml_str.parse()?;
        let layers = Layers::from_toml(toml_value.get("layers").unwrap())?;

        assert_eq!(layers.layers.len(), 2);
        assert_eq!(layers.layers[0].name, "layer1".to_string());
        assert_eq!(layers.layers[0].keys[0], Key::Hyper);

        assert_eq!(layers.layers[1].name, "layer2".to_string());
        assert_eq!(layers.layers[1].keys[0], Key::Hyper);
        assert_eq!(layers.layers[1].keys[1], Key::V);
        Ok(())
    }

    #[test]
    fn test_layer_assignments_from_toml() -> Result<()> {
        let toml_str = r#"
            [layers]
            layer1 = "hyper"
            layer2 = "hyper+v"

            [layer1]
            h = "hello"
            y = { command = "hello2", next_layer= "layer2", description = "These arguments are optional" }
            "#;

        let toml_value: Value = toml_str.parse()?;
        let layers = Layers::from_toml(toml_value.get("layers").unwrap())?;
        let layer_assignments = LayerAssingments::from_toml(&toml_value, &layers)?;

        assert_eq!(layer_assignments.layer_assingments.len(), 2);
        assert_eq!(
            layer_assignments.layer_assingments[0].layer.name,
            "layer1".to_string()
        );
        assert_eq!(
            layer_assignments.layer_assingments[0].command.value,
            "hello".to_string()
        );

        assert_eq!(
            layer_assignments.layer_assingments[1].layer.name,
            "layer1".to_string()
        );
        assert_eq!(
            layer_assignments.layer_assingments[1].command.value,
            "hello2".to_string()
        );
        assert_eq!(
            layer_assignments.layer_assingments[1]
                .description
                .as_ref()
                .unwrap(),
            "These arguments are optional"
        );
        assert_eq!(
            layer_assignments.layer_assingments[1]
                .next_layer
                .as_ref()
                .unwrap()
                .name,
            "layer2"
        );
        Ok(())
    }

    #[test]
    fn test_configuration_from_toml() -> Result<()> {
        let toml_str = r#"
            [remaps]
            caps_lock = "hyper"

            [commands]
            hello = "sh -c \"echo Hello World\""
            hello2 = "sh -c \"echo Hello World 2\""

            [layers]
            layer1 = "hyper"
            layer2 = "hyper+v"

            [layer1]
            h = "hello"
            y = { command = "hello2", next_layer= "base", description = "These arguments are optional" }
            "#;

        let toml_value: Value = toml_str.parse()?;
        let config = Configuration::from_toml(&toml_value)?;

        // Test Remaps
        assert_eq!(config.remaps.remaps.len(), 1);
        assert_eq!(config.remaps.remaps[0].from, Key::CapsLock);
        assert_eq!(config.remaps.remaps[0].to[0], Key::Hyper);

        // Test Commands
        assert_eq!(config.commands.commands.len(), 2);
        assert_eq!(
            config.commands.commands[0].value,
            "sh -c \"echo Hello World\"".to_string()
        );

        // Test Layers
        assert_eq!(config.layers.layers.len(), 2);
        assert_eq!(config.layers.layers[0].name, "layer1".to_string());

        // Test Layer Assignments
        assert_eq!(config.layer_assignments.layer_assingments.len(), 2);
        assert_eq!(
            config.layer_assignments.layer_assingments[0].layer.name,
            "layer1".to_string()
        );
        Ok(())
    }
}
