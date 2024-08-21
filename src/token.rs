
pub fn other_token(token: char) -> char {
    match token {
        token if token == 'X' => 'O',
        token if token == 'O' => 'X',
        _ => ' ',
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
}