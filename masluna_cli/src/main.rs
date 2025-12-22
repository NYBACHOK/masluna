use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    JS(JsArgs),
    Run(RunArgs),
}

#[derive(Debug, clap::Args)]
struct JsArgs {
    #[command(subcommand)]
    command: JsSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum JsSubcommand {
    Compile,
    Plugin,
}

#[derive(Debug, clap::Args)]
struct RunArgs {
    /// Path to wasm file
    #[arg(required = true)]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { command } = Args::parse();

    match command {
        Command::JS(JsArgs { command }) => match command {
            JsSubcommand::Compile => todo!(),
            JsSubcommand::Plugin => todo!(),
        },
        Command::Run(RunArgs { path }) => {
            smol::block_on(async move {
                let mut module = masluna_core::WasmModule::from_file(path).await?;

                let _engine = module.build_instance(None)?;

                Result::<_, Box<dyn std::error::Error>>::Ok(())
            })?;
        }
    }

    Ok(())
}
