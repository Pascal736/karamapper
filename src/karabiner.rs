use serde::{Deserialize, Serialize};

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
    pub manipulators: Manipulator,
}

impl Rule {
    pub fn set_environment(name: String, from: KeyMapping) -> Self {
        Rule {
            description: Some(format!("Change to {}", name)),
            enabled: true,
            manipulators: Manipulator::set_environment(name, from),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Manipulator {
    pub conditions: Option<Vec<Condition>>,
    pub from: KeyMapping,
    pub to: Option<Vec<KeyMappingOrSetVariable>>,
    pub to_delayed_action: Option<DelayedAction>,
    pub to_after_key_up: Option<Vec<KeyMappingOrSetVariable>>,
    pub to_if_alone: Option<Vec<KeyMapping>>,
    #[serde(rename = "type")]
    pub manipulator_type: String,
}

impl Manipulator {
    pub fn set_environment(name: String, from: KeyMapping) -> Self {
        Manipulator {
            conditions: Some(vec![Condition::inactive(name.clone())]),
            from,
            to: Some(vec![KeyMappingOrSetVariable::set_active(name)]),
            to_delayed_action: None,
            to_after_key_up: None,
            to_if_alone: None,
            manipulator_type: "basic".to_string(),
        }
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
pub struct KeyMapping {
    pub key_code: String,
    pub modifiers: Option<Modifiers>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SimpleKeyMapping {
    pub key_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Modifiers {
    pub mandatory: Option<Vec<String>>,
    pub optional: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum KeyMappingOrSetVariable {
    KeyMapping(KeyMapping),
    SetVariable(SetVariable),
}

impl KeyMappingOrSetVariable {
    pub fn set_active(name: String) -> Self {
        KeyMappingOrSetVariable::SetVariable(SetVariable { name, value: 1 })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SetVariable {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DelayedAction {
    pub to_if_canceled: Vec<SetVariable>,
    pub to_if_invoked: Vec<SetVariable>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Device {
    pub identifiers: DeviceIdentifiers,
    pub simple_modifications: Vec<SimpleModification>,
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
