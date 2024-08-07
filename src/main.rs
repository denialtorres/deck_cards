use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
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

        // implicit return
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    let cards = deck.deal(3);

    println!("Here is your hand: {cards:#?}");
}
