use crate::command::{Command, CommandRun, CommandRunArgs};
use clap::Parser;
use err_context::AnyError;
use serenity::async_trait;

struct HelpRun;

#[async_trait]
impl CommandRun for HelpRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), AnyError> {
        let target_command = a.matches.value_of("command");
        let r = a.bot.read().await;
        if target_command.is_none() {
            let mut prefix = &"".to_string();
            if let Some(id) = &a.msg.guild_id {
                prefix = r.prefix.get(id).unwrap_or(prefix);
            }

            let mut out = format!(
                r#"```imgBot v{}
Type `{}help' to see this list.
Type `{}help name' to find out more about the function `{}name'.

"#,
                env!("CARGO_PKG_VERSION"),
                prefix,
                prefix,
                prefix
            );
            for (k, v) in &r.commands {
                let mut app: clap::App = (v.parser)(k.clone());
                let usage = app.render_usage();
                let usage = &usage[11..];
                out += format!(
                    "{}{:<30}{}\n",
                    prefix,
                    usage,
                    app.get_about().unwrap_or("No description provided")
                )
                .as_str();
            }

            out += "```";

            a.msg.channel_id.say(a.http.clone(), out).await?;
        } else if target_command.is_some() {
            let target_command = target_command.unwrap();
            let command = r.commands.get(target_command);

            if let None = command {
                a.msg
                    .channel_id
                    .say(
                        a.http.clone(),
                        format!("No command named {}", target_command),
                    )
                    .await?;
                return Ok(());
            }

            let command = command.unwrap();
            let mut app: clap::App = (command.parser)(target_command.to_string().clone());
            let mut buf = Vec::new();
            app.write_long_help(&mut buf)?;
            a.msg
                .channel_id
                .say(
                    a.http.clone(),
                    format!("```{}```", std::str::from_utf8(buf.as_slice())?.to_string()),
                )
                .await?;
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
/// Show help messages about commands
struct HelpArgs {
    command: Option<String>,
}

pub fn help() -> Command {
    Command::builder("help")
        .run(HelpRun)
        .parser::<HelpArgs>()
        .build()
}
