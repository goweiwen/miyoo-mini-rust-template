use std::{process, time::Duration};

use embedded_graphics::{pixelcolor::Rgb888, prelude::Size};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

use crate::input::{Key, KeyEvent};

type Display = SimulatorDisplay<Rgb888>;

pub struct Platform {
    window: Window,
    pub display: Display,
}

const DISPLAY_WIDTH: u32 = 640;
const DISPLAY_HEIGHT: u32 = 480;

impl Platform {
    pub fn new() -> Self {
        let output_settings = OutputSettingsBuilder::new().build();
        let window = Window::new("Simulator", &output_settings);
        let display = Display::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));

        Self { window, display }
    }

    pub fn flush(&mut self) {
        self.window.show_static(&self.display)
    }

    pub async fn poll(&mut self) -> KeyEvent {
        loop {
            let event = self.window.events().next();
            if let Some(event) = event {
                match event {
                    SimulatorEvent::KeyDown {
                        keycode, repeat, ..
                    } => {
                        if keycode == Keycode::Q {
                            process::exit(0);
                        }
                        return if repeat {
                            KeyEvent::Autorepeat(Key::from(keycode))
                        } else {
                            KeyEvent::Pressed(Key::from(keycode))
                        };
                    }
                    SimulatorEvent::KeyUp { keycode, .. } => {
                        return KeyEvent::Released(Key::from(keycode))
                    }
                    SimulatorEvent::Quit => {
                        process::exit(0);
                    }
                    _ => {}
                }
            } else {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    }
}

impl From<Keycode> for Key {
    fn from(value: Keycode) -> Self {
        match value {
            Keycode::Up => Key::Up,
            Keycode::Down => Key::Down,
            Keycode::Left => Key::Left,
            Keycode::Right => Key::Right,
            Keycode::Space => Key::A,
            Keycode::LCtrl => Key::B,
            Keycode::LShift => Key::X,
            Keycode::LAlt => Key::Y,
            Keycode::Return => Key::Start,
            Keycode::RCtrl => Key::Select,
            Keycode::E => Key::L,
            Keycode::T => Key::R,
            Keycode::Escape => Key::Menu,
            Keycode::Tab => Key::L2,
            Keycode::Backspace => Key::R2,
            Keycode::Power => Key::Power,
            Keycode::LGui => Key::VolDown,
            Keycode::RGui => Key::VolUp,
            _ => Key::Unknown,
        }
    }
}
