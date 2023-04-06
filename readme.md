cargo run

```sh
# start a new tele-gpt session
screen -S tele-gpt

./target/release/tele-gpt

# Detach from the screen session: To detach from the screen session and leave your program running, press CTRL+a, followed by d. This will return you to the regular SSH shell, while leaving the screen session and your Rust program running in the background.

# re-attach
screen -r tele-gpt
```

