use crate::tui;
use crate::win_checker;
use crate::game;
use crate::turn_handler;

pub fn get_game_over(game: &game::Game) -> char {
    match win_checker::get_winner(&game.board) {
        winner if winner != ' ' => winner,
        _ if game.round >= game.board.board.len() as i32 => 'T',
        _ => ' ',
    }
}

pub fn play_round(mut game: game::Game) -> game::Game {
    tui::print_board(&game.board);
    game = turn_handler::play_next_turn(game);
    return game;
}

pub fn run_game(mut game: game::Game) {
    let mut winner = ' ';
    while winner == ' ' {
        game = play_round(game);
        winner = get_game_over(&game);
    }
    tui::print_board(&game.board);
    println!("{} is the winner!", winner);
}

pub fn init_game() {
    let game = game::default_game();
    run_game(game);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board;

    #[test]
    fn get_game_over_test_not_over() {
        let game = game::default_game();
        assert_eq!(' ', get_game_over(&game));
    }

    #[test]
    fn get_game_over_test_x_wins() {
        let mut game = game::default_game();
        game.board = board::Board {
            board: vec!['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('X', get_game_over(&game));
    }

    #[test]
    fn get_game_over_test_o_wins() {
        let mut game = game::default_game();
        game.board = board::Board {
            board: vec!['O', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'O'],
            side_len: 3,
        };
        assert_eq!('O', get_game_over(&game));
    }

    #[test]
    fn get_game_over_test_tie() {
        let mut game = game::default_game();
        game.board = board::Board {
            board: vec![
                 'X', 'X', 'O',
                 'O', 'O', 'X', 
                 'X', 'X', 'O'],
            side_len: 3,
        };
        game.round = 9;
        assert_eq!('T', get_game_over(&game));
    }

    #[test]
    fn get_game_over_final_round() {
        let mut game = game::default_game();
        game.board = board::Board {
            board: vec![
                 'O', 'X', 'X',
                 'O', 'O', 'X', 
                 'X', 'O', 'X'],
            side_len: 3,
        };
        game.round = 9;
        assert_eq!('X', get_game_over(&game));
    }
}