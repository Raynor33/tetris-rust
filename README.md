# Tetris rust

## Testing
```shell
cargo test
```

## Releasing
```shell
cargo build --release
```

## Executing

### The game
```shell
target/release/game
```

### A bot
Runs a bot which uses the given strategy (see code for allowed values)
```shell
target/release/bot --strategy nogaps --action_pause 10
```
