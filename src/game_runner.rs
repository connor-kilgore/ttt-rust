use mockall::{automock};
use crate::{win_checker};
use crate::game;
use crate::tui::Tui;
use crate::turn_handler;

#[automock]
pub trait Runner {
    fn run_game(&mut self, game: game::Game);
    fn play_round(&mut self, game: game::Game) -> game::Game;
}

pub(crate) struct GameRunner;
impl Runner for GameRunner {
    fn run_game(&mut self, mut game: game::Game) {
        game.io.start_screen();
        game.io.display_game_round(&game);
        let mut winner = ' ';
        while winner == ' ' {
            game = self.play_round(game);
            winner = win_checker::get_tie_or_winner(&game.board);
        }
        game.io.display_end(winner);
    }

    fn play_round(&mut self, mut game: game::Game) -> game::Game {
        game = turn_handler::play_next_turn(game);
        game.round += 1;
        game.io.display_game_round(&game);
        game
    }
}

pub fn init_game(token: char, mut runner: Box<dyn Runner>) {
    let game: game::Game;
    if token == 'O' { game = game::new_game(3, false, true, Box::new(Tui)); }
    else { game = game::default_game(); }
    runner.run_game(game);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ui;
    use crate::board::Board;

    #[test]
    fn init_game_test_x() {
        let mut mock_runner = MockRunner::new();
        mock_runner
            .expect_run_game()
            .withf(|game: &game::Game| {
                game.player_one.token == 'X' &&
                    game.player_one.is_human == true &&
                    game.player_two.token == 'O' &&
                    game.player_two.is_human == false})
            .times(1)
            .return_const(());
        init_game('X', Box::new(mock_runner));
    }

    #[test]
    fn init_game_test_o() {
        let mut mock_runner = MockRunner::new();
        mock_runner
            .expect_run_game()
            .withf(|game: &game::Game| {
                game.player_one.token == 'X' &&
                    game.player_one.is_human == false &&
                    game.player_two.token == 'O' &&
                    game.player_two.is_human == true})
            .times(1)
            .return_const(());
        init_game('O', Box::new(mock_runner));
    }

    #[test]
    fn run_game_test() {
        let mut mock_ui = ui::MockUi::new();
        mock_ui.expect_start_screen().times(1).return_const(());
        mock_ui.expect_display_game_round().times(2).return_const(());
        mock_ui.expect_get_user_move().times(1).return_const(1);
        mock_ui.expect_display_end().times(1).return_const(());
        let mut game = game::new_game(3, true, false, Box::new(mock_ui));
        game.board = Board {board: vec!['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' '], side_len: 3};
        GameRunner.run_game(game);
    }

    #[test]
    fn play_round_test() {
        let mut mock_ui = ui::MockUi::new();
        mock_ui.expect_get_user_move().times(1).return_const(1);
        mock_ui.expect_display_game_round()
            .withf(|game: &game::Game| {
                game.board.board == vec![' ', 'X', ' ', ' ']
            })
            .times(1).return_const(());
        let my_game = game::new_game(2, true, false, Box::new(mock_ui));
        GameRunner.play_round(my_game);
    }
}