extern crate core;

use clap::Parser;
use tetris_rust::tetris::bot::Bot;
use tetris_rust::tetris::bot::strategy::always_drop::AlwaysDrop;

#[derive(Parser)]
struct BotArgs {
    /// The strategy name
    #[arg(short, long)]
    strategy: String,
}

fn main() {
    let bot_args = BotArgs::parse();
    if bot_args.strategy == "always_drop" {
        Bot::new().run(&AlwaysDrop::new())
    } else {
        panic!("not a known strategy")
    }
}