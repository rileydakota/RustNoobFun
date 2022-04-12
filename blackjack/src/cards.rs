use rand::seq::SliceRandom;
use rand::thread_rng;



pub enum Card {
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

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: i8) -> Deck {
        let mut deck = Deck { cards: vec![] };
        
        for x in 0..num_decks {
            for i in 0..4 {
                deck.cards.push(Card::Two);
                deck.cards.push(Card::Three);
                deck.cards.push(Card::Four);
                deck.cards.push(Card::Five);
                deck.cards.push(Card::Six);
                deck.cards.push(Card::Seven);
                deck.cards.push(Card::Eight);
                deck.cards.push(Card::Nine);
                deck.cards.push(Card::Ten);
                deck.cards.push(Card::Jack);
                deck.cards.push(Card::Queen);
                deck.cards.push(Card::King);
                deck.cards.push(Card::Ace);
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

pub fn to_val(card: &Card) -> i8 {
    match card {
        Card::Two => 2,
        Card::Three => 3,
        Card::Four => 4,
        Card::Five => 5,
        Card::Six => 6,
        Card::Seven => 7,
        Card::Eight => 8,
        Card::Nine => 9,
        Card::Ten => 10,
        Card::Jack => 10,
        Card::Queen => 10,
        Card::King => 10,
        Card::Ace => 11,
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
