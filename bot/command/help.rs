// use clap::{AppSettings, Parser};
// use err_context::AnyError;
// use crate::command::{Command, CommandRun, CommandRunArgs};
//
// struct HelpRun;
//
// impl CommandRun for HelpRun {
//     async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
//         {
//             let r = a.bot.read().await;
//
//             let mut out = String::new();
//             for (k, v) in r.commands {
//
//             }
//         }
//
//         Ok(())
//     }
// }
//
// pub fn help() -> Command {
//     Command::builder("caption")
//         .run(CaptionsRun)
//         .build()
// }
