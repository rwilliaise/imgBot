use serenity::async_trait;
use crate::command::{Command, CommandRun, CommandRunArgs};

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) {
        if a.matches.opt_present("h") {
            a.msg.channel_id.say(a.http, a.opts.usage("Usage: caption [options] TEXT")).await;
            return;
        }
    }
}

pub fn caption() -> Command {
    Command::builder()
        .name("caption")
        .options(|o| o.optflag("h", "help", "say this help menu"))
        .run(CaptionsRun)
        .build()
}
