// use std::string::ToString;

pub struct Board {
    pub board: Vec<char>,
    pub side_len: i32,
}

pub fn divider(side: i32) -> String {
    return format!("{}\n", "=".repeat(((side * 3) + (side + 1)) as usize));
}

pub fn row(index: i32, board: &Board) -> String {
    let mut row_str = String::from("| ");
    for i in (index)..(index + board.side_len) {
        let mut val = format!("{}", board.board[i as usize]);
        if val == " " {val = format!("{}", i)};
        row_str = format!("{}{}{}", row_str, val, " | ");
    }
    return format!("{}\n", row_str);
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut board_str = String::from(divider(self.side_len));
    
        for i in 0..self.side_len {
            board_str = format!("{}{}{}", board_str, row(i * self.side_len, self), divider(self.side_len));
        }
        return board_str;
    }
}

pub fn new_board(side: i32) -> Board {
    Board {
        board: vec![' '; (side * side) as usize],
        side_len: side,
    }
}

pub fn place_value_into_board(old_board: Board, index: usize, val: char) -> Board {
    let mut new_board = old_board.board.clone();
    new_board[index] = val;
    Board {
        board: new_board,
        side_len: old_board.side_len,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn board_structure() {
        let board = new_board(3);
        assert_eq!(board.board.len(), 9);
        assert_eq!(board.side_len, 3);

        for i in 0..board.board.len() {
            assert_eq!(board.board[i], ' ');
        }
    }

    #[test]
    fn divider_test() {
        assert_eq!(divider(3).chars().count(), 14);
        assert_eq!(divider(4).chars().count(), 18);
    }    

    #[test]
    fn row_test_empty_board() {
        let board = new_board(3);
        assert_eq!(row(0, &board), "| 0 | 1 | 2 | \n");
        assert_eq!(row(3, &board), "| 3 | 4 | 5 | \n");
        assert_eq!(row(6, &board), "| 6 | 7 | 8 | \n");
    }    

    #[test]
    fn row_test_board_with_vals() {
        let mut board = new_board(3);
        board.board[0] = 'X';
        board.board[2] = 'O';
        assert_eq!(row(0, &board), "| X | 1 | O | \n");
    }

    #[test]
    fn to_string_test_empty_board() {
        let board = new_board(3);
        assert!(board.to_string().contains(&divider(board.side_len)));
        assert!(board.to_string().contains(&row(0, &board)));
        assert!(board.to_string().contains(&row(3, &board)));
        assert!(board.to_string().contains(&row(6, &board)));
    }

    #[test]
    fn to_string_test_with_vals() {
        let mut board = new_board(3);
        board.board = ['X', ' ', 'O', ' ', 'X', ' ', 'O', 'O', 'X'].to_vec();
        assert!(board.to_string().contains(&divider(board.side_len)));
        assert!(board.to_string().contains(&row(0, &board)));
        assert!(board.to_string().contains(&row(3, &board)));
        assert!(board.to_string().contains(&row(6, &board)));
    }

    #[test]
    fn to_string_test_4x4() {
        let board = new_board(4);
        assert!(board.to_string().contains(&divider(board.side_len)));
        assert!(board.to_string().contains(&row(0, &board)));
        assert!(board.to_string().contains(&row(4, &board)));
        assert!(board.to_string().contains(&row(8, &board)));
        assert!(board.to_string().contains(&row(12, &board)));
    }
}