use crate::board;

pub fn get_all_the_same(sub_board: &[char]) -> bool {
    return sub_board[0..sub_board.len()].iter().all(|&x| x == sub_board[0] && x != ' ');
}

pub fn coordinate_to_index(x: i32, y: i32, side_len: i32) -> usize{
    return (x + (y * side_len)) as usize;
}

pub fn get_horizontal_winner(board: &board::Board) -> char {
    for i in 0..board.side_len {
        let start = (i * board.side_len) as usize;
        let end = start + board.side_len as usize;
        if get_all_the_same(&board.board[start..end]) {
            return board.board[start];
        }
    }
    return ' ';
}

pub fn get_vertical_winner(board: &board::Board) -> char {
    for n in 0..board.side_len {
        let mut sub_board: Vec<char> = vec![];
        for m in 0..board.side_len {
            sub_board.push(board.board[coordinate_to_index(n, m, board.side_len)]);}
        if get_all_the_same(&sub_board){
            return sub_board[0];
        }
    }
    return ' ';
}

pub fn get_diagonal_winner(board: &board::Board) -> char {
    let mut left_right: Vec<char> = vec![];
    let mut right_left: Vec<char> = vec![];
    for n in 0..board.side_len {
        for m in 0..board.side_len {
            if n == m { left_right.push(board.board[coordinate_to_index(n, m, board.side_len)]); } 
            if n == (m - board.side_len + 1).abs() { right_left.push(board.board[coordinate_to_index(n, m, board.side_len)]); }
        }
    }
    if get_all_the_same(&left_right) { return left_right[0]; } 
    else if get_all_the_same(&right_left) { return right_left[0]; }
    else { return ' '; }
}

pub fn get_winner(board: &board::Board) -> char {
    let wins = [get_horizontal_winner(board), get_vertical_winner(board), get_diagonal_winner(board)];
    return *wins.iter().find(|&&x| x != ' ').unwrap_or(&' ');
}

fn is_board_full(board: &board::Board) -> bool {
    return board.board.iter().all(|&c| c != ' ');
}

pub fn get_tie_or_winner(board: &board::Board) -> char {
    let winner = get_winner(board);
    if winner == ' ' && is_board_full(board) { return 'T'; }
    else { return winner; }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn get_horizontal_winner_test_empty() {
        let board = board::new_board(3);
        assert_eq!(' ', get_horizontal_winner(&board));
    }

    #[test]
    fn get_horizontal_winner_test_false() {
        let board = board::Board {
                board: vec![' ', 'X', 'X', 'X', 'X', ' ', 'O', ' ', 'X'],
                side_len: 3,
            };

        assert_eq!(' ', get_horizontal_winner(&board));
    }

    #[test]
    fn get_horizontal_winner_test_true() {
        let mut board = board::Board {
                board: vec!['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' '],
                side_len: 3,
            };
        assert_eq!('X', get_horizontal_winner(&board));

        board = board::Board {
            board: vec![' ', ' ', ' ', 'O', 'O', 'O', ' ', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('O', get_horizontal_winner(&board));
    }

    #[test]
    fn get_horizontal_winner_test_4x4() {
        let board = board::Board {
                board: vec![
                    'X', 'X', 'X', 'X',
                     ' ', ' ', ' ', ' ', 
                     ' ', ' ', ' ', ' ', 
                     ' ', ' ', ' ', ' '],
                side_len: 4,
            };
        assert_eq!('X', get_horizontal_winner(&board));
    }

    #[test]
    fn get_vertical_winner_test_empty() {
        let board = board::new_board(3);
        assert_eq!(' ', get_vertical_winner(&board));
    }

    #[test]
    fn get_vertical_winner_test_false() {
        let board = board::Board {
            board: vec!['X', ' ', 'X', 'O', 'O', ' ', 'X', ' ', ' '],
            side_len: 3,
        };
        assert_eq!(' ', get_vertical_winner(&board));
    }

    #[test]
    fn get_vertical_winner_test_true() {
        let mut board = board::Board {
            board: vec!['X', ' ', 'X', 'X', 'O', ' ', 'X', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('X', get_vertical_winner(&board));

        board = board::Board {
            board: vec![' ', 'O', 'X', ' ', 'O', ' ', 'X', 'O', ' '],
            side_len: 3,
        };
        assert_eq!('O', get_vertical_winner(&board));
    }

    #[test]
    fn get_vertical_winner_test_4x4() {
        let board = board::Board {
                board: vec![
                    'O', ' ', ' ', ' ', 
                    'O', ' ', ' ', ' ', 
                    'O', ' ', ' ', ' ', 
                    'O', ' ', ' ', ' '],
                side_len: 4,
            };
        assert_eq!('O', get_vertical_winner(&board));
    }

    #[test]
    fn get_diagonal_winner_test_empty() {
        let board = board::new_board(3);
        assert_eq!(' ', get_diagonal_winner(&board));
    }

    #[test]
    fn get_diagonal_winner_test_false() {
        let board = board::Board {
            board: vec!['X', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'X'],
            side_len: 3,
        };
        assert_eq!(' ', get_diagonal_winner(&board));
    }

    #[test]
    fn get_diagonal_winner_test_true() {
        let board = board::Board {
            board: vec!['X', ' ', ' ', ' ', 'X', ' ', ' ', ' ', 'X'],
            side_len: 3,
        };
        assert_eq!('X', get_diagonal_winner(&board));

        let board = board::Board {
            board: vec![' ', ' ', 'O', ' ', 'O', ' ', 'O', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('O', get_diagonal_winner(&board));
    }
 
    #[test]
    fn get_diagonal_winner_test_4x4() {
        let board = board::Board {
                board: vec![
                    'O', ' ', ' ', ' ', 
                    ' ', 'O', ' ', ' ', 
                    ' ', ' ', 'O', ' ', 
                    ' ', ' ', ' ', 'O'],
                side_len: 4,
            };
        assert_eq!('O', get_diagonal_winner(&board));

        let board = board::Board {
            board: vec![
                ' ', ' ', ' ', 'X', 
                ' ', ' ', 'X', ' ', 
                ' ', 'X', ' ', ' ', 
                'X', ' ', ' ', ' '],
            side_len: 4,
        };
        assert_eq!('X', get_diagonal_winner(&board));
    }

    #[test]
    fn get_winner_test_empty() {
        let board = board::new_board(3);
        assert_eq!(' ', get_winner(&board));
    }

    #[test]
    fn get_winner_test_false() {
        let board = board::Board {
            board: vec!['X', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'X'],
            side_len: 3,
        };
        assert_eq!(' ', get_winner(&board));
    }

    #[test]
    fn get_winner_test_true() {
        let mut board = board::Board {
            board: vec!['X', ' ', ' ', ' ', 'X', ' ', ' ', ' ', 'X'],
            side_len: 3,
        };
        assert_eq!('X', get_winner(&board));

        board = board::Board {
            board: vec!['O', ' ', ' ', 'O', ' ', ' ', 'O', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('O', get_winner(&board));

        board = board::Board {
            board: vec!['O', ' ', ' ', ' ', 'O', ' ', ' ', ' ', 'O'],
            side_len: 3,
        };
        assert_eq!('O', get_winner(&board));
    }

    #[test]
    fn get_tie_or_winner_is_ongoing() {
        let board = board::new_board(3);
        assert_eq!(' ', get_tie_or_winner(&board));
    }

    #[test]
    fn get_tie_or_winner_x_win() {
        let board = board::Board {
            board: vec!['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('X', get_tie_or_winner(&board));
    }

    #[test]
    fn get_tie_or_winner_o_win() {
        let board = board::Board {
            board: vec!['O', ' ', ' ', 'O', ' ', ' ', 'O', ' ', ' '],
            side_len: 3,
        };
        assert_eq!('O', get_tie_or_winner(&board));
    }

    #[test]
    fn get_tie_or_winner_is_tie() {
        let board = board::Board {
            board: vec![
                'X', 'O', 'O',
                'O', 'X', 'X', 
                'X', 'X', 'O'],
            side_len: 3,
        };
        assert_eq!('T', get_tie_or_winner(&board));
    }

    #[test]
    fn is_board_full_test_false() {
        let board = board::new_board(3);
        assert_eq!(false, is_board_full(&board));
    }

    #[test]
    fn is_board_full_test_true() {
        let board = board::Board{board: vec!['X'; 9], side_len: 3};
        assert_eq!(true, is_board_full(&board));
    }
}