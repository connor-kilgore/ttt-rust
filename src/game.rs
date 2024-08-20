use crate::board;
use crate::tui;

pub fn play_round(board: board::Board, token: char) -> board::Board {
    let upper: i32 = (board.board.len() - 1) as i32;
    tui::print_board(&board);
    let index = tui::get_user_move(tui::get_input(format!("Enter a number between 0-{}: ", upper)), &board);
    return board::place_value_into_board(board, index as usize, token);
}

pub fn run_game(board: board::Board) {
    let mut current_board = board;
    let mut round = 0;
    loop {
        let token: char;
        if round % 2 == 0 { token = 'X'; } else { token = 'O'; }
        current_board = play_round(current_board, token);
        round += 1;
    }
}

// TODO: give it a default config
pub fn init_game() {
    let board = board::new_board(3);
    run_game(board);
}