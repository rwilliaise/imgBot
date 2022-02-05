use crate::command::{Command, CommandRun, CommandRunArgs};
use crate::process::basic_img_job;
use std::borrow::{Borrow, Cow};

use clap::Parser;
use err_context::AnyError;
use serenity::async_trait;
use serenity::http::AttachmentType;
use shared::CommandError;

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
        {
            let r = a.bot.read().await;
            let response = basic_img_job(&r, &a, "/caption").await;

            if let Err(e) = response {
                return Err(CommandError::SourcedError(
                    "Failed to queue caption request. Report this to a dev!\n\n",
                    e
                )
                    .into());
            }

            let response = response?;

            match response.error_for_status_ref() {
                Ok(_) => (),
                Err(e) => {
                    let text = response.text().await?;
                    return Err(CommandError::StringError(
                        format!("Image server contact failure.\n\n{}", text.as_str())
                    )
                    .into());
                }
            }
            let bytes = response.bytes().await?;


            a.http.send_files(
                a.msg.channel_id.clone().into(),
                [ AttachmentType::Bytes {
                    data: Cow::Borrowed(bytes.borrow()),
                    filename: "caption.png".to_string(), // TODO: support more content types
                } ],
                serde_json::Map::default(),
            ).await;
        };

        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(
    author = "rwilliaise (lego man)",
    version = "v0.1.0"
)]
/// Caption an image, with specified text.
///
/// Font included: Futura Condensed Extra Bold
///
/// Supported content: image/png
struct CaptionArgs {
    #[clap(short, long)]
    /// URL pointing to image to caption. If this is not supplied, imgBot will
    /// automatically pull the latest image from chat.
    url: Option<String>,

    #[clap(short, long)]
    /// Print extra information with some error messages.
    verbose: bool,

    /// Text to caption the image with.
    text: String,
}

pub fn caption() -> Command {
    Command::builder("caption")
        .run(CaptionsRun)
        .parser::<CaptionArgs>()
        .build()
}
