# Tetris rust

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
target/release/bot --strategy fixed --action-pause 10 --debug
```
