use std::io;
use std::process;
use rand::seq::SliceRandom; 

#[derive(Debug)]
enum Winner {
  Human,
  Computer,
  Draw
}

const CHOICES: &'static [&str] = &["rock", "paper", "scissors"];

fn determine_winner(player_choice: &str, comp_choice: &str) -> Winner {
  if player_choice == "rock" && comp_choice == "scissors" {
    Winner::Human
  }
  else if player_choice == "rock" && comp_choice == "paper" {
    Winner::Computer
  }
  else if player_choice == "rock" && comp_choice == "rock" {
    Winner::Draw
  }
  else if player_choice == "paper" && comp_choice == "paper" {
    Winner::Draw
  }
  else if player_choice == "paper" && comp_choice == "scissors" {
    Winner::Computer
  }
  else if player_choice == "paper" && comp_choice == "rock" {
    Winner::Human
  }
  else if player_choice == "scissors" && comp_choice == "scissors" {
    Winner::Draw
  }
  else if player_choice == "scissors" && comp_choice == "paper" {
    Winner::Human
  }
  else {
    Winner::Computer
  }
}

fn clean_input(input: String) -> String {
  let new_string = input.trim().to_lowercase();
  new_string
}

fn is_valid_input(input: &str) -> bool {
  CHOICES.contains(&input)
}

fn main() {

    println!("Rock, Paper, or Scissors!");
    println!("Make your choice:");

    let mut player_choice = String::new();

    io::stdin().read_line(&mut player_choice).expect("Failed to read line.");

    let clean_player_choice = clean_input(player_choice);

    if !is_valid_input(&clean_player_choice) {
      println!("Invalid Choice {}, must choose Rock, Paper, or Scissors!", &clean_player_choice);
      process::exit(0);
    }

    println!("player answered: {}", &clean_player_choice);

    println!("computer choosing...");

    let computer_choice = CHOICES.choose(&mut rand::thread_rng()).expect("Error");

    println!("computer choose {}!", computer_choice);
    let winner:Winner = determine_winner(&clean_player_choice, computer_choice);
    //println!("{:#?}", winner);
    match winner {
      Winner::Computer => println!("Computer wins!"),
      Winner::Draw => println!("Match is a draw!"),
      Winner::Human => println!("You win!")
    }
}

#[test]
  fn check_determine_winner(){
    assert!(matches!(determine_winner("rock", "scissors"), Winner::Human));
    assert!(matches!(determine_winner("rock", "rock"), Winner::Draw));
    assert!(matches!(determine_winner("rock", "paper"), Winner::Computer));
    assert!(matches!(determine_winner("paper", "scissors"), Winner::Computer));
    assert!(matches!(determine_winner("paper", "rock"), Winner::Human));
    assert!(matches!(determine_winner("paper", "paper"), Winner::Draw));
    assert!(matches!(determine_winner("scissors", "scissors"), Winner::Draw));
    assert!(matches!(determine_winner("scissors", "rock"), Winner::Computer));
    assert!(matches!(determine_winner("scissors", "paper"), Winner::Human));
  }
#[test]
  fn check_clean_input(){
    let usr_input = "ROCK \r".to_string();
    assert_eq!(clean_input(usr_input), "rock")
  }
#[test]
  fn check_is_valid_input(){
    let good_usr_input = "rock";
    let bad_usr_input = "one million dollars";

    assert!(is_valid_input(good_usr_input));
    assert!(!is_valid_input(bad_usr_input));
  }

