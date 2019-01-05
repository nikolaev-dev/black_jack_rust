use std::io::{self, BufRead};
use deck::*;
extern crate rand;

mod deck {

    use rand::Rng;


    pub struct Card {
        pub value: i32,
        pub name: String
    }

    impl Clone for Card {
        fn clone(&self) -> Card {
            Card {
                value: self.value,
                name: self.name.clone()
            }
        }
    }

    pub fn new() -> Vec<Card> {
        let suits = vec!["♠", "♥", "♣", "♦"];
        let values = vec![(2,"2"),(3,"3"),(4,"4"),(5,"5"),
                          (6,"6"),(7,"7"),(8,"8"),(9,"9"),
                          (10,"10"),(10,"J"),(10,"Q"),(10, "K"),(11,"A")];

        let mut res:Vec<Card> = vec![];
        for s in &suits {
            for v in &values {
                let card = Card { value: v.0, name: s.to_string() + v.1};
                res.push(card);
            }
        }
        rand::thread_rng().shuffle(&mut res);
        res
    }
}


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
