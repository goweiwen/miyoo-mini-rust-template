use embedded_graphics::{
    mono_font::{iso_8859_14::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{CornerRadii, PrimitiveStyle, Rectangle, RoundedRectangle},
    text::Text,
};
use log::info;

use crate::input::{Key, KeyEvent};
use crate::platform::Platform;

pub struct AppState {
    margin_top: i32,
    margin_left: i32,
    margin_right: i32,
    margin_bottom: i32,
    grid_interval: i32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            margin_top: 0,
            margin_left: 0,
            margin_right: 0,
            margin_bottom: 0,
            grid_interval: 3,
        }
    }
}

pub struct App {
    platform: Platform,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        let platform = Platform::new();
        let state = AppState::default();

        Self { platform, state }
    }

    pub async fn run_event_loop(&mut self) -> anyhow::Result<()> {
        loop {
            self.draw()?;

            let event = self.platform.poll().await;
            info!("event: {:?}", event);
            match event {
                KeyEvent::Released(Key::Menu) => {
                    break;
                }
                KeyEvent::Pressed(Key::Left) => {
                    self.state.margin_left -= 1;
                }
                KeyEvent::Pressed(Key::Right) => {
                    self.state.margin_right -= 1;
                }
                KeyEvent::Pressed(Key::Up) => {
                    self.state.margin_top -= 1;
                }
                KeyEvent::Pressed(Key::Down) => {
                    self.state.margin_bottom -= 1;
                }
                KeyEvent::Pressed(Key::Y) => {
                    self.state.margin_left += 1;
                }
                KeyEvent::Pressed(Key::A) => {
                    self.state.margin_right += 1;
                }
                KeyEvent::Pressed(Key::X) => {
                    self.state.margin_top += 1;
                }
                KeyEvent::Pressed(Key::B) => {
                    self.state.margin_bottom += 1;
                }
                KeyEvent::Pressed(Key::Select) => {
                    self.state.grid_interval -= 1;
                }
                KeyEvent::Pressed(Key::Start) => {
                    self.state.grid_interval += 1;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        let display = &mut self.platform.display;
        let state = &self.state;

        display.clear(Rgb888::WHITE)?;

        let Point { x, y } = display.bounding_box().top_left;
        let Size {
            width: w,
            height: h,
        } = display.bounding_box().size;

        let x = x + state.margin_left;
        let y = y + state.margin_top;
        let w = w - state.margin_left as u32 - state.margin_right as u32;
        let h = h - state.margin_top as u32 - state.margin_bottom as u32;

        let fill_style = PrimitiveStyle::with_fill(Rgb888::WHITE);
        Rectangle::new(Point::new(x, y), Size::new(w, h))
            .into_styled(fill_style)
            .draw(display)?;

        if state.grid_interval > 0 {
            let fill_style = PrimitiveStyle::with_fill(Rgb888::BLACK);
            for x in (x..x + w as i32).step_by(state.grid_interval as usize) {
                Rectangle::new(Point::new(x, y), Size::new(1, h))
                    .into_styled(fill_style)
                    .draw(display)?;
            }
            for y in (y..y + h as i32).step_by(state.grid_interval as usize) {
                Rectangle::new(Point::new(x, y), Size::new(w, 1))
                    .into_styled(fill_style)
                    .draw(display)?;
            }
        }

        let text_style = MonoTextStyle::new(&FONT_10X20, Rgb888::BLACK);
        let label = format!(
            r"Adjust Margin: UDLR, ABXY
top: {}px
right: {}px
bottom: {}px
left: {}px
Adjust Grid Interval: Start/Select
grid interval: {}px",
            state.margin_top,
            state.margin_right,
            state.margin_bottom,
            state.margin_left,
            state.grid_interval
        );
        let text = Text::with_alignment(
            &label,
            Point::new(x + w as i32 / 2, y + h as i32 / 2 - 70),
            text_style,
            embedded_graphics::text::Alignment::Center,
        );
        RoundedRectangle::new(text.bounding_box(), CornerRadii::new(Size::new_equal(12)))
            .into_styled(fill_style)
            .draw(display)?;
        text.draw(display)?;

        self.platform.flush();

        Ok(())
    }
}
