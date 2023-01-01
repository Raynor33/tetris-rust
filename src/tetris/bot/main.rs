extern crate core;

use clap::Parser;
use tetris_rust::tetris::bot::Bot;
use tetris_rust::tetris::bot::strategy::random::Random;
use tetris_rust::tetris::bot::strategy::weighted::Weighted;

#[derive(Parser)]
struct BotArgs {
    /// The strategy name
    #[arg(short, long)]
    strategy: String,
    #[arg(short, long, default_value_t = 0)]
    action_pause: u64,
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

fn main() {
    let bot_args = BotArgs::parse();
    if bot_args.strategy == "random" {
        Bot::new().run(&Random::new(), bot_args.action_pause, bot_args.debug)
    }
    else if bot_args.strategy == "weighted" {
        Bot::new().run(&Weighted::new(), bot_args.action_pause, bot_args.debug)
    } else {
        panic!("not a known strategy")
    }
}