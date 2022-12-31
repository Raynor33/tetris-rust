extern crate core;

use clap::Parser;
use tetris_rust::tetris::bot::Bot;
use tetris_rust::tetris::bot::strategy::nogaps::NoGaps;
use tetris_rust::tetris::bot::strategy::random::Random;

#[derive(Parser)]
struct BotArgs {
    /// The strategy name
    #[arg(short, long)]
    strategy: String,
    #[arg(short, long, default_value_t = 0)]
    action_pause: u64
}

fn main() {
    let bot_args = BotArgs::parse();
    if bot_args.strategy == "random" {
        Bot::new().run(&Random::new(), bot_args.action_pause)
    }
    else if bot_args.strategy == "nogaps" {
        Bot::new().run(&NoGaps::new(), bot_args.action_pause)
    } else {
        panic!("not a known strategy")
    }
}