
mod sub_modules;

use std::env;
use ggez::GameResult;
use crate::sub_modules::{config, window};


pub fn main() -> GameResult {

    let args: Vec<String> = env::args().collect();
    let config: config::Config = config::parse_args(args);
    window::run(config)
}