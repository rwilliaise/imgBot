mod bot;

#[tokio::main]
async fn main() {
    let mut bot = bot::Bot::new().await.expect("bot creation err");

    bot.start().await.expect("bot start err");
}
