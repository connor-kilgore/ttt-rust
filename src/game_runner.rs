use crate::tui;
use crate::win_checker;
use crate::game;
use crate::turn_handler;

pub fn play_round(mut game: game::Game) -> game::Game {
    game = turn_handler::play_next_turn(game);
    game.round += 1;
    tui::print_game_round(&game);
    return game;
}

pub fn run_game(mut game: game::Game) {
    let mut winner = ' ';
    tui::print_game_round(&game);
    while winner == ' ' {
        game = play_round(game);
        winner = win_checker::get_tie_or_winner(&game.board);
    }
    tui::print_end_condition(winner);
}

pub fn init_game(token: char) {
    let game: game::Game;
    if token == 'O' { game = game::o_game(); }
    else { game = game::default_game(); }
    run_game(game);
}

#[cfg(test)]
mod test {
    use super::*;

}