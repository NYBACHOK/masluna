use crate::*;

use masluna_ui::commands::ApplicationCommands;

use tokio::sync::mpsc::UnboundedSender;

pub fn prepare_handlers(ui: App, sr: UnboundedSender<ApplicationCommands>) {
    ui.global::<AppLogic>().on_file_pick({
        let sr = sr.clone();

        move || {
            let _ = sr.send(ApplicationCommands::OpenWasmFile);
        }
    });
}
