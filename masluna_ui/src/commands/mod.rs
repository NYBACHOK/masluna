use tokio::sync::mpsc::UnboundedSender;

use crate::{TOKIO_RUNTIME, commands::file_picker::select_file};

mod file_picker;

#[derive(Debug, Clone)]
pub enum ApplicationCommands {
    OpenWasmFile,
}

pub fn start_commands_loop() -> UnboundedSender<ApplicationCommands> {
    let (sr, mut rc) = tokio::sync::mpsc::unbounded_channel::<ApplicationCommands>();

    std::thread::spawn(move || {
        TOKIO_RUNTIME.block_on(async move {
            while let Some(cmd) = rc.recv().await {
                tracing::debug!("Received command: {cmd:#?}");

                match cmd {
                    ApplicationCommands::OpenWasmFile => select_file().await,
                }
            }
        });
    });

    sr
}
