use core::time::Duration;
use evdev::{Device, EventType};
use slint::{platform::WindowEvent, SharedString};

const MAXIMUM_FRAME_TIME: Duration = Duration::from_millis(100);

fn map_key(value: evdev::Key) -> Option<SharedString> {
    Some(
        match value {
            evdev::Key::KEY_UP => "up",
            evdev::Key::KEY_DOWN => "down",
            evdev::Key::KEY_LEFT => "left",
            evdev::Key::KEY_RIGHT => "right",
            evdev::Key::KEY_SPACE => "a",
            evdev::Key::KEY_LEFTCTRL => "b",
            evdev::Key::KEY_LEFTSHIFT => "x",
            evdev::Key::KEY_LEFTALT => "y",
            evdev::Key::KEY_ENTER => "start",
            evdev::Key::KEY_RIGHTCTRL => "select",
            evdev::Key::KEY_E => "l",
            evdev::Key::KEY_T => "r",
            evdev::Key::KEY_ESC => "menu",
            evdev::Key::KEY_TAB => "l2",
            evdev::Key::KEY_BACKSPACE => "r2",
            evdev::Key::KEY_POWER => "power",
            evdev::Key::KEY_VOLUMEDOWN => "vol_down",
            evdev::Key::KEY_VOLUMEUP => "vol_up",
            _ => return None,
        }
        .into(),
    )
}

pub struct EvdevKeys {
    pub device: Device,
}

impl EvdevKeys {
    pub fn new() -> Self {
        Self {
            device: Device::open("/dev/input/event0").expect("Failed to open /dev/input/event0"),
        }
    }

    pub fn poll(&mut self) -> Option<WindowEvent> {
        for event in self.device.fetch_events().unwrap() {
            match event.event_type() {
                EventType::KEY => {
                    if event.timestamp().elapsed().unwrap() > MAXIMUM_FRAME_TIME {
                        continue;
                    }
                    let key = evdev::Key(event.code());
                    let text = map_key(key);
                    if let Some(text) = text {
                        return Some(match event.value() {
                            0 => WindowEvent::KeyReleased { text },
                            1 => WindowEvent::KeyPressed { text },
                            2 => WindowEvent::KeyPressRepeated { text },
                            _ => unreachable!(),
                        });
                    }
                }
                _ => {}
            }
        }
        return None;
    }
}
