use crate::gpt;
use std::env;
use teloxide::prelude::*;
use teloxide::types::{ChatAction, User};

pub async fn init_telegram_bot(bot: Bot) {
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if msg.text().is_some() {
            handle_message(bot, msg).await;
        }
        ResponseResult::<()>::Ok(())
    })
    .await;
}

async fn handle_message(bot: Bot, msg: Message) {
    // let allowed_user_id: UserId = UserId(
    //     env::var("ALLOWED_USER_ID")
    //         .unwrap()
    //         .parse()
    //         .expect("ALLOWED_USER_ID must be a valid integer"),
    // );

    let white_list: Vec<String> = env::var("ALLOWED_USER_ID")
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    let chat_id = msg.chat.id;

    let user_id = msg.from().unwrap().id.to_string();

    println!("user id: {}", user_id);

    if msg.from().is_none() || !white_list.contains(&user_id) {
        // Ignore messages from users with a different ID
        let _sent = bot
            .send_message(chat_id, "️年轻人，这里不是你该来的地慌")
            .await;
        let _sent = bot.send_message(chat_id, "️不是你该来的地慌").await;
        let _sent = bot.send_message(chat_id, "️该来的地慌").await;
        let _sent = bot.send_message(chat_id, "️地慌").await;
        let _sent = bot.send_message(chat_id, "️慌").await;
        let _sent = bot.send_message(chat_id, "小").await;
        return;
    }

    // if msg.from().is_none() || msg.from().unwrap().id != allowed_user_id {
    //     // Ignore messages from users with a different ID
    //     return;
    // }

    let user_message = msg.text().unwrap_or_default();

    // Send the initial message
    bot.send_chat_action(chat_id, ChatAction::Typing)
        .await
        .unwrap();
    let sent_message = bot.send_message(chat_id, "...✍️").await.unwrap();

    // Fetch and update the message with the GPT output
    if let Err(e) = gpt::fetch_chat_gpt_output(&bot, chat_id, &sent_message, &user_message).await {
        eprintln!("Error fetching GPT output: {:?}", e);
    }
}
