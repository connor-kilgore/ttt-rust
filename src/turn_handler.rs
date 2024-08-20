use crate::game::Game;
use crate::board;
use crate::tui;
use crate::player::Player;

pub fn play_human_turn(mut game: Game, token: char) -> Game {
    let upper: i32 = (game.board.board.len() - 1) as i32;
    let index = tui::get_user_move(tui::get_input(format!("Enter a number between 0-{}: ", upper)), &game.board);
    game.board = board::place_value_into_board(&game.board, index as usize, token);
    return game;
}

pub fn play_turn(game: Game, player: &Player) -> Game {
    if player.is_human { return play_human_turn(game, player.token); }
    else { return play_human_turn(game, player.token); }
}

pub fn next_player(game: &Game) -> Player {
    if game.round % 2 == 0 { return game.player_one.clone(); }
    else { return game.player_two.clone(); }
}

pub fn play_next_turn(mut game: Game) -> Game {
    let player = &next_player(&game);
    game = play_turn(game, player);
    game.round += 1;
    return game;
}