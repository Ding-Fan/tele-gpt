cargo run

cargo build --release

```sh
# start a new tele-gpt session
screen -S tele-gpt

./target/release/tele_gpt

# Detach from the screen session: To detach from the screen session and leave your program running, press CTRL+a, followed by d. This will return you to the regular SSH shell, while leaving the screen session and your Rust program running in the background.

# re-attach
screen -r tele-gpt
```


### resources

https://github.com/tokio-rs/tokio
A runtime for writing reliable asynchronous applications with Rust. Provides I/O, networking, scheduling, timers, ...

https://github.com/seanmonstar/reqwest
An easy and powerful Rust HTTP Client

https://github.com/teloxide/teloxide/network/dependents?dependents_after=Mjc5OTg5NTc1MTE
public repositories using teloxide

https://github.com/BillGPT/GPTelegram/blob/main/src/gpt.rs
https://github.com/cyphersnake/greetings-chatgpt/blob/master/src/main.rs
https://github.com/Yevgnen/chatgpt_bot/blob/7aac31e8ee1555274114da98f3e5a059574923fc/src/main.rs#L151

