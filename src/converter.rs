use crate::configuration::*;
use crate::karabiner::*;
use crate::keys::Key;

pub const DEFAULT_PROFILE_NAME: &str = "Default";

pub fn convert_configuration(configuration: &Configuration) -> KarabinerConfig {
    let mut layer_rules: Vec<Rule> = configuration
        .layers
        .layers
        .iter()
        .filter(|l| l.name != BASE_LAYER)
        .map(|l| layer_to_rule(l.clone()))
        .collect();

    let rules: Vec<Rule> = configuration
        .layer_assignments
        .assignments
        .iter()
        .map(|a| layer_assignment_to_rule(a.clone()))
        .collect();

    layer_rules.extend(rules);

    let complex_modifications = ComplexModifications {
        rules: Some(layer_rules),
    };

    let devices = vec![Device {
        identifiers: DeviceIdentifiers::default(),
        simple_modifications: remaps_to_simple_modifications(configuration.simple_remaps.clone()),
    }];
    let name = DEFAULT_PROFILE_NAME.to_string();
    let selected = true;

    KarabinerConfig {
        profiles: vec![Profile {
            complex_modifications,
            devices: Some(devices),
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
            layer_assignment.next_layer,
        ),
        Action::Command(command) => Rule::set_command_in_layer(
            layer_assignment.layer.name,
            layer_assignment.key.into(),
            command.into(),
            layer_assignment.next_layer,
        ),
        Action::LayerShift(layer) => Rule::switch_layer(
            layer.move_layer,
            layer_assignment.layer.name,
            layer_assignment.key.into(),
        ),
    }
}

fn remaps_to_simple_modifications(remaps: SimpleRemaps) -> Vec<SimpleModification> {
    remaps
        .remaps
        .iter()
        .map(|m| remap_to_simple_modification(m.clone()))
        .collect()
}

fn remap_to_simple_modification(remap: SimpleRemap) -> SimpleModification {
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

impl From<Key> for FromKeyMapping {
    fn from(value: Key) -> Self {
        FromKeyMapping {
            key_code: value,
            modifiers: None,
        }
    }
}

impl From<Vec<Key>> for FromKeyMapping {
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
impl From<Vec<Key>> for ToKeyMapping {
    fn from(keys: Vec<Key>) -> Self {
        let key_code = keys.first().unwrap().clone();
        let modifiers = match keys.len() {
            1 => vec![],
            _ => keys.iter().skip(1).map(|k| k.clone()).collect(),
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
        let remaps = SimpleRemaps {
            remaps: vec![
                SimpleRemap {
                    from: Key::CapsLock,
                    to: vec![Key::LeftCommand],
                },
                SimpleRemap {
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
        assert_eq!(
            rule.manipulators.first().unwrap().from.key_code,
            Key::LeftCommand
        );
        assert_eq!(
            rule.manipulators.clone().first().unwrap().from.modifiers,
            None
        );
        assert_eq!(rule.description, Some("Change to layer1".to_string()));
        assert_eq!(rule.manipulators.first().unwrap().conditions, Some(vec![]));
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
        assert_eq!(
            rule.manipulators.first().unwrap().from.key_code,
            Key::LeftCommand
        );
        assert_eq!(
            rule.manipulators.first().unwrap().from.modifiers,
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
                to: vec![Key::Escape, Key::LeftShift],
            }),
            next_layer: Some(BASE_LAYER.to_string()),
            description: None,
        };

        let expected = Rule {
            description: Some("Remap h to escape".to_string()),
            enabled: true,
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition {
                    name: "layer1".to_string(),
                    condition_type: "variable_if".into(),
                    value: 1,
                }]),
                from: FromKeyMapping {
                    key_code: Key::H,
                    modifiers: None,
                },
                to: Some(vec![ManipulationTarget::KeyMapping(ToKeyMapping {
                    key_code: Key::Escape,
                    modifiers: vec![Key::LeftShift],
                })]),
                manipulator_type: "basic".into(),
                to_if_alone: None,
                to_after_key_up: None,
                to_delayed_action: Some(DelayedAction {
                    to_if_invoked: vec![SetVariable::new("layer1".to_string(), 0)],
                    to_if_canceled: vec![],
                }),
            }],
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
            next_layer: Some(base_layer.name),
            description: None,
        };

        let expected = Rule {
            description: Some("Run command open -a Terminal".to_string()),
            enabled: true,
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition {
                    name: "layer1".to_string(),
                    condition_type: "variable_if".into(),
                    value: 1,
                }]),
                from: FromKeyMapping {
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
                    to_if_invoked: vec![SetVariable::new("layer1".to_string(), 0)],
                }),
            }],
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
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition {
                    name: "layer1".to_string(),
                    condition_type: "variable_if".into(),
                    value: 1,
                }]),
                from: FromKeyMapping {
                    key_code: Key::H,
                    modifiers: None,
                },
                to: Some(vec![
                    ManipulationTarget::SetVariable(SetVariable::new("layer2".to_string(), 1)),
                    ManipulationTarget::SetVariable(SetVariable::new("layer1".to_string(), 0)),
                ]),
                manipulator_type: "basic".into(),
                to_if_alone: None,
                to_after_key_up: None,
                to_delayed_action: None,
            }],
        };

        let rule = layer_assignment_to_rule(layer_assignment);

        assert_eq!(rule, expected);
    }
}
