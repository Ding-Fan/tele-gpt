mod gpt;
mod telegram;

use dotenvy::dotenv;

// use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    // .await;

    telegram::init_telegram_bot(bot).await;
}
