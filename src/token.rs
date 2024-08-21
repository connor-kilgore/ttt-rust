
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

pub fn current_token (round: i32) -> char {
    match is_turn('X', round){
        true => 'X',
        _ => 'O',
    }
}

pub fn token_by_str_arg(arg: &str) -> char{
    match arg {
        "o" | "O" => 'O',
        _ => 'X',
    }
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

    #[test]
    fn current_token_test_x() {
        assert_eq!('X', current_token(0));
    }

    #[test]
    fn token_by_str_arg_test() {
        assert_eq!('X', token_by_str_arg("x"));
        assert_eq!('X', token_by_str_arg(" "));
        assert_eq!('O', token_by_str_arg("o"));
        assert_eq!('O', token_by_str_arg("O"));
    }
}