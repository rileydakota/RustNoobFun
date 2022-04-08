mod cards {
enum PlayerAction {
  Hit,
  Stand
}

enum Card {
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
  Ace
}

struct Deck {
  cards: Vec<Card>
}

pub fn to_val(card:Card){
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
    Card::Ace => 12
  }
}

pub fn create_deck(num_decks:i8) -> Deck {
  let mut deck = Deck::new();
  
  for x in num_decks {
    deck.cards.push(Card::Two * 4);
    deck.cards.push(Card::Three * 4);
  }
}


struct GameHand {
  cards: Vec
}
}