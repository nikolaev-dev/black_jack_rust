use card::*;
use rand::Rng;

pub fn new() -> Vec<Card> {
    let suits = vec!["♠", "♥", "♣", "♦"];
    let values = vec![
        (2, "2"),
        (3, "3"),
        (4, "4"),
        (5, "5"),
        (6, "6"),
        (7, "7"),
        (8, "8"),
        (9, "9"),
        (10, "10"),
        (10, "J"),
        (10, "Q"),
        (10, "K"),
        (11, "A"),
    ];

    let mut res: Vec<Card> = vec![];
    for s in &suits {
        for v in &values {
            let card = Card {
                value: v.0,
                name: s.to_string() + v.1,
            };
            res.push(card);
        }
    }
    rand::thread_rng().shuffle(&mut res);
    res
}
