mod cards;

struct AceError //raised when the value is an Ace - which can be a 1 or 11

pub enum PlayerAction {
    Hit,
    Stand,
}

enum HandState {
    Bust,
    Natural,
}


pub struct Player {
    hand: vec<cards::Card>
}

struct DealerHand {
    facedown_card: cards::Card
    cards: vec<cards:Card>
}

pub struct Dealer {
    hand: DealerHand
}

impl Dealer {

    pub fn new (mut &self) -> Dealer {
       
    }
    pub fn deal(mut &self, players: mut &vec<Player>, deck: mut &cards::Deck){

    }
}

pub fn to_val(value: &Value) -> result() {
    match value {
        Value::Two => 2,
        Value::Three => 3,
        Value::Four => 4,
        Value::Five => 5,
        Value::Six => 6,
        Value::Seven => 7,
        Value::Eight => 8,
        Value::Nine => 9,
        Value::Ten => 10,
        Value::Jack => 10,
        Value::Queen => 10,
        Value::King => 10,
        Value::Ace => 11,
    }
}

