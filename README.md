# Tetris rust

## Prerequisites

Install rust, eg 
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Testing
```shell
cargo test
```

## Releasing
```shell
cargo build --release
```

## Running...

### ...the game
```shell
target/release/game
```

### ...a bot
Runs a bot which uses the given strategy (see code for allowed values), eg
```shell
target/release/bot --strategy weighted --action-pause 10 --debug
```
