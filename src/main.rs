extern crate rand;

mod card;
mod deck;
mod round;
mod game;
mod interface;
use game::*;
use round::*;

fn main() {
    println!("Введите имя игрока");
    let name = interface::input();
    let mut game = Game {
        player_name: name,
        player_balance: 100,
        dealer_balance: 100
    };
    println!("player_name:  {}", game.player_name);

    loop {
        let mut round = round::new();

        round.init_turn();
        interface::show_round_info(&round);
        round.player_turn();
        round.dealer_turn();
        round.set_winner();
        pay_win(&mut game, &round);
        interface::show_game_info(&game);
    }
}

fn pay_win(game: &mut Game, round: &Round) {
    match round.winner {
        Winner::Draw => {
            println!("Ничья")
        },
        Winner::Dealer => {
            println!("Дилер выиграл");
            game.dealer_balance += 10;
            game.player_balance -= 10;
        },
        Winner::Player => {
            println!("Игрок {} выиграл", game.player_name);
            game.dealer_balance -= 10;
            game.player_balance += 10;
        }
    }

}

