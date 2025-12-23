use crate::*;

use masluna_ui::commands::{ApplicationCommands, ApplicationResponse};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub fn prepare_handlers(
    ui: App,
    sr: UnboundedSender<ApplicationCommands>,
    mut rr: UnboundedReceiver<ApplicationResponse>,
) {
    ui.global::<AppLogic>().on_file_pick({
        let sr = sr.clone();

        move || {
            let _ = sr.send(ApplicationCommands::OpenWasmFile);
        }
    });

    let _handler = slint::spawn_local(async move {
        while let Some(response) = rr.recv().await {
           
        }
    })
    .expect("Critical error. Failed to run internal apps command loop");
}
