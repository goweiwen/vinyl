use core::time::Duration;
use evdev::{Device, EventType};
use slint::platform::WindowEvent;

const MAXIMUM_FRAME_TIME: Duration = Duration::from_millis(100);
use crate::input::Key;

pub struct Evdev {
    pub device: Device,
}

impl Evdev {
    pub fn new() -> Self {
        Self {
            device: Device::open("/dev/input/event0").expect("Failed to open /dev/input/event0"),
        }
    }

    pub fn fetch_events(&mut self) -> Option<WindowEvent> {
        for event in self.device.fetch_events().unwrap() {
            match event.event_type() {
                EventType::KEY => {
                    if event.timestamp().elapsed().unwrap() > MAXIMUM_FRAME_TIME {
                        continue;
                    }
                    let key = Key::from(evdev::Key(event.code()));
                    let text = key.into();
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
