use serenity::async_trait;
use crate::command::{Command, CommandRun, CommandRunArgs};

struct CaptionsRun;

#[async_trait]
impl CommandRun for CaptionsRun {
    async fn run(&self, a: CommandRunArgs) {
        {
            let r = a.bot.read().await;
            let url = r.latest_image.get(&a.msg.channel_id);
            dbg!(url);
        }
    }
}

pub fn caption() -> Command {
    Command::builder("caption")
        .run(CaptionsRun)

        .build()
}
