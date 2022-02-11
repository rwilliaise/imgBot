use crate::command::{Command, CommandRun, CommandRunArgs};
use crate::process::img_job;
use clap::{AppSettings, Parser};
use err_context::AnyError;
use serenity::async_trait;

struct SeveredRun;

#[async_trait]
impl CommandRun for SeveredRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
        img_job(a, "/severed", true).await
    }
}

#[derive(Parser, Debug)]
#[clap(author = "rwilliaise (lego man)", version = "v0.1.0")]
#[clap(setting(AppSettings::TrailingVarArg))]
/// Exploitable image macro.
///
/// Image from the iconic "Divine Light Severed" death screen from Cruelty Squad.
///
/// Font included: MingLiu
struct SeveredArgs {
    #[clap(short, long)]
    /// Print extra information with some error messages.
    verbose: bool,

    /// Text to caption the image with.
    text: Vec<String>,
}

pub fn severed() -> Command {
    Command::builder("severed")
        .run(SeveredRun)
        .parser::<SeveredArgs>()
        .build()
}
