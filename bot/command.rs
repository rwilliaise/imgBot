use std::fmt::Debug;
use std::sync::Arc;
use clap::ErrorKind;
use err_context::ErrorExt;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::async_trait;
use crate::bot::BotLock;

pub type CommandAppCreate = Box<dyn Fn() -> clap::App<'static> + Send + Sync>;

pub struct CommandRunArgs {
    pub http: Arc<Http>,
    pub bot: BotLock,
    pub matches: clap::ArgMatches,
    pub msg: Message,
}

pub struct Command {
    pub name: String,
    parser: CommandAppCreate,
    runnable: Arc<dyn CommandRun>,
}

impl Command {

    pub fn new(name: String, parser: CommandAppCreate, f: Arc<dyn CommandRun + 'static>) -> Self {
        Self {
            name,
            parser,
            runnable: f,
        }
    }

    pub fn builder(name: &'static str) -> CommandBuilder {
        CommandBuilder::new(name)
    }

    pub async fn run(
        &self,
        http: Arc<Http>,
        bot: BotLock,
        args: Vec<String>,
        msg: Message
    ) {
        let app = (self.parser)();
        let matches: clap::Result<clap::ArgMatches> = app.try_get_matches_from(args);

        match matches {
            Ok(matches) => {
                self.runnable.run(CommandRunArgs {
                    http,
                    bot,
                    matches,
                    msg,
                }).await;
            }
            Err(e) => {
                match e.kind {
                    ErrorKind::DisplayHelp => {
                        msg.channel_id.say(http, format!("Help: ```{}```", e.to_string())).await;
                    }
                    ErrorKind::DisplayVersion => {
                        msg.channel_id.say(http, e.to_string()).await;
                    }
                    _ => {
                        if e.kind != ErrorKind::DisplayHelp && e.kind != ErrorKind::DisplayVersion {
                            msg.channel_id.say(http, format!("```{}```", e.to_string())).await;
                        }
                    }
                }
            }
        };
    }
}

pub struct CommandBuilder {
    name: Option<String>,
    app: Option<CommandAppCreate>,
    run: Option<Arc<dyn CommandRun>>,
}

struct UnimplementedCommandRun;

#[async_trait]
impl CommandRun for UnimplementedCommandRun {
    async fn run(&self, a: CommandRunArgs) {
        unimplemented!()
    }
}

impl CommandBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            app: Some(Box::new(|| { clap::App::new(&name.to_string()) })),
            name: Some(String::from(name)),
            run: None
        }
    }

    pub fn build(&mut self) -> Command {
        let mut run = match self.run.take() {
            Some(r) => r,
            None => {
                println!("No run provided for command!");
                Arc::new(UnimplementedCommandRun)
            }
        };

        let name = match self.name.take() {
            Some(n) => n,
            None => {
                panic!("No name provided for command!")
            }
        };

        Command::new(name, std::mem::take(&mut self.app).unwrap(), run)
    }

    pub fn run(&mut self, f: impl CommandRun + 'static) -> &mut Self {
        self.run = Some(Arc::new(f));
        self
    }

    pub fn options(&mut self, f: CommandAppCreate) -> &mut Self {
        self.app = Some(f);
        self
    }
}

#[async_trait]
pub trait CommandRun: Send + Sync {

    async fn run(&self, a: CommandRunArgs);
}
