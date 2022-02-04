use crate::command::{Command, CommandRun, CommandRunArgs};
use clap::Parser;
use err_context::AnyError;
use serde_json::json;
use serenity::async_trait;
use serenity::http::LightMethod::Post;
use shared::CommandError;

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
        {
            let r = a.bot.read().await;

            r.check_health().await?;

            let url = r.latest_image.get(&a.msg.channel_id);

            let mut img_url: String = String::from("<void>");
            match url {
                Some(url) => {
                    img_url = url.clone();
                }
                None => {
                    let url = a.matches.value_of("url");
                    if url.is_none() {
                        return Err(CommandError::GenericError("No url provided. Try sending a new image, or specify a url with -u.").into());
                    }
                    img_url = url.unwrap().to_string();
                }
            }

            let request = json!({
                "target_url": img_url,
                "text": a.matches.value_of("text").unwrap_or("No caption provided! :(")
            });

            let request = r.send_post("/caption").await
                .body(request.to_string())
                .build()?;

            let response = r.client.execute(request).await?;
        };

        Ok(())
    }
}

#[derive(Parser, Debug)]
#[clap(
    author = "rwilliaise (lego man)",
    about = "Caption an image with specified text",
    version = "v0.1.0"
)]
struct CaptionArgs {
    #[clap(short, long)]
    url: Option<String>,
    text: String,
}

pub fn caption() -> Command {
    Command::builder("caption")
        .run(CaptionsRun)
        .parser::<CaptionArgs>()
        .build()
}
