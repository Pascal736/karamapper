use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, strum::EnumString, Clone, derive_more::Display, PartialEq, Eq,
)]
#[strum(serialize_all = "snake_case")]
pub enum Key {
    Hyper,
    V,
    H,
    J,
    A,
    Y,
    N,
    Ctrl,
    Shift,
    DownArrow,
    LeftArrow,
    CapsLock,
    Esc,
}

impl Key {
    pub fn to_string(&self) -> String {
        match self {
            Key::Hyper => "hyper".to_string(),
            Key::V => "v".to_string(),
            Key::H => "h".to_string(),
            Key::J => "j".to_string(),
            Key::A => "a".to_string(),
            Key::Y => "y".to_string(),
            Key::N => "n".to_string(),
            Key::Ctrl => "ctrl".to_string(),
            Key::Shift => "shift".to_string(),
            Key::DownArrow => "down_arrow".to_string(),
            Key::LeftArrow => "left_arrow".to_string(),
            Key::CapsLock => "caps_lock".to_string(),
            Key::Esc => "escape".to_string(),
        }
    }
}
