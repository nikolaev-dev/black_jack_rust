use std::io::{self, BufRead};
use round::*;
use game::*;

pub fn show_round_info(round: &Round) {
    println!();
    println!("====");
    println!("player_score: {}", round.player_score());
    println!("dealer_score: {}", round.dealer_score());
    println!("player cards: {:?}", round.player_cards_str );
    println!("dealer cards: {:?}", round.dealer_cards_str );
}

pub fn show_game_info(game: &Game) {
    println!("player_balance: {}", game.player_balance);
    println!("dealer_balance: {}", game.dealer_balance);

}

pub fn input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).expect("Could not read line");
    input.pop();
    input
}
