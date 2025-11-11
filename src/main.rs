mod bot;
mod qr;
mod config;

use teloxide::Bot;
use config::get_bot_token;
use bot::run_bot;

#[tokio::main]
async fn main() {
    let token = get_bot_token();
    let bot = Bot::new(token);

    println!("Bot is running...");
    run_bot(bot).await;
}

