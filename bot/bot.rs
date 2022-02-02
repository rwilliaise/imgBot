use std::env;
use std::ops::Add;
use err_context::AnyError;
use reqwest::Response;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            Interaction,
        },
        prelude::*,
    },
    prelude::*,
};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use crate::command::Command;

pub struct BotHandler {
    pub prefix: HashMap<GuildId, String>,
    commands: HashMap<String, Command>,
    client: reqwest::Client,
    url_base: &'static str
}

impl BotHandler {
    async fn new() -> Self {
        let mut handler = Self {
            prefix: Default::default(),
            commands: Default::default(),
            client: reqwest::Client::builder().build().expect("http client build failure"),
            url_base: match env::var("KUBERNETES_SERVICE_HOST") {
                Ok(_) => { // we are running in k8s
                    "http://img-server:8080"
                }
                Err(_) => {
                    "http://localhost:8080"
                }
            },
        };

        handler.add_commands().await;
        handler
    }

    async fn add_commands(&mut self) {
        self.add_command(crate::caption::caption()).await;
    }

    fn get_url(&self, url: &str) -> String {
        String::from(self.url_base).add(url)
    }

    async fn send_get(&self, url: &str) -> reqwest::Result<Response> {
        self.client.get(self.get_url(url)).send().await
    }

    async fn add_command(&mut self, mut command: Command) {
        self.commands.insert(std::mem::take(&mut command.name), command);
    }

    async fn process_command(&self, _ctx: Context, _interaction: Interaction) -> std::result::Result<(), serenity::Error> {

        Ok(())
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        let content = &_new_message.content;
        let guild_id = &_new_message.guild_id;

        if let Some(id) = guild_id {
            let default = "=".to_string();
            let prefix = self.prefix.get(&id).unwrap_or(&default);
            if content.starts_with(prefix) {
                println!("Command dispatched");
                let content = &content[prefix.len()..].to_string();
                let split: Vec<String> = crate::process::get_args(content);
                let void = "<void>".to_string();

                let command = split.get(0).unwrap_or(&void);
                println!("Processing command: {}", command);

                let command = self.commands.get(command);

                if let Some(command) = command {
                    command.run(_ctx.http.clone(), split, _new_message).await;
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
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
        let client = Client::builder(&token).application_id(id).event_handler(BotHandler::new().await).await.expect("client build err");

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
