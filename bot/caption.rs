use crate::command::{Command, CommandRun, CommandRunArgs};
use clap::Parser;
use err_context::AnyError;
use serenity::async_trait;

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), &'static str> {
        {
            let r = a.bot.read().await;
            let url = r.latest_image.get(&a.msg.channel_id);

            let mut img_url: String = String::from("<void>");
            match url {
                Some(url) => {
                    img_url = url.clone();
                }
                None => {
                    let url = a.matches.value_of("url");
                    if url.is_none() {
                        return Err("No url found. Send a new image to caption, or use -u!");
                    }
                    img_url = url.unwrap().to_string();
                }
            }
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
