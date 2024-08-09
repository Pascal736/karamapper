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
