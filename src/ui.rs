use mockall::automock;
use crate::board::Board;
use crate::game;

#[automock]
pub(crate) trait Ui {
    fn start_screen(&self);
    fn display_game_round(&self, game: &game::Game);
    fn display_end(&self, winner: char);
    fn get_user_move(&self, board: &Board) -> i32;
}