#[derive(Clone)]
pub struct Player {
    pub token: char,
    pub is_human: bool,
}

pub fn new_player(token: char) -> Player {
    Player {
        token: token,
        is_human: true,
    }
}

pub fn new_bot(token: char) -> Player {
    Player {
        token: token,
        is_human: false,
    }
}

pub fn current_player_str(round: i32) -> String {
    match round % 2 == 0 {
        true => "ONE".to_string(),
        false => "TWO".to_string(),
    }
}

pub fn player_by_token_str(token: char) -> String {
    match token {
        'X' => "PLAYER ONE".to_string(),
        'O' => "PLAYER TWO".to_string(),
        _ => "NOBODY".to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn new_player_test() {
        let player = new_player('X');
        assert_eq!('X', player.token);
        assert_eq!(true, player.is_human);
    }

    #[test]
    fn new_bot_test() {
        let player = new_bot('O');
        assert_eq!('O', player.token);
        assert_eq!(false, player.is_human);
    }

    #[test]
    fn current_player_str_test_one() {
        assert_eq!("ONE", current_player_str(0));
        assert_eq!("ONE", current_player_str(6));
    }

    #[test]
    fn current_player_str_test_two() {
        assert_eq!("TWO", current_player_str(1));
        assert_eq!("TWO", current_player_str(7));
    }

    #[test]
    fn current_player_by_token_str_test() {
        assert_eq!("NOBODY", player_by_token_str(' '));
        assert_eq!("NOBODY", player_by_token_str('Y'));
        assert_eq!("PLAYER ONE", player_by_token_str('X'));
        assert_eq!("PLAYER TWO", player_by_token_str('O'));
    }
}