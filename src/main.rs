use std::io::{self, BufRead};
extern crate rand;

mod card;
mod deck;
use deck::*;
use card::*;

struct Round {
    player_name: String,
    deck:         Vec<Card>,
    player_cards: Vec<Card>,
    dealer_cards: Vec<Card>,
    player_cards_str: Vec<String>,
    dealer_cards_str: Vec<String>
}

impl Round {

    fn player_score(&self) -> i32 {
        self.player_cards.iter().map(|s| s.value).sum()
    }

    fn dealer_score(&self) -> i32 {
        self.dealer_cards.iter().map(|s| s.value).sum()
    }

    fn first_turn(&mut self) {
        self.player_cards.push(self.deck.remove(0));
        self.player_cards.push(self.deck.remove(0));
        self.dealer_cards.push(self.deck.remove(0));
        self.dealer_cards.push(self.deck.remove(0));
        self.cards_to_str();

    }

    fn cards_to_str(&mut self) {
        self.player_cards_str = self.player_cards.clone().into_iter().map(|c| c.name).collect();
        self.dealer_cards_str = self.dealer_cards.clone().into_iter().map(|c| c.name).collect();
    }

    fn player_turn(&mut self) {

        println!("Введите 1, если хотите взять еще одну карту");
        println!("Нажмите Enter, чтобы пропустить");

        let input = text_input();
        if input == String::from("1") {
            self.player_cards.push(self.deck.remove(0));
        }
        self.cards_to_str();
    }

    fn show_info(&self) {
        println!("player_score: {}", self.player_score());
        println!("dealer_score: {}", self.dealer_score());
        println!("player cards: {:?}", self.player_cards_str );
        println!("player cards: {:?}", self.dealer_cards_str );
    }
}


fn main() {

    let mut round = initial_round();

    println!("player_name:  {}", round.player_name);

    round.first_turn();
    round.show_info();
    round.player_turn();
    round.show_info();
}



fn initial_round() -> Round {
    println!("Введите имя игрока");
    let name = text_input();
    Round {
        player_name: name,
        player_cards: vec![],
        player_cards_str: vec![],
        dealer_cards_str: vec![],
        dealer_cards: vec![],
        deck: deck::new()
    }
}


fn text_input() -> String {
    let mut name = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut name).expect("Could not read line");
    name.pop();
    name
}


// TODO сделать interface::show_info(&round)
// TODO сделать deck::new
