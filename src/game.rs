use crate::board;
use crate::player;
use crate::ui;
use crate::tui;

pub struct Game {
    pub board: board::Board,
    pub round: i32,
    pub player_one: player::Player,
    pub player_two: player::Player,
    pub io: Box<dyn ui::Ui>,
}

pub fn new_game(side_len: i32, one_is_human: bool,
                two_is_human: bool, io: Box<dyn ui::Ui>) -> Game {
    Game {
        board: board::new_board(side_len),
        round: 0,
        player_one: player::Player { token: 'X', is_human: one_is_human},
        player_two: player::Player { token: 'O', is_human: two_is_human,},
        io,
    }
}

pub fn default_game() -> Game {
    Game {
        board: board::new_board(3),
        round: 0,
        player_one: player::new_player('X'),
        player_two: player::new_bot('O'),
        io: Box::new(tui::Tui {}),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tui::Tui;
    
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

    #[test]
    fn o_game_structure() {
        let game = new_game(4, false, true, Box::new(Tui));
        assert_eq!(4, game.board.side_len);
        assert_eq!(16, game.board.board.len());
        assert_eq!(0, game.round);
        assert_eq!('X', game.player_one.token);
        assert_eq!(false, game.player_one.is_human);
        assert_eq!('O', game.player_two.token);
        assert_eq!(true, game.player_two.is_human);
    }
}