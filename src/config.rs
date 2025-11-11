use dotenvy::dotenv;
use std::env;

pub fn get_bot_token() -> String {
    dotenv().ok();
    env::var("BOT_TOKEN").expect("BOT_TOKEN not set in .env")
}

