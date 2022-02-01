use std::env;
use std::error::Error;
use std::ops::Add;
use err_context::AnyError;
use reqwest::Response;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
            },
            Interaction,
            InteractionResponseType,
        },
        prelude::*,
        prelude::application_command::ApplicationCommandInteraction
    },
    prelude::*,
    http::Http,
};
use serenity::builder::CreateApplicationCommand;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

pub struct BotHandler {
    client: reqwest::Client,
    url_base: &'static str
}

impl BotHandler {
    fn new() -> Self {
        Self {
            client: reqwest::Client::builder().build().expect("http client build failure"),
            url_base: match env::var("KUBERNETES_SERVICE_HOST") {
                Ok(_) => { // we are running in k8s
                    "http://img-server:8080"
                }
                Err(_) => {
                    "http://localhost:8080"
                }
            }
        }
    }

    async fn add_commands(&self) {

    }

    fn get_url(&self, url: &str) -> String {
        String::from(self.url_base).add(url)
    }

    async fn send_get(&self, url: &str) -> reqwest::Result<Response> {
        self.client.get(self.get_url(url)).send().await
    }

    async fn create_command<F>(
        &self,
        http: &Http,
        f: F
    )
    where
        F: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    {
        ApplicationCommand::create_global_application_command(http, f).await.expect("Command creation err");
    }

    async fn process_command(&self, _ctx: Context, _interaction: Interaction) -> std::result::Result<(), serenity::Error> {
        if let Interaction::ApplicationCommand(command) = _interaction {
            match command.data.name.as_str() {
                "ping" => {
                    match self.send_get("/health").await {
                        Ok(_) => {
                            self.command_response_msg(&command, &_ctx.http, "Pong!", false).await?
                        }
                        Err(e) => {
                            self.command_response_msg(&command, &_ctx.http, &*format!("Ping failure (preferably report this to a dev):\n```{:#?}```", e.source().unwrap_or_else(|| &e)), true).await?
                        }
                    }
                }
                "caption" => {
                    let mut message = self.command_followup_msg(&command, &_ctx.http, "Uploading... 1/4\n🟩⬛⬛⬛", true).await?;
                }
                _ => {
                    self.command_response_msg(&command, &_ctx.http, "Not implemented", true).await?
                }
            }
        };
        Ok(())
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        self.create_command(&_ctx.http, |command| {
            command
                .name("ping")
                .description("Check if imgBot is alive.")
        }).await;

        self.create_command(&_ctx.http, |command| {
            command
                .name("caption")
                .description("Caption an image with some text.")
                .create_option(|option| {
                    option
                        .name("text")
                        .description("Caption text")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                })
                .create_option(|option| {
                    option
                        .name("url")
                        .description("Optionally, target url")
                        .kind(ApplicationCommandOptionType::String)
                        .required(false)
                })
        }).await;

        println!("Starting bot!")
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {
        if let Err(e) = self.process_command(_ctx, _interaction).await {
            println!("Encountered command process error: {}", e)
        };
    }
}

pub struct Bot {
    client: Client
}

impl Bot {
    /// Creates a new instance of imgBot.
    ///
    /// # Panics
    /// If environment variable `IMGBOT_DISCORD_TOKEN` is not set, `#new` will panic.
    /// If building the client fails, `#new` will panic.
    pub async fn new() -> std::result::Result<Self, AnyError> {
        let token = std::env::var("IMGBOT_DISCORD_TOKEN").expect("discord token fetch err");
        let id = std::env::var("IMGBOT_DISCORD_APPID").expect("discord appid fetch err").parse::<u64>()?;
        let client = Client::builder(&token).application_id(id).event_handler(BotHandler::new()).await.expect("client build err");

        Ok(Self {
            client
        })
    }

    pub async fn start(&mut self) -> std::result::Result<(), SerenityError> {
        self.client.start_autosharded().await
    }
}

#[macro_export]
macro_rules! img_url {
    ($($arg:tt)*) => {
        format!("{}{}", URL_BASE, $($arg)*)
    };
}
