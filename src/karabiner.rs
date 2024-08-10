use serde::{Deserialize, Serialize};

use crate::converter::BASE_LAYER;
use crate::keys::Key;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Profiles {
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Profile {
    pub complex_modifications: ComplexModifications,
    pub devices: Vec<Device>,
    pub name: String,
    pub selected: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ComplexModifications {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Rule {
    pub description: Option<String>,
    pub enabled: bool,
    pub manipulators: Vec<Manipulator>,
}

impl Rule {
    pub fn set_environment(name: String, from: KeyMapping) -> Self {
        Rule {
            description: Some(format!("Change to {}", name.to_lowercase())),
            enabled: true,
            manipulators: vec![Manipulator::set_environment(name, from)],
        }
    }

    pub fn set_keymapping_in_layer(
        layer: String,
        from: KeyMapping,
        to: KeyMapping,
        target_layer: Option<String>,
    ) -> Self {
        Self {
            description: Some(format!("Remap {} to {}", from.key_code, to.key_code)),
            enabled: true,
            manipulators: vec![Manipulator::set_keymapping_in_layer(
                layer,
                from,
                to,
                target_layer,
            )],
        }
    }

    pub fn set_command_in_layer(
        layer: String,
        from: KeyMapping,
        to: ShellCommand,
        target_layer: Option<String>,
    ) -> Self {
        Self {
            description: Some(format!("Run command {}", to.shell_command)),
            enabled: true,
            manipulators: vec![Manipulator::set_command_in_layer(
                layer,
                from,
                to,
                target_layer,
            )],
        }
    }

    pub fn switch_layer(target_layer: String, source_layer: String, from: KeyMapping) -> Self {
        Self {
            description: Some(format!("Switch to {}", target_layer)),
            enabled: true,
            manipulators: vec![Manipulator::switch_layer(target_layer, source_layer, from)],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Manipulator {
    pub conditions: Option<Vec<Condition>>,
    pub from: KeyMapping,
    pub to: Option<Vec<ManipulationTarget>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_delayed_action: Option<DelayedAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ManipulationTarget>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_alone: Option<Vec<KeyMapping>>,
    #[serde(rename = "type")]
    pub manipulator_type: String,
}

impl Manipulator {
    pub fn set_environment(name: String, from: KeyMapping) -> Self {
        Manipulator {
            conditions: Some(vec![]),
            from,
            to: Some(vec![ManipulationTarget::set_active(name)]),
            to_delayed_action: None,
            to_after_key_up: None,
            to_if_alone: None,
            manipulator_type: "basic".to_string(),
        }
    }

    pub fn set_keymapping_in_layer(
        layer: String,
        from: KeyMapping,
        to: KeyMapping,
        target_layer: Option<String>,
    ) -> Self {
        Manipulator {
            conditions: Some(vec![Condition::active(layer.clone())]),
            from,
            to: Some(vec![ManipulationTarget::KeyMapping(to)]),
            to_delayed_action: set_target_layer(target_layer, layer),
            to_after_key_up: None,
            to_if_alone: None,
            manipulator_type: "basic".to_string(),
        }
    }

    pub fn set_command_in_layer(
        layer: String,
        from: KeyMapping,
        to: ShellCommand,
        target_layer: Option<String>,
    ) -> Self {
        Manipulator {
            conditions: Some(vec![Condition::active(layer.clone())]),
            from,
            to: Some(vec![ManipulationTarget::ShellCommand(to)]),
            to_delayed_action: set_target_layer(target_layer, layer),
            to_after_key_up: None,
            to_if_alone: None,
            manipulator_type: "basic".to_string(),
        }
    }
    fn switch_layer(target_layer: String, source_layer: String, from: KeyMapping) -> Manipulator {
        Manipulator {
            conditions: Some(vec![]),
            from,
            to: Some(vec![
                ManipulationTarget::set_active(target_layer),
                ManipulationTarget::set_inactive(source_layer),
            ]),
            to_delayed_action: None,
            to_after_key_up: None,
            to_if_alone: None,
            manipulator_type: "basic".to_string(),
        }
    }
}

fn set_target_layer(target_layer: Option<String>, source_layer: String) -> Option<DelayedAction> {
    match target_layer {
        None => None,
        Some(layer) => Some(DelayedAction::set_layer(layer, source_layer)),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Condition {
    pub name: String,
    #[serde(rename = "type")]
    pub condition_type: String,
    pub value: i32,
}

impl Condition {
    pub fn active(name: String) -> Self {
        Condition {
            name,
            condition_type: "variable_if".to_string(),
            value: 1,
        }
    }
    pub fn inactive(name: String) -> Self {
        Condition {
            name,
            condition_type: "variable_if".to_string(),
            value: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SimpleKeyMapping {
    pub key_code: Key,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Modifiers {
    pub mandatory: Option<Vec<Key>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<Vec<Key>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct KeyMapping {
    pub key_code: Key,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Modifiers>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SetVariableValues {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SetVariable {
    set_variable: SetVariableValues,
}
impl SetVariable {
    pub fn new(name: String, value: i32) -> Self {
        SetVariable {
            set_variable: SetVariableValues { name, value },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ShellCommand {
    pub shell_command: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ManipulationTarget {
    KeyMapping(KeyMapping),
    SetVariable(SetVariable),
    ShellCommand(ShellCommand),
}

impl ManipulationTarget {
    pub fn set_active(name: String) -> Self {
        ManipulationTarget::SetVariable(SetVariable::new(name, 1))
    }
    pub fn set_inactive(name: String) -> Self {
        ManipulationTarget::SetVariable(SetVariable::new(name, 0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DelayedAction {
    pub to_if_canceled: Vec<SetVariable>,
    pub to_if_invoked: Vec<SetVariable>,
}

impl DelayedAction {
    fn set_layer(target_layer: String, source_layer: String) -> Self {
        let mut actions = vec![SetVariable::new(source_layer, 0)];

        if target_layer != BASE_LAYER {
            actions.insert(0, SetVariable::new(target_layer, 1));
        }

        DelayedAction {
            to_if_canceled: vec![],
            to_if_invoked: actions,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Device {
    pub identifiers: DeviceIdentifiers,
    pub simple_modifications: Vec<SimpleModification>,
}

impl Default for DeviceIdentifiers {
    fn default() -> Self {
        DeviceIdentifiers {
            is_keyboard: true,
            product_id: 0,
            vendor_id: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DeviceIdentifiers {
    pub is_keyboard: bool,
    pub product_id: u16,
    pub vendor_id: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SimpleModification {
    pub from: SimpleKeyMapping,
    pub to: Vec<SimpleKeyMapping>,
}
