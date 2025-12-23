// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use crate::ui_logic::prepare_handlers;

mod ui_logic;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();

    let ui = App::new()?;

    let (sr, rr) = masluna_ui::commands::start_commands_loop();

    prepare_handlers(ui.clone_strong(), sr, rr);

    ui.run()?;

    Ok(())
}

fn setup_logger() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("winit=warn".parse().unwrap())
                .add_directive("sctk=warn".parse().unwrap()),
        )
        .init();
}
