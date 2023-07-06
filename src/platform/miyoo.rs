use std::time::Duration;

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use evdev::{Device, EventStream, EventType};
use framebuffer::Framebuffer;

use crate::input::{Key, KeyEvent};

const MAXIMUM_FRAME_TIME: Duration = Duration::from_millis(100);

type Display = FramebufferDisplay;

pub struct Platform {
    pub display: Display,
    events: EventStream,
}

impl Platform {
    pub fn new() -> Self {
        let display = Display::new().unwrap();
        let events = Device::open("/dev/input/event0")
            .unwrap()
            .into_event_stream()
            .unwrap();

        Self { display, events }
    }

    pub fn flush(&mut self) {
        self.display.flush().unwrap();
    }

    pub async fn poll(&mut self) -> KeyEvent {
        loop {
            let event = self.events.next_event().await.unwrap();
            match event.event_type() {
                EventType::KEY => {
                    let key = event.code();
                    let key: Key = evdev::Key(key).into();
                    return match event.value() {
                        0 => KeyEvent::Released(key),
                        1 => KeyEvent::Pressed(key),
                        2 => {
                            if event.timestamp().elapsed().unwrap() > MAXIMUM_FRAME_TIME {
                                continue;
                            }
                            KeyEvent::Autorepeat(key)
                        }
                        _ => unreachable!(),
                    };
                }
                _ => {}
            }
        }
    }
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
            evdev::Key::KEY_ESC => Key::Menu,
            evdev::Key::KEY_TAB => Key::L2,
            evdev::Key::KEY_BACKSPACE => Key::R2,
            evdev::Key::KEY_POWER => Key::Power,
            evdev::Key::KEY_VOLUMEDOWN => Key::VolDown,
            evdev::Key::KEY_VOLUMEUP => Key::VolUp,
            _ => Key::Unknown,
        }
    }
}

pub struct Buffer {
    buffer: Vec<u8>,
    size: Size,
    bytes_per_pixel: u32,
}

pub struct FramebufferDisplay {
    framebuffer: Buffer,
    iface: Framebuffer,
}

impl FramebufferDisplay {
    pub fn new() -> anyhow::Result<FramebufferDisplay> {
        let iface = Framebuffer::new("/dev/fb0")?;

        let background = iface.read_frame();
        let size = Size::new(iface.var_screen_info.xres, iface.var_screen_info.yres);

        let width = size.width as usize;
        let height = size.height as usize;
        let bytes_per_pixel = iface.var_screen_info.bits_per_pixel / 8;
        let mut buffer = vec![0; width * height * bytes_per_pixel as usize];

        let (xoffset, yoffset) = (
            iface.var_screen_info.xoffset as usize,
            iface.var_screen_info.yoffset as usize,
        );
        let location = (yoffset * width + xoffset) * bytes_per_pixel as usize;

        let buffer_size = buffer.len();
        buffer[..].copy_from_slice(&background[location..location + buffer_size]);

        Ok(FramebufferDisplay {
            framebuffer: Buffer {
                buffer,
                size,
                bytes_per_pixel,
            },
            iface,
        })
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        let (xoffset, yoffset) = (
            self.iface.var_screen_info.xoffset as usize,
            self.iface.var_screen_info.yoffset as usize,
        );
        let width = self.framebuffer.size.width as usize;
        let location = (yoffset * width + xoffset) * self.framebuffer.bytes_per_pixel as usize;
        self.iface.frame[location..location + self.framebuffer.buffer.len()]
            .copy_from_slice(&self.framebuffer.buffer);
        Ok(())
    }
}

impl DrawTarget for FramebufferDisplay {
    type Color = Rgb888;
    type Error = anyhow::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let width = self.framebuffer.size.width as i32;
        let height = self.framebuffer.size.height as i32;
        let bytespp = self.framebuffer.bytes_per_pixel;

        for Pixel(coord, color) in pixels.into_iter() {
            // rotate 180 degrees
            let x: i32 = width - coord.x - 1;
            let y: i32 = height - coord.y - 1;
            if 0 <= x && x < width && 0 <= y && y < height {
                let index: u32 = (x as u32 + y as u32 * width as u32) * bytespp;
                self.framebuffer.buffer[index as usize] = color.b();
                self.framebuffer.buffer[index as usize + 1] = color.g();
                self.framebuffer.buffer[index as usize + 2] = color.r();
            }
        }

        Ok(())
    }
}

impl OriginDimensions for FramebufferDisplay {
    fn size(&self) -> Size {
        self.framebuffer.size
    }
}
