use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use log::info;

use crate::input::{Key, KeyEvent};
use crate::platform::Platform;

pub struct App {
    platform: Platform,
}

impl App {
    pub fn new() -> Self {
        let platform = Platform::new();

        Self { platform }
    }

    pub async fn run_event_loop(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;

            let event = self.platform.poll().await;
            info!("event: {:?}", event);
            match event {
                KeyEvent::Pressed(Key::Menu) => {
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        let display = &mut self.platform.display;

        display.clear(Rgb888::BLACK)?;

        let line_style = PrimitiveStyle::with_stroke(Rgb888::new(255, 255, 255), 1);
        let text_style = MonoTextStyle::new(&FONT_6X9, Rgb888::new(255, 255, 255));

        Circle::new(Point::new(72, 8), 48)
            .into_styled(line_style)
            .draw(display)?;

        Line::new(Point::new(48, 16), Point::new(8, 16))
            .into_styled(line_style)
            .draw(display)?;

        Line::new(Point::new(48, 16), Point::new(64, 32))
            .into_styled(line_style)
            .draw(display)?;

        Rectangle::new(Point::new(79, 15), Size::new(34, 34))
            .into_styled(line_style)
            .draw(display)?;

        Text::new("Hello World!", Point::new(5, 5), text_style).draw(display)?;

        self.platform.flush();

        Ok(())
    }
}
