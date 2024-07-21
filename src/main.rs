#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    // List of suits - 'hearts', 'spades
    // List of values - 'ace', 'two', 'three'

    // Double nested for loop
    //
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = [
        "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
        "Queen", "King",
    ];

    let mut cards = vec![];
    // double nested for loop
    //
    for suit in suits {
        for value in values {
            let card = format!("{value} of {suit}");

            cards.push(card);
        }
    }

    let deck = Deck { cards };

    println!("Here is your deck: {deck:?}");
}
