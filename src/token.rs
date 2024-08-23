
pub fn other_token(token: char) -> char {
    match token {
        token if token == 'X' => 'O',
        token if token == 'O' => 'X',
        _ => ' ',
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
    fn token_by_str_arg_test() {
        assert_eq!('X', token_by_str_arg("x"));
        assert_eq!('X', token_by_str_arg(" "));
        assert_eq!('O', token_by_str_arg("o"));
        assert_eq!('O', token_by_str_arg("O"));
    }
}