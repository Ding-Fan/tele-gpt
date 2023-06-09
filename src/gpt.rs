use bytes::Bytes;
use futures_util::stream::StreamExt;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::time::Duration;
use teloxide::prelude::Requester;
use teloxide::types::{ChatAction, ChatId, Message};
use teloxide::Bot;
use tokio::time::{timeout, Instant};

pub async fn fetch_chat_gpt_output(
    bot: &Bot,
    chat_id: ChatId,
    sent_message: &Message,
    user_message: &str,
) -> Result<(), reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").unwrap();
    let api_url = "https://api.openai.com/v1/chat/completions";

    // println!("user_message: {}", user_message);
    let mut start = Instant::now();

    let payload = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
          // {"role": "system", "content": "You are a helpful assistant."},
          {"role": "user", "content": user_message}
        ],
        "temperature": 0,
        "top_p": 1.0,
        "n": 1,
        "stream": true,
        "presence_penalty": 0.2,
        "frequency_penalty": 0.2
    });

    let client = Client::new();
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut output = String::new();
    let mut count = 0;
    let update_interval = 20; // Update the message every 3 content strings

    while let Ok(Some(chunk_result)) = timeout(Duration::from_secs(3), stream.next()).await {
        match chunk_result {
            Ok(chunk) => {
                let chunk: Bytes = chunk;
                // println!("chunk: {:?}", chunk);
                let mut utf8_str = String::from_utf8_lossy(&chunk).to_string();

                // ! the most first chunk might have two data:
                // chunk: b"data: {\"id\":\"chatcmpl-72kgOPhcOQwHingS2AE69YZBBG75A\",\"object\":\"chat.completion.chunk\",\"created\":1680890460,\"model\":\"gpt-3.5-turbo-0301\",\"choices\":[{\"delta\":{\"role\":\"assistant\"},\"index\":0,\"finish_reason\":null}]}\n\ndata: {\"id\":\"chatcmpl-72kgOPhcOQwHingS2AE69YZBBG75A\",\"object\":\"chat.completion.chunk\",\"created\":1680890460,\"model\":\"gpt-3.5-turbo-0301\",\"choices\":[{\"delta\":{\"content\":\"As\"},\"index\":0,\"finish_reason\":null}]}\n\n"
                // trimmed_str: {"id":"chatcmpl-72kgOPhcOQwHingS2AE69YZBBG75A","object":"chat.completion.chunk","created":1680890460,"model":"gpt-3.5-turbo-0301","choices":[{"delta":{"role":"assistant"},"index":0,"finish_reason":null}]}
                let second_part = utf8_str.splitn(3, "data: ").nth(2).unwrap_or(&utf8_str);

                let trimmed_str = second_part.trim_start_matches("data: ");
                // println!("trimmed_str: {}", trimmed_str);

                // ! the most last chunk might be "[DONE]"
                if trimmed_str.trim_end() == "[DONE]" {
                    // println!("is done: {:?}", trimmed_str);
                    continue;
                }

                let json_result: Result<Value, _> = serde_json::from_str(trimmed_str);
                match json_result {
                    Ok(json) => {
                        if let Some(choices) = json.get("choices") {
                            if let Some(choice) = choices.get(0) {
                                if let Some(content) =
                                    choice.get("delta").and_then(|delta| delta.get("content"))
                                {
                                    if let Some(content_str) = content.as_str() {
                                        //println!("output: {}", content_str);
                                        let content_str = content_str.trim_start_matches('\n');
                                        if content_str.trim().is_empty() {
                                            // Skip this iteration if the content_str only contains whitespace characters
                                            continue;
                                        }
                                        output.push_str(content_str);
                                        count += 1;
                                        if count % update_interval == 0 {
                                            let tmp = format!("{}💭️", output);
                                            bot.send_chat_action(chat_id, ChatAction::Typing)
                                                .await
                                                .unwrap();
                                            bot.edit_message_text(chat_id, sent_message.id, &tmp)
                                                .await
                                                .unwrap();
                                            let end = Instant::now();
                                            let duration = end - start;
                                            println!(
                                                "更新间隔: {}\t平均时间: {:?}",
                                                update_interval,
                                                duration / (update_interval as u32)
                                            );
                                            start = Instant::now();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing JSON: {:?}", e);
                        //break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error fetching chunk: {:?}", e);
                break;
            }
        }
    }

    // Update the message with the final output
    if !output.is_empty() {
        bot.edit_message_text(chat_id, sent_message.id, &output)
            .await
            .unwrap();
    }

    Ok(())
}
