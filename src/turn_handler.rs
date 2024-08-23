use crate::game::Game;
use crate::board;
use crate::player::Player;
use crate::bot;

pub fn play_human_turn(mut game: Game, token: char) -> Game {
    let index = game.io.get_user_move(&game.board);
    game.board = board::place_value_into_board(&game.board, index as usize, token);
    game
}

pub fn play_bot_turn(game: Game, token: char) -> Game {
    bot::play_turn(game, token)
}

pub fn play_turn(game: Game, player: &Player) -> Game {
    if player.is_human { play_human_turn(game, player.token) }
    else { play_bot_turn(game, player.token) }
}

pub fn next_player(game: &Game) -> Player {
    if game.round % 2 == 0 { game.player_one.clone() }
    else { game.player_two.clone() }
}

pub fn play_next_turn(mut game: Game) -> Game {
    let player = &next_player(&game);
    game = play_turn(game, player);
    game
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{game};
    use crate::ui;

    #[test]
    fn next_player_test_player_one() {
        let game = game::default_game();
        let player = next_player(&game);
        assert_eq!('X', player.token);
        assert_eq!(true, player.is_human);
    }

    #[test]
    fn next_player_test_player_two() {
        let mut game = game::default_game();
        game.round += 1;
        let player = next_player(&game);
        assert_eq!('O', player.token);
        assert_eq!(false, player.is_human);
    }

    #[test]
    fn play_next_turn_human() {
        let mut mock_ui = ui::MockUi::new();
        mock_ui.expect_get_user_move().times(1).return_const(1);
        let game = game::new_game(2, true, false, Box::new(mock_ui));
        assert_eq!(vec![' ', 'X', ' ', ' '], play_next_turn(game).board.board);
    }

    #[test]
    fn play_turn_bot() {
        let mut mock_ui = ui::MockUi::new();
        mock_ui.expect_get_user_move().times(0).return_const(1);
        let game = game::new_game(2, false, true, Box::new(mock_ui));
        assert_eq!(vec!['X', ' ', ' ', ' '], play_next_turn(game).board.board);
    }
}
