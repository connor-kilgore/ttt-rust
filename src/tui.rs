use crate::board::*;
use crate::{game};
use crate::player;
use crate::ui;
use std::io::{self, Write};

pub struct Tui;

pub const START_SCREEN_MSG: &str = "\n\n############################\n## UNBEATABLE TIC-TAC-TOE ##\n############################\n\n";

fn display_board(board: &Board){
    println!("{}", board.to_string());
}

fn player_turn_str(round: i32) -> String{
    format!("=== PLAYER {} == ROUND {} ===\n", player::current_player_str(round), round)
}

fn end_condition_str(winner: char) -> String {
    format!("=== {} IS THE WINNER! ===\n", player::player_by_token_str(winner))
}

pub fn get_input(prompt: String) -> String{
    println!("{}", prompt);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

impl ui::Ui for Tui {
    fn start_screen(&self) {
        println!("{}", START_SCREEN_MSG);
    }

    fn display_game_round(&self, game: &game::Game) {
        println!("{}", player_turn_str(game.round));
        display_board(&game.board);
    }

    fn display_end(&self, winner: char) {
        println!("{}", end_condition_str(winner));
    }


    fn get_user_move(&self, board: &Board) -> i32{
        let upper: i32 = (board.board.len() - 1) as i32;
        loop {
            let input = get_input(format!("Enter a number between 0-{}: ", upper));
            let num_input: i32 = input.trim().parse::<i32>().unwrap_or(-1);
            match num_input {
                num if (0..=upper).contains(&num) => {
                    if is_taken(board, num as usize){
                        println!("Spot taken.");
                    } else {return num_input;}

                }
                _ => {
                    println!("Invalid input.");
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn start_screen_test() {
        assert!(START_SCREEN_MSG.contains("UNBEATABLE TIC-TAC-TOE"));
    }

    // #[test]
    // fn get_user_move_test() {
    //     let board = new_board(3);
    //     assert_eq!(get_user_move(&board), 0);
    //     assert_eq!(get_user_move(&board), 1);
    //     assert_eq!(get_user_move(&board), 8);
    // }

    #[test]
    fn player_turn_str_test() {
        assert_eq!("=== PLAYER ONE == ROUND 0 ===\n", player_turn_str(0));
        assert_eq!("=== PLAYER ONE == ROUND 6 ===\n", player_turn_str(6));
        assert_eq!("=== PLAYER TWO == ROUND 1 ===\n", player_turn_str(1));
        assert_eq!("=== PLAYER TWO == ROUND 7 ===\n", player_turn_str(7));
    }

    #[test]
    fn end_condition_str_test() {
        assert_eq!("=== NOBODY IS THE WINNER! ===\n", end_condition_str(' '));
        assert_eq!("=== NOBODY IS THE WINNER! ===\n", end_condition_str('Y'));
        assert_eq!("=== PLAYER ONE IS THE WINNER! ===\n", end_condition_str('X'));
        assert_eq!("=== PLAYER TWO IS THE WINNER! ===\n", end_condition_str('O'));
    }
}