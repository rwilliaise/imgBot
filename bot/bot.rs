use err_context::AnyError;
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

struct BotHandler;

impl BotHandler {
    async fn command_response_msg(&self, command: &ApplicationCommandInteraction, http: impl AsRef<Http>, content: &str, ephemeral: bool) -> std::result::Result<(), SerenityError> {
        command.create_interaction_response(http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.content(content)
                        .flags(match ephemeral {
                            true => InteractionApplicationCommandCallbackDataFlags::EPHEMERAL,
                            _ => InteractionApplicationCommandCallbackDataFlags::empty()
                        })
                )
        }).await
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        ApplicationCommand::create_global_application_command(&_ctx.http, |command| {
            command
                .name("ping")
                .description("Check if imgBot is alive.")
        }).await.expect("Command creation err");
    }

    async fn interaction_create(&self, _ctx: Context, _interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = _interaction {
            match command.data.name.as_str() {
                "ping" => {
                    self.command_response_msg(&command, &_ctx.http, "Pong!", false).await;
                }
                _ => {
                    self.command_response_msg(&command, &_ctx.http, "Not implemented", true).await;
                }
            };
        }
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
        let client = Client::builder(&token).application_id(id).event_handler(BotHandler).await.expect("client build err");

        Ok(Self {
            client
        })
    }

    pub async fn start(&mut self) -> std::result::Result<(), SerenityError> {
        self.client.start_autosharded().await
    }
}
