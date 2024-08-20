use crate::board;
use crate::player;

pub struct Game {
    pub board: board::Board,
    pub round: i32,
    pub player_one: player::Player,
    pub player_two: player::Player,
}

pub fn default_game() -> Game {
    Game {
        board: board::new_board(3),
        round: 0,
        player_one: player::new_player('X'),
        player_two: player::new_bot('O'),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn game_structure() {
        let game = default_game();
        assert_eq!(3, game.board.side_len);
        assert_eq!(9, game.board.board.len());
        assert_eq!(0, game.round);
        assert_eq!('X', game.player_one.token);
        assert_eq!(true, game.player_one.is_human);
        assert_eq!('O', game.player_two.token);
        assert_eq!(false, game.player_two.is_human);
    }
}