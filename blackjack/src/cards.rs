use rand::seq::SliceRandom;
use rand::thread_rng;
use strum_macros::EnumIter; 
use strum::IntoEnumIterator;


#[derive(EnumIter)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

#[derive(EnumIter)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub struct Card {
    pub suit: Suit,
    pub value: Value
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: i8) -> Deck {
        let mut deck = Deck { cards: vec![] };

        for s in Suit::iter() {
            for v in Value::iter() {
                deck.cards.push(Card{
                    suit: s,
                    value: v
                })
            }
        }

        deck
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}



#[test]
fn check_deck_new() {
    let deck_1 = Deck::new(1);
    assert_eq!(deck_1.cards.len(), 52);

    let deck_2 = Deck::new(2);
    assert_eq!(deck_2.cards.len(), 104);
}

#[test]
fn check_deck_shuffle() {}
