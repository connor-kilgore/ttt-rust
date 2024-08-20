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
}