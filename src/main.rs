mod tui;
mod game;
mod board;


fn main() {
    tui::start_screen();
    game::init_game();
}

