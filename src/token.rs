
pub fn other_token(token: char) -> char {
    match token {
        token if token == 'X' => 'O',
        token if token == 'O' => 'X',
        _ => ' ',
    }
}

pub fn is_turn(token: char, round: i32) -> bool {
    return (token == 'X' && round % 2 == 0) || (token == 'O' && round % 2 == 1);
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn other_token_test() {
        assert_eq!('O', other_token('X'));
        assert_eq!('X', other_token('O'));
        assert_eq!(' ', other_token(' '));
        assert_eq!(' ', other_token('F'));
    }

    #[test]
    fn is_turn_test_not_turn() {
        assert_eq!(false, is_turn('X', 1));
        assert_eq!(false, is_turn('O', 0));
    }

    #[test]
    fn is_turn_test_is_turn() {
        assert_eq!(true, is_turn('X', 0));
        assert_eq!(true, is_turn('O', 1));
    }
}