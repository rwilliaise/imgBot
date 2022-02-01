use serenity::http::Http;
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use crate::bot::BotHandler;

pub struct CommandRunArgs<'a> {
    handler: &'a mut BotHandler,
    http: &'a Http,
    matches: getopts::Matches,
}

pub struct Command {
    name: String,
    opts: getopts::Options,
    runnable: Box<dyn Fn(CommandRunArgs)>,
}

impl Command {

    pub fn new<F: 'static>(name: String, opts: getopts::Options, f: F) -> Self
    where
        F: Fn(CommandRunArgs)
    {
        Self {
            name,
            opts,
            runnable: Box::new(f),
        }
    }

    pub fn builder() -> CommandBuilder {
        Default::default()
    }

    pub fn run(
        &self,
        handler: &mut BotHandler,
        http: &Http,
        args: Vec<String>
    ) {
        let matches = self.opts.parse(args);

        match matches {
            Ok(matches) => {
                (self.runnable)(CommandRunArgs {
                    handler,
                    http,
                    matches
                });
            }
            Err(e) => {
                println!("Execution err! {:#?}", e)
            }
        }
    }
}

#[derive(Default)]
pub struct CommandBuilder {
    name: Option<String>,
    opts: getopts::Options,
    run: Option<Box<dyn Fn(CommandRunArgs)>>,
}

impl CommandBuilder {
    pub fn build(&mut self) -> Command {
        let run = match self.run.take() {
            Some(r) => r,
            None => {
                println!("No run provided for command!");
                Box::new(|e: CommandRunArgs| {
                    unimplemented!()
                })
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

    pub fn run<F: 'static>(&mut self, f: F) -> &mut Self
    where
        F: Fn(CommandRunArgs)
    {
        self.run = Some(Box::new(f));
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
