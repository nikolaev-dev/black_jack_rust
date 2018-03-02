extern crate rand;
use rand::Rng;

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
    dealer_cards: Vec<Card>
}

impl Round {
    fn first_turn(&mut self) {
        self.player_score = 10;
        self.player_cards.push(self.deck.remove(0));
        println!("{}", self.player_cards[0].name);
    }
}


fn main() {

    let mut round:Round = initial_round();
    println!("Hello, world!");
    println!("{:?}", round.player_name);
    round.first_turn();
    println!("player_score: {}", round.player_score);
}



fn initial_round() -> Round {
    Round {
        player_name: "Alexander".to_string(),
        player_score: 0,
        dealer_score: 0,
        player_cards: vec![],
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
