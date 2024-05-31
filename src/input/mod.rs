use slint::SharedString;
use strum::{AsRefStr, Display};

#[derive(Debug, Copy, Clone, Display, AsRefStr, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    X,
    Y,
    Start,
    Select,
    L,
    R,
    L2,
    R2,
    Menu,
    Power,
    VolumeDown,
    VolumeUp,
    Unknown,
}

impl From<evdev::Key> for Key {
    fn from(value: evdev::Key) -> Self {
        match value {
            evdev::Key::KEY_UP => Key::Up,
            evdev::Key::KEY_DOWN => Key::Down,
            evdev::Key::KEY_LEFT => Key::Left,
            evdev::Key::KEY_RIGHT => Key::Right,
            evdev::Key::KEY_SPACE => Key::A,
            evdev::Key::KEY_LEFTCTRL => Key::B,
            evdev::Key::KEY_LEFTSHIFT => Key::X,
            evdev::Key::KEY_LEFTALT => Key::Y,
            evdev::Key::KEY_ENTER => Key::Start,
            evdev::Key::KEY_RIGHTCTRL => Key::Select,
            evdev::Key::KEY_E => Key::L,
            evdev::Key::KEY_T => Key::R,
            evdev::Key::KEY_TAB => Key::L2,
            evdev::Key::KEY_BACKSPACE => Key::R2,
            evdev::Key::KEY_ESC => Key::Menu,
            evdev::Key::KEY_POWER => Key::Power,
            evdev::Key::KEY_VOLUMEDOWN => Key::VolumeDown,
            evdev::Key::KEY_VOLUMEUP => Key::VolumeUp,
            _ => Key::Unknown,
        }
    }
}

impl From<Key> for Option<SharedString> {
    fn from(key: Key) -> Option<SharedString> {
        if key == Key::Unknown {
            None
        } else {
            Some(key.to_string().into())
        }
    }
}

#[cfg(test)]
mod tests {
    // use slint::{platform::Key as SlintKey, SharedString};
    // use std::convert::Into;

    use super::Key;

    #[test]
    fn test_display() {
        // assert_eq!(
        //     Key::Up.as_ref(),
        //     Into::<SharedString>::into(SlintKey::UpArrow).as_str(),
        // );
        // assert_eq!(
        //     Key::Down.as_ref(),
        //     Into::<SharedString>::into(SlintKey::DownArrow).as_str(),
        // );
        // assert_eq!(
        //     Key::Left.as_ref(),
        //     Into::<SharedString>::into(SlintKey::LeftArrow).as_str(),
        // );
        // assert_eq!(
        //     Key::Right.as_ref(),
        //     Into::<SharedString>::into(SlintKey::RightArrow).as_str(),
        // );
        assert_eq!(Key::A.to_string(), "a");
        assert_eq!(Key::B.to_string(), "b");
        assert_eq!(Key::X.to_string(), "x");
        assert_eq!(Key::Y.to_string(), "y");
    }
}
