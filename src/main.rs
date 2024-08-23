mod tui;
mod game_runner;
mod game;
mod board;
mod win_checker;
mod token;
mod player;
mod turn_handler;
mod bot;
mod ui;

use std::env;

fn start(runner: Box<dyn game_runner::Runner>){
    let args: Vec<String> = env::args().collect();
    let mut token = 'X';
    if args.len() > 1 {
        token = token::token_by_str_arg(args[1].as_str());
    }
    game_runner::init_game(token, runner);
}

fn main() {
    start(Box::new(game_runner::GameRunner))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game_runner::{MockRunner};

    #[test]
    fn main_test() {
        let mut mock_runner = MockRunner::new();
        mock_runner
            .expect_run_game()
            .withf(|game: &game::Game| {
                game.player_one.token == 'X' &&
                    game.player_one.is_human == true &&
                    game.player_two.token == 'O' &&
                    game.player_two.is_human == false
            })
            .times(1)
            .return_const(());
        start(Box::new(mock_runner));
    }
}