use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, strum::EnumString, Clone, derive_more::Display, PartialEq, Eq,
)]
#[strum(serialize_all = "snake_case")]
pub enum Key {
    Hyper,
    V,
    CapsLock,
    Esc,
}

impl Key {
    pub fn to_string(&self) -> String {
        match self {
            Key::Hyper => "hyper".to_string(),
            Key::V => "v".to_string(),
            Key::CapsLock => "caps_lock".to_string(),
            Key::Esc => "escape".to_string(),
        }
    }
}
