extern crate rand;
use rand::Rng;
use std::io::{self, BufRead};

struct Card {
    value: i32,
    name: String
}

struct Round {
    player_name: String,
    player_score: i32,
    dealer_score: i32,
    deck:         Vec<Card>,
    player_cards: Vec<Card>,
    player_cards_str: Vec<String>,
    dealer_cards: Vec<Card>
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

    fn player_cards_to_str(&mut self, cards: Vec<Card>) {
        self.player_cards_str = cards.into_iter().map(|c| c.name).collect();
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


    let cards: Vec<Card> = vec![];
    round.player_cards_to_str(cards);

//    let player_cards = round.player_cards.clone();

    let pcards:Vec<String> = round.player_cards.into_iter().map(|c| c.name).collect();
    let dcards:Vec<String> = round.dealer_cards.into_iter().map(|c| c.name).collect();

    println!("player cards: {:?}", round.player_cards_str );
    println!("player cards: {:?}", pcards );
    println!("dealer cards: {:?}", dcards );

}



fn initial_round() -> Round {
    let s_slice: Vec<String> = vec![];
    Round {
        player_name: String::new(),
        player_score: 0,
        dealer_score: 0,
        player_cards: vec![],
        player_cards_str: s_slice,
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
    name
}


