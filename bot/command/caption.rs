use crate::command::{Command, CommandRun, CommandRunArgs};
use crate::process::img_job;

use clap::{AppSettings, Parser};
use err_context::AnyError;
use serenity::async_trait;

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
        img_job(a, "/caption", false).await
    }
}

#[derive(Parser, Debug)]
#[clap(author = "rwilliaise (lego man)", version = "v0.1.1")]
#[clap(setting(AppSettings::TrailingVarArg))]
/// Caption an image, with specified text.
///
/// Font included: Futura Condensed Extra Bold
///
/// All image formats listed here are supported:
/// https://github.com/image-rs/image#supported-image-formats
///
/// However, only GIF animations will be decoded as animations and re-encoded as animations.
struct CaptionArgs {
    #[clap(short, long)]
    /// URL pointing to image to caption. If this is not supplied, imgBot will
    /// automatically pull the latest image from chat.
    url: Option<String>,

    #[clap(short, long)]
    /// Print extra information with some error messages.
    verbose: bool,

    /// Text to caption the image with.
    text: Vec<String>,
}

pub fn caption() -> Command {
    Command::builder("caption")
        .run(CaptionsRun)
        .parser::<CaptionArgs>()
        .build()
}
