use card::*;
use deck;
use interface;

pub enum Winner {
    Player,
    Dealer,
    Draw,
}

pub struct Round {
    pub deck: Vec<Card>,
    pub player_cards: Vec<Card>,
    pub dealer_cards: Vec<Card>,
    pub player_cards_str: Vec<String>,
    pub dealer_cards_str: Vec<String>,
    pub finished: bool,
    pub winner: Winner,
}

pub fn new() -> Round {
    Round {
        player_cards: vec![],
        player_cards_str: vec![],
        dealer_cards_str: vec![],
        dealer_cards: vec![],
        deck: deck::new(),
        finished: false,
        winner: Winner::Draw,
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
        self.player_cards_str = self
            .player_cards
            .clone()
            .into_iter()
            .map(|c| c.name)
            .collect();
        self.dealer_cards_str = self
            .dealer_cards
            .clone()
            .into_iter()
            .map(|c| c.name)
            .collect();
    }

    pub fn player_turn(&mut self) {
        loop {
            if self.player_score() >= 21 {
                break;
            }
            println!(
                "Введите 1, если хотите взять еще одну карту"
            );
            println!("Нажмите Enter, чтобы передать ход дилеру");

            let input = interface::input();
            if input == String::from("1") {
                self.player_cards.push(self.deck.remove(0));
                self.cards_to_str();
                interface::show_round_info(&self);
            } else {
                break;
            }
        }
        if self.player_score() > 21 {
            self.finished = true
        }
    }

    pub fn dealer_turn(&mut self) {
        if self.finished {
            return;
        }
        loop {
            if self.dealer_score() >= 17 {
                break;
            }
            self.dealer_cards.push(self.deck.remove(0));
            if self.dealer_score() > 17 {
                break;
            }
        }
        self.cards_to_str();
        interface::show_round_info(&self);
    }

    pub fn set_winner(&mut self) {
        let player_score = self.player_score();
        let dealer_score = self.dealer_score();

        if player_score > 21 {
            self.winner = Winner::Dealer;
            return;
        }

        if dealer_score > 21 {
            self.winner = Winner::Player;
            return;
        }

        if player_score == dealer_score {
            self.winner = Winner::Draw;
            return;
        }

        if player_score > dealer_score {
            self.winner = Winner::Player;
        } else {
            self.winner = Winner::Dealer;
        }
    }
}
