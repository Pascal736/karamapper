use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use toml::Value;

use crate::keys::Key;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub remaps: Remaps,
    pub commands: Commands,
    pub layers: Layers,
}

impl Configuration {
    pub fn from_toml(value: &Value) -> Result<Self> {
        let remaps = match value.get("remaps") {
            Some(remaps) => Remaps::from_toml(remaps)?,
            None => Remaps { remaps: vec![] },
        };

        let commands = value
            .get("commands")
            .map(Commands::from_toml)
            .unwrap_or_else(|| Ok(Commands { commands: vec![] }))?;

        let layers = value
            .get("layers")
            .map(Layers::from_toml)
            .unwrap_or_else(|| Ok(Layers { layers: vec![] }))?;

        Ok(Configuration {
            remaps,
            commands,
            layers,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
impl Default for Layers {
    fn default() -> Self {
        Self { layers: vec![] }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
    name: String,
    activation_keys: Vec<Key>,
    assignments: LayerAssingments,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayerAssingments {
    layer_assingments: Vec<LayerAssingment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayerAssingment {
    keys: Vec<Key>,
    command: Command,
    next_layer: Option<Layer>,
    description: Option<String>,
}

impl LayerAssingment {
    fn parse_from_toml(value: &Value) -> Result<Self> {
        // Get each vale belww
        todo!()
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
    fn test_layer_assignments_from_toml() -> Result<()> {
        let toml_str = r#"
            layer.layer1.shortcut = hyper
            [layer.layer1.assignments]
            h = "hello"
            y = { command = "hello2", next_layer= "layer2", description = "These arguments are optional" }

            layer.layer2.shortcut = hyper+v
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

    #[test]
    fn test_missing_categories_is_ok() -> Result<()> {
        let toml_str = r#"
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

        assert_eq!(config.remaps.remaps, vec![]);
        Ok(())
    }

    #[test]
    fn test_not_defined_layer_is_error() -> Result<()> {
        let toml_str = r#"
            [commands]
            hello = "sh -c \"echo Hello World\""

            [layers]
            layer1 = "hyper"

            [wrong-layer]
            h = "hello"
            "#;

        let toml_value: Value = toml_str.parse()?;
        let result = LayerAssingments::from_toml(&toml_value, &Layers::default());
        assert!(result.is_err());
        Ok(())
    }
}
