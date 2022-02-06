mod bot;
mod command;
mod process;
mod tenor;

#[tokio::main]
async fn main() {
    let mut bot = bot::Bot::new().await.expect("Bot creation err");
    bot.start().await.unwrap();
}
