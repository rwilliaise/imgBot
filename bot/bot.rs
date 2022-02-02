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
use std::sync::Arc;
use crate::command::Command;

pub struct BotHandler {
    bot: BotLock
}

impl BotHandler {
    async fn new(bot: BotLock) -> Self {
        Self {
            bot
        }
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn message(&self, _ctx: Context, _new_message: Message) {
        let content = &_new_message.content;
        let guild_id = &_new_message.guild_id;

        if let Some(id) = guild_id {
            for attachment in &_new_message.attachments {
                if let Some(content) = &attachment.content_type {
                    if content == "image/png" {
                        println!("Attachment found!");
                        let mut w = self.bot.write().await;
                        w.latest_image.insert(_new_message.channel_id.clone(), attachment.url.clone());
                    }
                }
            }

            let r = self.bot.read().await;
            let default = "=".to_string();
            let prefix = r.prefix.get(&id).unwrap_or(&default);
            if content.starts_with(prefix) {
                println!("Command dispatched");
                let content = &content[prefix.len()..].to_string();
                let split: Vec<String> = crate::process::get_args(content);
                let void = "<void>".to_string();

                let command = split.get(0).unwrap_or(&void);
                println!("Processing command: {}", command);

                let command = r.commands.get(command);

                if let Some(command) = command {
                    command.run(_ctx.http.clone(), self.bot.clone(), split, _new_message).await;
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Starting bot!")
    }
}

pub type BotLock = Arc<RwLock<BotData>>;

pub struct BotData {
    pub prefix: HashMap<GuildId, String>,
    pub latest_image: HashMap<ChannelId, String>,
    pub client: Arc<reqwest::Client>,
    commands: HashMap<String, Command>,
    url_base: &'static str,
}

impl BotData {
    /// Creates a new instance of imgBot.
    ///
    /// # Panics
    /// If environment variable `IMGBOT_DISCORD_TOKEN` is not set, `#new` will panic.
    /// If building the client fails, `#new` will panic.
    pub async fn new() -> BotLock {
        let mut bot = Arc::new(RwLock::new(Self {
            prefix: Default::default(),
            commands: Default::default(),
            latest_image: Default::default(),
            client: Arc::new(reqwest::Client::builder().build().expect("http client build failure")),
            url_base: match env::var("KUBERNETES_SERVICE_HOST") {
                Ok(_) => { // we are running in k8s
                    "http://img-server:8080"
                }
                Err(_) => {
                    "http://localhost:8080"
                }
            },
        }));

        {
            let mut guard = bot.write().await;
            guard.add_commands().await;
        }

        bot
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

pub struct Bot {
    lock: BotLock,
    bot_client: Client,
}

impl Bot {
    pub async fn new() -> std::result::Result<Self, AnyError> {
        let token = std::env::var("IMGBOT_DISCORD_TOKEN").expect("discord token fetch err");
        let id = std::env::var("IMGBOT_DISCORD_APPID").expect("discord appid fetch err").parse::<u64>()?;
        let mut bot = BotData::new().await;

        Ok(Self {
            bot_client: Client::builder(&token).application_id(id).event_handler(BotHandler::new(bot.clone()).await).await.expect("client build err"),
            lock: bot,
        })
    }

    pub async fn start(&mut self) -> std::result::Result<(), SerenityError> {
        self.bot_client.start_autosharded().await
    }

}

#[macro_export]
macro_rules! img_url {
    ($($arg:tt)*) => {
        format!("{}{}", URL_BASE, $($arg)*)
    };
}
