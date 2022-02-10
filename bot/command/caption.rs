use crate::command::{Command, CommandRun, CommandRunArgs};
use crate::process::basic_img_job;
use std::borrow::{Borrow, Cow};
use std::time::Duration;

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
            let mut msg = a
                .msg
                .channel_id
                .say(a.http.clone(), "1/3 🟩⬛⬛ Requesting")
                .await?;

            let response = basic_img_job(&a, "/caption").await;

            msg.edit(a.http.clone(), |m| m.content("2/3 🟩🟩⬛ Processing"))
                .await?;

            if let Err(e) = response {
                crate::process::delay_delete(a.http.clone(), msg, Duration::from_millis(1000)).await;
                return Err(CommandError::SourcedError("Failed caption request.\n\n", e).into());
            }

            let response = response?;

            match response.error_for_status_ref() {
                Ok(_) => (),
                Err(_) => {
                    crate::process::delay_delete(a.http.clone(), msg, Duration::from_millis(1000)).await;
                    let text = response.text().await?;
                    return Err(CommandError::StringError(format!(
                        "Image server contact failure.\n\n{}",
                        text.as_str()
                    ))
                    .into());
                }
            }
            let mime = response
                .headers()
                .get(CONTENT_TYPE)
                .ok_or(CommandError::GenericError("Unknown format"))?;
            let is_gif = mime == "image/gif";

            let bytes = response.bytes().await?;

            msg.edit(a.http.clone(), |m| m.content("3/3 🟩🟩🟩 Uploading"))
                .await?;

            a.http
                .send_files(
                    a.msg.channel_id.clone().into(),
                    [AttachmentType::Bytes {
                        data: Cow::Borrowed(bytes.borrow()),
                        filename: match is_gif {
                            // TODO: support more content types
                            false => "caption.png",
                            true => "caption.gif",
                        }
                        .to_string(),
                    }],
                    serde_json::Map::default(),
                )
                .await?;


            crate::process::delay_delete(a.http.clone(), msg, Duration::from_millis(1000)).await;
        };

        Ok(())
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
