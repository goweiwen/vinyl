use std::cell::Cell;
use std::sync::mpsc::channel;
use std::thread;
use std::{cell::RefCell, rc::Rc};

use framebuffer::Framebuffer;
use log::debug;
use slint::platform::software_renderer::{PremultipliedRgbaColor, TargetPixel};
use slint::{
    platform::{
        software_renderer::{MinimalSoftwareWindow, RenderingRotation, RepaintBufferType},
        Platform, WindowAdapter,
    },
    PhysicalSize,
};

use crate::miyoo::evdev::Evdev;

pub struct MyPlatform {
    evdev: Cell<Option<Evdev>>,
    framebuffer: RefCell<Framebuffer>,
    buffer_size: usize,
    buffer_offset: usize,
    window: Rc<MinimalSoftwareWindow>,
}

impl MyPlatform {
    pub fn new() -> Self {
        let framebuffer = Framebuffer::new("/dev/fb0").expect("Failed to open /dev/fb0");
        debug!(
            "init fb: var_screen_info: {:?}, fix_screen_info: {:?}",
            framebuffer.var_screen_info, framebuffer.fix_screen_info,
        );

        let (width, height, xoffset, yoffset) = (
            framebuffer.var_screen_info.xres as usize,
            framebuffer.var_screen_info.yres as usize,
            framebuffer.var_screen_info.xoffset as usize,
            framebuffer.var_screen_info.yoffset as usize,
        );
        let bytes_per_pixel = framebuffer.var_screen_info.bits_per_pixel / 8;
        let buffer_size = width * height * bytes_per_pixel as usize;
        let buffer_offset = (yoffset * width + xoffset) * bytes_per_pixel as usize;

        let window = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
        window.request_redraw();
        window.draw_if_needed(|renderer| {
            renderer.set_rendering_rotation(RenderingRotation::Rotate180)
        });
        window.set_size(PhysicalSize::new(
            framebuffer.var_screen_info.xres,
            framebuffer.var_screen_info.yres,
        ));
        let framebuffer = RefCell::new(framebuffer);
        let evdev = Cell::new(Some(Evdev::new()));

        Self {
            evdev,
            framebuffer,
            buffer_size,
            buffer_offset,
            window,
        }
    }
}

impl Platform for MyPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        let mut evdev = self.evdev.take().unwrap();
        let (input_tx, input_rx) = channel();
        thread::spawn(move || loop {
            if let Some(event) = evdev.fetch_events() {
                let _ = input_tx.send(event);
            }
        });

        let mut framebuffer = self.framebuffer.borrow_mut();
        let mut frame: Vec<RGBX8> = vec![
            RGBX8::default();
            self.window.size().width as usize
                * self.window.size().height as usize
        ];
        loop {
            // Let Slint run the timer hooks and update animations.
            slint::platform::update_timers_and_animations();

            while let Ok(event) = input_rx.try_recv() {
                debug!("input event: {:?}", &event);
                self.window.dispatch_event(event);
            }

            // Draw the scene if something needs to be drawn.
            self.window.draw_if_needed(|renderer| {
                renderer.render(&mut frame, self.window.size().width as usize);
                framebuffer.frame[self.buffer_offset..self.buffer_offset + self.buffer_size]
                    .copy_from_slice(&frame.as_bytes());
            });
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct RGBX8([u8; 4]);

impl TargetPixel for RGBX8 {
    fn blend(&mut self, color: PremultipliedRgbaColor) {
        let a = (u8::MAX - color.alpha) as u16;
        self.0[0] = (self.0[0] as u16 * a / 255) as u8 + color.blue;
        self.0[1] = (self.0[1] as u16 * a / 255) as u8 + color.green;
        self.0[2] = (self.0[2] as u16 * a / 255) as u8 + color.red;
    }

    fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([b, g, r, 255])
    }
}

trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for [RGBX8] {
    /// The components interpreted as raw bytes, in machine's native endian. In `RGB` bytes of the red component are first.
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.as_ptr() as *const _,
                self.len() * core::mem::size_of::<RGBX8>(),
            )
        }
    }
}
