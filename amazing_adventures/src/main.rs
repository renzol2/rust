use clap::Parser;

use amazing_adventures::{run_game, Args};

fn main() {
    let args = Args::parse();
    run_game(args);
}
