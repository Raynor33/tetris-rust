# Tetris rust

## Prerequisites

Install rust, eg 
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install libtorch from https://download.pytorch.org/libtorch/cpu/libtorch-macos-1.13.1.zip
export LIBTORCH=<path to unzipped libtorch>
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

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
