use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Profiles {
    profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    complex_modifications: ComplexModifications,
    devices: Vec<Device>,
    name: String,
    selected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct ComplexModifications {
    rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rule {
    description: Option<String>,
    enabled: bool,
    manipulators: Vec<Manipulator>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Manipulator {
    conditions: Option<Vec<Condition>>,
    from: KeyMapping,
    to: Option<Vec<KeyMappingOrSetVariable>>,
    to_delayed_action: Option<DelayedAction>,
    to_after_key_up: Option<Vec<KeyMappingOrSetVariable>>,
    to_if_alone: Option<Vec<KeyMapping>>,
    #[serde(rename = "type")]
    manipulator_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Condition {
    name: String,
    #[serde(rename = "type")]
    condition_type: String,
    value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyMapping {
    key_code: String,
    modifiers: Option<Modifiers>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Modifiers {
    mandatory: Option<Vec<String>>,
    optional: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum KeyMappingOrSetVariable {
    KeyMapping(KeyMapping),
    SetVariable(SetVariable),
}

#[derive(Debug, Serialize, Deserialize)]
struct SetVariable {
    name: String,
    value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DelayedAction {
    to_if_canceled: Vec<SetVariable>,
    to_if_invoked: Vec<SetVariable>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    identifiers: DeviceIdentifiers,
    simple_modifications: Vec<SimpleModification>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceIdentifiers {
    is_keyboard: bool,
    product_id: u16,
    vendor_id: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct SimpleModification {
    from: KeyMapping,
    to: Vec<KeyMapping>,
}
