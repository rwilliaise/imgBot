use crate::command::{Command, CommandRun, CommandRunArgs};
use crate::process::basic_img_job;
use std::borrow::{Borrow, Cow};
use std::time::{Duration, Instant};

use clap::{AppSettings, Parser};
use err_context::AnyError;
use reqwest::header::CONTENT_TYPE;
use serenity::async_trait;
use serenity::http::AttachmentType;
use shared::CommandError;

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
        {
            let r = a.bot.read().await;
            let mut msg = a.msg.channel_id.say(a.http.clone(), "1/2 🟩⬛ Requesting").await?;

            let response = basic_img_job(&r, &a, "/caption").await;

            if let Err(e) = response {
                return Err(CommandError::SourcedError(
                    "Failed to queue caption request.\n\n",
                    e,
                )
                .into());
            }

            let response = response?;

            match response.error_for_status_ref() {
                Ok(_) => (),
                Err(_) => {
                    let text = response.text().await?;
                    return Err(CommandError::StringError(format!(
                        "Image server contact failure.\n\n{}",
                        text.as_str()
                    ))
                    .into());
                }
            }
            let mime = response.headers().get(CONTENT_TYPE).ok_or(CommandError::GenericError("Unknown format"))?;
            let is_gif = mime == "image/gif";

            let bytes = response.bytes().await?;

            msg.edit(a.http.clone(), |m| m.content("2/2 🟩🟩 Uploading")).await?;

            a.http
                .send_files(
                    a.msg.channel_id.clone().into(),
                    [AttachmentType::Bytes {
                        data: Cow::Borrowed(bytes.borrow()),
                        filename: match is_gif { // TODO: support more content types
                            false => {
                                "caption.png"
                            }
                            true => {
                                "caption.gif"
                            }
                        }.to_string(),
                    }],
                    serde_json::Map::default(),
                )
                .await;

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                msg.delete(a.http.clone()).await;
            }).await;

        };

        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(author = "rwilliaise (lego man)", version = "v0.1.0")]
#[clap(setting(AppSettings::TrailingVarArg))]
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
    text: Vec<String>,
}

pub fn caption() -> Command {
    Command::builder("caption")
        .run(CaptionsRun)
        .parser::<CaptionArgs>()
        .build()
}
