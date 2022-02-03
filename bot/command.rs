use crate::bot::BotLock;
use clap::ErrorKind;
use err_context::{AnyError, ErrorExt};
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::Message;
use std::fmt::Debug;
use std::sync::Arc;

pub type CommandAppCreate = Box<dyn Fn(String) -> clap::App<'static> + Send + Sync>;

pub struct CommandRunArgs {
    pub http: Arc<Http>,
    pub bot: BotLock,
    pub matches: clap::ArgMatches,
    pub msg: Message,
}

pub struct Command {
    pub name: &'static str,
    parser: CommandAppCreate,
    runnable: Arc<dyn CommandRun>,
}

impl Command {
    pub fn new(
        name: &'static str,
        parser: CommandAppCreate,
        f: Arc<dyn CommandRun + 'static>,
    ) -> Self {
        Self {
            name,
            parser,
            runnable: f,
        }
    }

    pub fn builder(name: &'static str) -> CommandBuilder {
        CommandBuilder::new(name)
    }

    pub async fn run(&self, http: Arc<Http>, bot: BotLock, args: Vec<String>, msg: Message) {
        let app = (self.parser)(self.name.to_string());
        let matches: clap::Result<clap::ArgMatches> = app.try_get_matches_from(args);

        match matches {
            Ok(matches) => {
                let channel_id = msg.channel_id.clone();
                let result = self
                    .runnable
                    .run(CommandRunArgs {
                        http: http.clone(),
                        bot,
                        matches,
                        msg,
                    })
                    .await;

                if let Err(e) = result {
                    channel_id.say(http, e).await;
                }
            }
            Err(e) => match e.kind {
                ErrorKind::DisplayHelp => {
                    msg.channel_id
                        .say(http, format!("Help: ```{}```", e.to_string()))
                        .await;
                }
                ErrorKind::DisplayVersion => {
                    msg.channel_id.say(http, e.to_string()).await;
                }
                _ => {
                    if e.kind != ErrorKind::DisplayHelp && e.kind != ErrorKind::DisplayVersion {
                        msg.channel_id
                            .say(http, format!("```{}```", e.to_string()))
                            .await;
                    }
                }
            },
        };
    }
}

pub struct CommandBuilder {
    name: &'static str,
    app: Option<CommandAppCreate>,
    run: Option<Arc<dyn CommandRun>>,
}

struct UnimplementedCommandRun;

#[async_trait]
impl CommandRun for UnimplementedCommandRun {
    async fn run(&self, a: CommandRunArgs) -> Result<(), &'static str> {
        unimplemented!()
    }
}

impl CommandBuilder {
    pub fn new(name: &'static str) -> Self {
        Self {
            app: Some(Box::new(|n| clap::App::new(n))),
            name,
            run: None,
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
        Command::new(
            std::mem::take(&mut self.name),
            std::mem::take(&mut self.app).unwrap(),
            run,
        )
    }

    pub fn run(&mut self, f: impl CommandRun + 'static) -> &mut Self {
        self.run = Some(Arc::new(f));
        self
    }

    pub fn parser<H: clap::Parser + 'static>(&mut self) -> &mut Self {
        self.app = Some(Box::new(|n| <H as clap::IntoApp>::into_app().name(n)));
        self
    }

    pub fn options(
        &mut self,
        f: impl Fn(String) -> clap::App<'static> + Send + Sync + 'static,
    ) -> &mut Self {
        self.app = Some(Box::new(f));
        self
    }
}

#[async_trait]
pub trait CommandRun: Send + Sync {
    async fn run(&self, a: CommandRunArgs) -> Result<(), &'static str>;
}
