use crate::configuration::*;
use crate::karabiner::*;
use crate::keys::Key;

pub const BASE_LAYER: &str = "base";
pub const DEFAULT_PROFILE_NAME: &str = "Default";

pub fn convert_configuration(configuration: &Configuration) -> Profiles {
    let complex_modifications = ComplexModifications {
        rules: configuration
            .layer_assignments
            .assignments
            .iter()
            .map(|a| layer_assignment_to_rule(a.clone()))
            .collect(),
    };

    let devices = vec![Device {
        identifiers: DeviceIdentifiers::default(),
        simple_modifications: configuration
            .remaps
            .remaps
            .iter()
            .map(|r| remap_to_simple_modification(r.clone()))
            .collect(),
    }];
    let name = DEFAULT_PROFILE_NAME.to_string();
    let selected = true;

    Profiles {
        profiles: vec![Profile {
            complex_modifications,
            devices,
            name,
            selected,
        }],
    }
}

pub fn layer_to_rule(layer: Layer) -> Rule {
    Rule::set_environment(layer.name, layer.keys.into())
}

fn layer_assignment_to_rule(layer_assignment: LayerAssignment) -> Rule {
    match layer_assignment.action {
        Action::LayerRemap(remaps) => Rule::set_keymapping_in_layer(
            layer_assignment.layer.name,
            layer_assignment.key.into(),
            remaps.to.into(),
            None,
        ),
        Action::Command(command) => Rule::set_command_in_layer(
            layer_assignment.layer.name,
            layer_assignment.key.into(),
            command.into(),
            layer_assignment.next_layer.map(|l| l.name),
        ),
        Action::LayerShift(layer) => Rule::switch_layer(
            layer.move_layer,
            layer_assignment.layer.name,
            layer_assignment.key.into(),
        ),
    }
}

fn remaps_to_simple_modifications(remaps: Remaps) -> Vec<SimpleModification> {
    remaps
        .remaps
        .iter()
        .map(|m| remap_to_simple_modification(m.clone()))
        .collect()
}

fn remap_to_simple_modification(remap: Remap) -> SimpleModification {
    SimpleModification {
        from: remap.from.into(),
        to: remap.to.iter().map(|k| k.clone().into()).collect(),
    }
}

impl From<Key> for SimpleKeyMapping {
    fn from(key: Key) -> Self {
        SimpleKeyMapping { key_code: key }
    }
}

impl From<Key> for Modifiers {
    fn from(key: Key) -> Self {
        Modifiers {
            mandatory: Some(vec![key]),
            optional: None,
        }
    }
}

impl From<Key> for KeyMapping {
    fn from(value: Key) -> Self {
        KeyMapping {
            key_code: value,
            modifiers: None,
        }
    }
}

impl From<Vec<Key>> for KeyMapping {
    fn from(keys: Vec<Key>) -> Self {
        let key_code = keys.first().unwrap().clone();

        let modifiers = match keys.len() {
            1 => None,
            _ => {
                let modifiers = Modifiers {
                    mandatory: Some(keys.iter().skip(1).map(|k| k.clone()).collect()),
                    optional: None,
                };

                Some(modifiers)
            }
        };

        Self {
            key_code,
            modifiers,
        }
    }
}

impl From<Command> for ShellCommand {
    fn from(command: Command) -> Self {
        ShellCommand {
            shell_command: command.value,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_remaps_to_simple_modifications() {
        let remaps = Remaps {
            remaps: vec![
                Remap {
                    from: Key::CapsLock,
                    to: vec![Key::LeftCommand],
                },
                Remap {
                    from: Key::V,
                    to: vec![Key::LeftCommand, Key::V],
                },
            ],
        };

        let simple_modifications = remaps_to_simple_modifications(remaps);

        assert_eq!(simple_modifications.len(), 2);
        assert_eq!(simple_modifications[0].from.key_code, Key::CapsLock);
        assert_eq!(simple_modifications[0].to.len(), 1);
        assert_eq!(simple_modifications[0].to[0].key_code, Key::LeftCommand);

        assert_eq!(simple_modifications[1].from.key_code, Key::V);
        assert_eq!(simple_modifications[1].to.len(), 2);
        assert_eq!(simple_modifications[1].to[0].key_code, Key::LeftCommand);
        assert_eq!(simple_modifications[1].to[1].key_code, Key::V);
    }

    #[test]
    fn test_layer_creates_rule() {
        let name = String::from("layer1");
        let keys = vec![Key::LeftCommand];

        let layer = Layer {
            name: name.clone(),
            keys: keys.clone(),
        };

        let expected_rule = Rule::set_environment(name, keys.into());
        let rule = layer_to_rule(layer);

        assert_eq!(rule, expected_rule);
        assert_eq!(rule.manipulators.from.key_code, Key::LeftCommand);
        assert_eq!(rule.manipulators.from.modifiers, None);
        assert_eq!(rule.description, Some("Change to layer1".to_string()));
    }

    #[test]
    fn test_layer_with_two_keys_creates_rule() {
        let name = String::from("layer1");
        let keys = vec![Key::LeftCommand, Key::V];

        let layer = Layer {
            name: name.clone(),
            keys: keys.clone(),
        };

        let expected_rule = Rule::set_environment(name, keys.into());
        let rule = layer_to_rule(layer);

        assert_eq!(rule, expected_rule);
        assert_eq!(rule.manipulators.from.key_code, Key::LeftCommand);
        assert_eq!(
            rule.manipulators.from.modifiers,
            Some(Modifiers {
                mandatory: Some(vec![Key::V]),
                optional: None,
            })
        );
    }

    #[test]
    fn test_layer_assignment_to_remap() {
        let layer_assignment = LayerAssignment {
            layer: Layer {
                name: "layer1".to_string(),
                keys: vec![Key::LeftCommand],
            },
            key: Key::H,
            action: Action::LayerRemap(LayerRemap {
                to: vec![Key::Escape],
            }),
            next_layer: None,
            description: None,
        };

        let expected = Rule {
            description: Some("Remap H to Escape".to_string()),
            enabled: true,
            manipulators: Manipulator {
                conditions: Some(vec![Condition {
                    name: "layer1".to_string(),
                    condition_type: "variable_if".into(),
                    value: 1,
                }]),
                from: KeyMapping {
                    key_code: Key::H,
                    modifiers: None,
                },
                to: Some(vec![ManipulationTarget::KeyMapping(KeyMapping {
                    key_code: Key::Escape,
                    modifiers: None,
                })]),
                manipulator_type: "basic".into(),
                to_if_alone: None,
                to_after_key_up: None,
                to_delayed_action: None,
            },
        };

        let rule = layer_assignment_to_rule(layer_assignment);

        assert_eq!(rule, expected);
    }
    #[test]
    fn test_layer_assignment_to_command() {
        let base_layer = Layer {
            name: BASE_LAYER.to_string(),
            keys: vec![],
        };
        let layer_assignment = LayerAssignment {
            layer: Layer {
                name: "layer1".to_string(),
                keys: vec![Key::LeftCommand],
            },
            key: Key::H,
            action: Action::Command(Command {
                value: String::from("open -a Terminal"),
            }),
            next_layer: Some(base_layer),
            description: None,
        };

        let expected = Rule {
            description: Some("Run command open -a Terminal".to_string()),
            enabled: true,
            manipulators: Manipulator {
                conditions: Some(vec![Condition {
                    name: "layer1".to_string(),
                    condition_type: "variable_if".into(),
                    value: 1,
                }]),
                from: KeyMapping {
                    key_code: Key::H,
                    modifiers: None,
                },
                to: Some(vec![ManipulationTarget::ShellCommand(ShellCommand {
                    shell_command: "open -a Terminal".to_string(),
                })]),
                manipulator_type: "basic".into(),
                to_if_alone: None,
                to_after_key_up: None,
                to_delayed_action: Some(DelayedAction {
                    to_if_canceled: vec![],
                    to_if_invoked: vec![SetVariable {
                        name: "layer1".to_string(),
                        value: 0,
                    }],
                }),
            },
        };

        let rule = layer_assignment_to_rule(layer_assignment);

        assert_eq!(rule, expected);
    }
    #[test]
    fn test_layer_assignment_to_layer_change() {
        let layer_assignment = LayerAssignment {
            layer: Layer {
                name: "layer1".to_string(),
                keys: vec![Key::LeftCommand],
            },
            key: Key::H,
            action: Action::LayerShift(LayerShift {
                move_layer: "layer2".into(),
            }),
            next_layer: None,
            description: None,
        };

        let expected = Rule {
            description: Some("Switch to layer2".to_string()),
            enabled: true,
            manipulators: Manipulator {
                conditions: Some(vec![Condition {
                    name: "layer1".to_string(),
                    condition_type: "variable_if".into(),
                    value: 1,
                }]),
                from: KeyMapping {
                    key_code: Key::H,
                    modifiers: None,
                },
                to: Some(vec![
                    ManipulationTarget::SetVariable(SetVariable {
                        name: "layer2".to_string(),
                        value: 1,
                    }),
                    ManipulationTarget::SetVariable(SetVariable {
                        name: "layer1".to_string(),
                        value: 0,
                    }),
                ]),
                manipulator_type: "basic".into(),
                to_if_alone: None,
                to_after_key_up: None,
                to_delayed_action: None,
            },
        };

        let rule = layer_assignment_to_rule(layer_assignment);

        assert_eq!(rule, expected);
    }
}
