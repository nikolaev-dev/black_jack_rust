use card::*;
use deck;
use interface;

pub struct Round {
    pub deck:         Vec<Card>,
    pub player_cards: Vec<Card>,
    pub dealer_cards: Vec<Card>,
    pub player_cards_str: Vec<String>,
    pub dealer_cards_str: Vec<String>
}

pub fn new() -> Round {
    Round {
        player_cards: vec![],
        player_cards_str: vec![],
        dealer_cards_str: vec![],
        dealer_cards: vec![],
        deck: deck::new()
    }
}

impl Round {

    pub fn player_score(&self) -> i32 {
        self.player_cards.iter().map(|s| s.value).sum()
    }

    pub fn dealer_score(&self) -> i32 {
        self.dealer_cards.iter().map(|s| s.value).sum()
    }

    pub fn init_turn(&mut self) {
        self.player_cards.push(self.deck.remove(0));
        self.player_cards.push(self.deck.remove(0));
        self.dealer_cards.push(self.deck.remove(0));
        self.dealer_cards.push(self.deck.remove(0));
        self.cards_to_str();
    }

    pub fn cards_to_str(&mut self) {
        self.player_cards_str = self.player_cards.clone().into_iter().map(|c| c.name).collect();
        self.dealer_cards_str = self.dealer_cards.clone().into_iter().map(|c| c.name).collect();
    }

    pub fn player_turn(&mut self) {
        println!("Введите 1, если хотите взять еще одну карту");
        println!("Нажмите Enter, чтобы пропустить");

        let input = interface::input();
        if input == String::from("1") {
            self.player_cards.push(self.deck.remove(0));
        }
        self.cards_to_str();
    }

}

