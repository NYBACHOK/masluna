// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::ToSharedString;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = App::new()?;

    ui.global::<AppLogic>().on_file_pick({
        let ui = ui.clone_strong();

        move || {
            let _handler = slint::spawn_local({
                let ui = ui.clone_strong();
                async move {
                    let file = rfd::AsyncFileDialog::new()
                        .set_directory(dirs::home_dir().expect("failed to fetch home dir"))
                        .pick_file()
                        .await;

                    if let Some(file) = file {
                        ui.set_file_path(file.path().display().to_shared_string());
                    }
                }
            });
        }
    });

    ui.run()?;

    Ok(())
}
