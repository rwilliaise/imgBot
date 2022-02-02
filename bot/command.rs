use std::sync::Arc;
use getopts::Fail;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::async_trait;
use crate::bot::BotLock;

pub struct CommandRunArgs {
    pub http: Arc<Http>,
    pub bot: BotLock,
    pub matches: getopts::Matches,
    pub msg: Message,
    pub opts: Arc<getopts::Options>,
}

pub struct Command {
    pub name: String,
    opts: Arc<getopts::Options>,
    runnable: Arc<dyn CommandRun>,
}

impl Command {

    pub fn new(name: String, opts: getopts::Options, f: Arc<dyn CommandRun + 'static>) -> Self {
        Self {
            name,
            opts: Arc::new(opts),
            runnable: f,
        }
    }

    pub fn builder() -> CommandBuilder {
        Default::default()
    }

    pub async fn run(
        &self,
        http: Arc<Http>,
        bot: BotLock,
        args: Vec<String>,
        msg: Message
    ) {
        let opts = self.opts.clone();
        let matches = self.opts.parse(&args[1..]);

        match matches {
            Ok(matches) => {
                self.runnable.run(CommandRunArgs {
                    http,
                    bot,
                    matches,
                    msg,
                    opts
                }).await;
            }
            Err(e) => {
                println!("Execution err! {:#?}", e);

                match e {
                    Fail::UnrecognizedOption(opt) => {
                        msg.channel_id.say(&http, format!("Unrecognized option `{}`", opt)).await;
                    }
                    Fail::OptionDuplicated(opt) => {
                        msg.channel_id.say(&http, format!("Duplicated option `{}`", opt)).await;
                    }
                    Fail::UnexpectedArgument(arg) => {
                        msg.channel_id.say(&http, format!("Unexpected arg `{}`", arg)).await;
                    }
                    Fail::ArgumentMissing(arg) => {
                        msg.channel_id.say(&http, format!("Missing arg `{}`", arg)).await;
                    }
                    Fail::OptionMissing(opt) => {
                        msg.channel_id.say(&http, format!("Missing option `{}`", opt)).await;
                    }
                }
            }
        };
    }
}

#[derive(Default)]
pub struct CommandBuilder {
    name: Option<String>,
    opts: getopts::Options,
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

        Command::new(name, std::mem::take(&mut self.opts), run)
    }

    pub fn run(&mut self, f: impl CommandRun + 'static) -> &mut Self {
        self.run = Some(Arc::new(f));
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn options<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut getopts::Options) -> &mut getopts::Options
    {
        f(&mut self.opts);
        self
    }
}

#[async_trait]
pub trait CommandRun: Send + Sync {

    async fn run(&self, a: CommandRunArgs);
}
