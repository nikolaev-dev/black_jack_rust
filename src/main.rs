extern crate rand;
use rand::Rng;
use std::io::{self, BufRead};

struct Card {
    value: i32,
    name: String
}

impl Clone for Card {
    fn clone(&self) -> Card {
        Card {
            value: self.value,
            name: self.name.clone()
        }
    }
}

struct Round {
    player_name: String,
    player_score: i32,
    dealer_score: i32,
    deck:         Vec<Card>,
    player_cards: Vec<Card>,
    dealer_cards: Vec<Card>,
    player_cards_str: Vec<String>,
    dealer_cards_str: Vec<String>
}

impl Round {

    fn score(&mut self) {
        self.player_score = self.player_cards.iter().map(|s| s.value).sum();
        self.dealer_score = self.dealer_cards.iter().map(|s| s.value).sum();
    }

    fn first_turn(&mut self) {
        self.player_cards.push(self.deck.remove(0));
        self.player_cards.push(self.deck.remove(0));
        self.dealer_cards.push(self.deck.remove(0));
        self.dealer_cards.push(self.deck.remove(0));

        self.score();
    }

    fn cards_to_str(&mut self) {
        self.player_cards_str = self.player_cards.clone().into_iter().map(|c| c.name).collect();
        self.dealer_cards_str = self.dealer_cards.clone().into_iter().map(|c| c.name).collect();
    }
}


fn main() {

    let mut round:Round = initial_round();
    println!("Введите имя игрока");
    round.player_name = text_input();

    round.first_turn();
    println!("player_name:  {}", round.player_name);
    println!("player_score: {}", round.player_score);
    println!("dealer_score: {}", round.dealer_score);


    round.cards_to_str();
    println!("player cards: {:?}", round.player_cards_str );
    println!("player cards: {:?}", round.dealer_cards_str );
}



fn initial_round() -> Round {
    Round {
        player_name: String::new(),
        player_score: 0,
        dealer_score: 0,
        player_cards: vec![],
        player_cards_str: vec![],
        dealer_cards_str: vec![],
        dealer_cards: vec![],
        deck: generate_cards()
    }
}

fn generate_cards() -> Vec<Card> {
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



fn text_input() -> String {
    let mut name = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut name).expect("Could not read line");
    name.pop();
    name
}


