mod tui;
mod game_runner;
mod game;
mod board;
mod win_checker;
mod token;
mod player;
mod turn_handler;
mod bot;

fn main() {
    tui::start_screen();
    game_runner::init_game();
}

