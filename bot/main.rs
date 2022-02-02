mod bot;
mod caption;
mod command;
mod process;

#[tokio::main]
async fn main() {
    let mut bot = bot::Bot::new().await.expect("Bot creation err");
    bot.start().await;
}
