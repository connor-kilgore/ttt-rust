mod tui;
mod game_runner;
mod game;
mod board;
mod win_checker;
mod token;
mod player;
mod turn_handler;
mod bot;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut token = 'X';
    if args.len() > 1 {
        token = token::token_by_str_arg(args[1].as_str());
    }
    tui::start_screen();
    game_runner::init_game(token);
}

