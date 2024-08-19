use crate::board;
use crate::tui;

pub fn run_game(board: board::Board) {
    let mut current_board = board;
    loop {
        let upper: i32 = (current_board.board.len() - 1) as i32;
        tui::print_board(&current_board);
        let index = tui::get_user_move(tui::get_input(format!("Enter a number between 0-{}: ", upper)), &current_board);
        current_board = board::place_value_into_board(current_board, index as usize, 'X')
    }
}

// TODO: give it a default config
pub fn init_game() {
    let board = board::new_board(3);
    run_game(board);
}