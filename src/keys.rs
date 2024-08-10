use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Key {
    // Sticky Modifier Keys
    LeftControl,
    LeftShift,
    LeftOption,
    LeftCommand,
    RightControl,
    RightShift,
    RightOption,
    RightCommand,
    Fn,

    // Modifier Keys
    CapsLock,

    // Control or Symbol Keys
    ReturnOrEnter,
    Escape,
    DeleteOrBackspace,
    DeleteForward,
    Tab,
    Spacebar,
    Hyphen,
    EqualSign,
    OpenBracket,
    CloseBracket,
    Backslash,
    NonUsPound,
    Semicolon,
    Quote,
    GraveAccentAndTilde,
    Comma,
    Period,
    Slash,
    NonUsBackslash,

    // Arrow Keys
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    PageUp,
    PageDown,
    Home,
    End,

    // Letter Keys
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Number Keys
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,

    // Function Keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,

    // Keypad Keys
    KeypadNumLock,
    KeypadSlash,
    KeypadAsterisk,
    KeypadHyphen,
    KeypadPlus,
    KeypadEnter,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    Keypad0,
    KeypadPeriod,
    KeypadEqualSign,
    KeypadComma,

    // PC Keyboard Keys
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Application,
    Help,
    Power,

    // International Keys
    International1,
    International2,
    International3,
    International4,
    International5,
    International6,
    International7,
    International8,
    International9,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,

    // Japanese Keys
    JapaneseEisuu,
    JapaneseKana,
    JapanesePcNfer,
    JapanesePcXfer,
    JapanesePcKatakana,

    // Other Keys
    VolumeDown,
    VolumeUp,
    Mute,
    VolumeDecrement,
    VolumeIncrement,

    // From Only Keys
    F21,
    F22,
    F23,
    F24,
    Execute,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    KeypadEqualSignAs400,
    LockingCapsLock,
    LockingNumLock,
    LockingScrollLock,
    AlternateErase,
    SysReqOrAttention,
    Cancel,
    Clear,
    Prior,
    Return,
    Separator,
    Out,
    Oper,
    ClearOrAgain,
    CrSelOrProps,
    ExSel,

    // To Only Keys
    VkNone,
    VkConsumerBrightnessDown,
    VkConsumerBrightnessUp,
    VkMissionControl,
    VkLaunchpad,
    VkDashboard,
    VkConsumerIlluminationDown,
    VkConsumerIlluminationUp,
    VkConsumerPrevious,
    VkConsumerPlay,
    VkConsumerNext,
    DisplayBrightnessDecrement,
    DisplayBrightnessIncrement,
    Rewind,
    PlayOrPause,
    Fastforward,
    AppleDisplayBrightnessDecrement,
    AppleDisplayBrightnessIncrement,
    AppleTopCaseDisplayBrightnessDecrement,
    AppleTopCaseDisplayBrightnessIncrement,
    IlluminationDecrement,
    IlluminationIncrement,
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Key::A
            | Key::B
            | Key::C
            | Key::D
            | Key::E
            | Key::F
            | Key::G
            | Key::H
            | Key::I
            | Key::J
            | Key::K
            | Key::L
            | Key::M
            | Key::N
            | Key::O
            | Key::P
            | Key::Q
            | Key::R
            | Key::S
            | Key::T
            | Key::U
            | Key::V
            | Key::W
            | Key::X
            | Key::Y
            | Key::Z => {
                let lowercase = format!("{}", self).to_lowercase();
                serializer.serialize_str(&lowercase)
            }
            _ => {
                let snake_case = format!("{}", self);
                serializer.serialize_str(&snake_case)
            }
        }
    }
}
