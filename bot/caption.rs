use serenity::async_trait;
use crate::command::{Command, CommandRun, CommandRunArgs};

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) {
        if a.matches.opt_present("h") {
            a.msg.channel_id.say(a.http, format!("```{}```", a.opts.usage("Usage: caption [options] TEXT"))).await;
            return;
        }

        {
            let r = a.bot.read().await;
            let url = r.latest_image.get(&a.msg.channel_id);
            dbg!(url);
        }
    }
}

pub fn caption() -> Command {
    Command::builder()
        .name("caption")
        .options(|o| o
            .optflag("h", "help", "Say this help menu")
            .optopt("u", "url", "URL of image or message including an embedded image to caption", "URL"))
        .run(CaptionsRun)
        .build()
}
