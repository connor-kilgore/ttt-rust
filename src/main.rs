mod tui;
mod game;
mod board;
mod win_checker;

fn main() {
    tui::start_screen();
    game::init_game();
}

