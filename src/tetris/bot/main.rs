extern crate core;

use clap::Parser;
use tetris_rust::tetris::bot::Bot;
use tetris_rust::tetris::bot::strategy::qlearning::QLearning;
use tetris_rust::tetris::bot::strategy::weighted::Weighted;

#[derive(Parser)]
struct BotArgs {
    /// The strategy name
    #[arg(short, long)]
    strategy: String,
    /// How long to pause between actions
    #[arg(short, long, default_value_t = 0)]
    action_pause: u64,
    /// Whether to print the game
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    /// How many games to play
    #[arg(short, long, default_value_t = 1)]
    games: u32,
}

fn main() {
    let bot_args = BotArgs::parse();
    if bot_args.strategy == "weighted" {
        Bot::new().run(&mut Weighted::new(), bot_args.action_pause, bot_args.debug, bot_args.games)
    }
    else if bot_args.strategy == "qlearning" {
        Bot::new().run(&mut QLearning::new(), bot_args.action_pause, bot_args.debug, bot_args.games)
    } else {
        panic!("not a known strategy")
    }
}