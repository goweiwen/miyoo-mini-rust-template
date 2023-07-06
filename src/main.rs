#![warn(clippy::all, clippy::pedantic, rust_2018_idioms)]

mod app;
mod input;
mod platform;

use log::info;
use simple_logger::SimpleLogger;

use crate::app::App;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    info!("Hello, world!");

    let mut app = App::new();
    app.run_event_loop().await?;

    info!("Goodbye!");

    Ok(())
}
