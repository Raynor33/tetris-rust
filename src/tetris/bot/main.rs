extern crate core;

use clap::Parser;
use tetris_rust::tetris::bot::Bot;
use tetris_rust::tetris::bot::strategy::random::Random;

#[derive(Parser)]
struct BotArgs {
    /// The strategy name
    #[arg(short, long)]
    strategy: String,
}

fn main() {
    let bot_args = BotArgs::parse();
    if bot_args.strategy == "random" {
        Bot::new().run(&Random::new())
    } else {
        panic!("not a known strategy")
    }
}