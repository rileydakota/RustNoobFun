mod cards;
use std::io;


type Hand=Vec<cards::Card>;

pub enum PlayerAction {
    Hit,
    Stand,
}

fn is_valid_input(input:&String)->bool{

  let input_clean = input.trim().to_uppercase();
  let input_trim = input_clean.as_str();
  
  match input_trim {
    "H" => true,
    "S" => true,
    _ => false
  }
}

fn get_hand_value(hand:&Hand)->i8{
  let mut total = 0;
  for card in hand {
    total += cards::to_val(&card);
  }
  total
}

fn display_hand_value(hand:&Hand)->String{

  let vals = hand
    .iter()
    .map(|x| cards::to_val(x))
    .map(|x| x.to_string())
    .collect::<Vec<String>>();

  let output = vals.join(", ");
  "Your hand: ".to_owned() + &output
  
}

fn display_dealer_hand(){}


fn main() {
    loop {
      println!("Lets play 21!");

      let mut deck = cards::Deck::new(4);
      deck.shuffle();

      let mut player_cards:Hand = vec![];
      let mut dealer_cards:Hand = vec![];

      for x in 0..2 {
        player_cards.push(deck.draw())
      }

      let dealer_facedown_card = deck.draw();
      dealer_cards.push(deck.draw());

      println!("dealer showing a {}", cards::to_val(&dealer_cards[0]));
      
      //let sum = get_hand_value(&player_cards);
      println!("{}", display_hand_value(&player_cards));
      println!("(H)it or (S)tand?");
      let mut player_input=String::new();

      io::stdin()
        .read_line(&mut player_input)
        .expect("failed to collect answer")
        .to_string();

      let mut player_input_valid = is_valid_input(&player_input);
      
      while !player_input_valid {
        println!("Invalid Answer! (H)it or (S)?");
        io::stdin()
        .read_line(&mut player_input)
        .expect("failed to collect answer")
        .to_string();
        player_input_valid = is_valid_input(&player_input);
      }
      
      
    };
    
}
