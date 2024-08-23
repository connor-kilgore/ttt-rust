use crate::game;
use crate::board;
use crate::win_checker;
use crate::token;

#[derive(Clone)]
struct Move {
    board: board::Board,
    score: i32,
}

fn get_end_score(win_token: char, bot_token: char, depth: i32, board_size: i32) -> i32 {
    if win_token == 'T' { return 0; } 
    else if win_token == bot_token { return board_size - depth; } 
    else { return depth - board_size; }
}

fn is_bot_turn(depth: i32) -> bool {
    depth % 2 == 0
}

fn current_turn_token(bot_token: char, depth: i32) -> char{
    if is_bot_turn(depth) { bot_token }
    else { token::other_token(bot_token) }
}

fn max_move(moves: Vec<Move>) -> Move {
    let mut max = &moves[0];
    for i in 1..moves.len() {
        if moves[i].score > max.score {
            max = &moves[i];
        }
    }
    max.clone()
}

fn min_move(moves: Vec<Move>) -> Move {
    let mut min = &moves[0];
    for i in 1..moves.len() {
        if moves[i].score < min.score {
            min = &moves[i];
        }
    }
    min.clone()
}

fn best_move(moves: Vec<Move>, depth: i32) -> Move { 
    if is_bot_turn(depth) { max_move(moves) }
    else { min_move(moves) }
}

fn mini_max(mut current_move: Move, token: char, depth: i32) -> Move {
    let result = win_checker::get_tie_or_winner(&current_move.board);
    let mut moves: Vec<Move> = vec![];
    if result != ' ' {
        current_move.score = get_end_score(result, token, depth, current_move.board.board.len() as i32);
        return current_move;
    }
    for i in 0..current_move.board.board.len() {
        if !board::is_taken(&current_move.board, i) {
            let mut new_move = 
            Move {
                board: board::place_value_into_board(&current_move.board, i, current_turn_token(token, depth)),
                score: current_move.score,
            };
            new_move.score = mini_max(new_move.clone(), token, depth + 1).score;
            moves.push(new_move);
        }
    }
    best_move(moves, depth)
}

pub fn play_turn(mut game: game::Game, token: char) -> game::Game {
    let current_move = Move{board: game.board, score: 0};
    let best_move = mini_max(current_move, token, 0);
    game.board = best_move.board;
    game
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_end_score_test_tie() {
        assert_eq!(0, get_end_score('T', 'X', 5, 9));
    }

    #[test]
    fn get_end_score_test_win() {
        assert_eq!(6, get_end_score('X', 'X', 3, 9));
        assert_eq!(4, get_end_score('X', 'X', 5, 9));
    }

    #[test]
    fn get_end_score_test_lose() {
        assert_eq!(-6, get_end_score('O', 'X', 3, 9));
        assert_eq!(-4, get_end_score('O', 'X', 5, 9));
    }

    #[test]
    fn current_turn_token_same() {
        assert_eq!('X', current_turn_token('X', 0));
        assert_eq!('X', current_turn_token('X', 2));
        assert_eq!('X', current_turn_token('X', 100));
    }

    #[test]
    fn current_turn_token_other() {
        assert_eq!('O', current_turn_token('X', 1));
        assert_eq!('O', current_turn_token('X', 3));
        assert_eq!('O', current_turn_token('X', 101));
    }

    #[test]
    fn max_move_test_one_move() {
        let move_one = Move {board: board::new_board(2), score: 0,};
        let best_move = best_move(vec![move_one], 0);
        assert_eq!(0, best_move.score);
        assert_eq!(vec![' ', ' ', ' ', ' '], best_move.board.board);
    }

    #[test]
    fn max_move_test_many_moves() {
        let move_one = Move {board: board::new_board(2), score: 0,};
        let move_two = Move {board: board::Board{board: vec![' ', 'X', ' ', ' '], side_len: 2}, score: 5,};
        let move_three = Move {board: board::Board{board: vec![' ', ' ', 'X', ' '], side_len: 2}, score: 1,};
        let best_move = best_move(vec![move_one, move_two, move_three], 0);
        assert_eq!(5, best_move.score);
        assert_eq!(vec![' ', 'X', ' ', ' '], best_move.board.board);
    }

    #[test]
    fn min_move_test_one_move() {
        let move_one = Move {board: board::new_board(2), score: 0,};
        let worst_move = best_move(vec![move_one], 1);
        assert_eq!(0, worst_move.score);
        assert_eq!(vec![' ', ' ', ' ', ' '], worst_move.board.board);
    }

    #[test]
    fn min_move_test_many_moves() {
        let move_one = Move {board: board::new_board(2), score: 0,};
        let move_two = Move {board: board::Board{board: vec![' ', 'X', ' ', ' '], side_len: 2}, score: 5,};
        let move_three = Move {board: board::Board{board: vec![' ', ' ', 'X', ' '], side_len: 2}, score: -5,};
        let worst_move = best_move(vec![move_one, move_two, move_three], 1);
        assert_eq!(-5, worst_move.score);
        assert_eq!(vec![' ', ' ', 'X', ' '], worst_move.board.board);
    }

    #[test]
    fn mini_max_test_empty() {
        let current_move = Move {board: board::new_board(3), score: 0,};
        let best_move = mini_max(current_move, 'X', 0);
        assert_eq!(vec!['X', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], best_move.board.board);
    }

    #[test]
    fn mini_max_test_block_win() {
        let current_move = Move {board: board::Board{board: vec![' ', ' ', ' ', ' ', ' ', ' ', 'X', 'X', ' '], side_len: 3}, score: 0,};
        let best_move = mini_max(current_move, 'O', 0);
        assert_eq!(vec![' ', ' ', ' ', ' ', ' ', ' ', 'X', 'X', 'O'], best_move.board.board);
    }

    #[test]
    fn mini_max_test_win() {
        let current_move = Move {board: board::Board{board: vec!['X', ' ', ' ', ' ', 'X', ' ', ' ', ' ', ' '], side_len: 3}, score: 0,};
        let best_move = mini_max(current_move, 'X', 0);
        assert_eq!(vec!['X', ' ', ' ', ' ', 'X', ' ', ' ', ' ', 'X'], best_move.board.board);
    }

    #[test]
    fn mini_max_test_win_over_block() {
        let current_move = Move {board: board::Board{board: vec!['O', 'O', ' ', 'X', 'X', ' ', ' ', ' ', ' '], side_len: 3}, score: 0,};
        let best_move = mini_max(current_move, 'X', 0);
        assert_eq!(vec!['O', 'O', ' ', 'X', 'X', 'X', ' ', ' ', ' '], best_move.board.board);
    }


    #[test]
    fn play_turn_test() {
        let mut game = game::default_game();
        game.board = board::Board{board: vec!['O', 'O', ' ', 'X', 'X', ' ', ' ', ' ', ' '], side_len: 3};
        game.round = 4;
        let new_game = play_turn(game, 'X');
        assert_eq!(vec!['O', 'O', ' ', 'X', 'X', 'X', ' ', ' ', ' '], new_game.board.board);
    }
}