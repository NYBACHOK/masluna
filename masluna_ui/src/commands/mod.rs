use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::TOKIO_RUNTIME;

#[derive(Debug, Clone)]
pub enum ApplicationCommands {
    OpenWasmFile,
}

#[derive(Debug, Clone)]
pub enum ApplicationResponse {
    WasmFileSelected,
    WasmFileLoaded,
    InvalidWasmFile,
}

pub fn start_commands_loop() -> (
    UnboundedSender<ApplicationCommands>,
    UnboundedReceiver<ApplicationResponse>,
) {
    let (sr, mut rc) = tokio::sync::mpsc::unbounded_channel::<ApplicationCommands>();
    let (mut rsr, rrc) = tokio::sync::mpsc::unbounded_channel::<ApplicationResponse>();

    std::thread::spawn(move || {
        TOKIO_RUNTIME.block_on(async move {
            while let Some(cmd) = rc.recv().await {
                tracing::debug!("Received command: {cmd:#?}");

                match cmd {
                    ApplicationCommands::OpenWasmFile => {
                        let file = rfd::AsyncFileDialog::new()
                            .set_directory(dirs::home_dir().expect("failed to fetch home dir"))
                            .pick_file()
                            .await;

                        let file = match file {
                            Some(file) => file.path().to_path_buf(),
                            None => continue,
                        };
                    }
                }
            }
        });
    });

    (sr, rrc)
}
