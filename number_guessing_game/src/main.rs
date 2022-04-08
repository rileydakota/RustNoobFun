use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_RAND: i8 = 20;
//const MAX_GUESSES: i8 = 5;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Guess a number between 1 and {}!", MAX_RAND);
    let answer = rng.gen_range(0..MAX_RAND);
    loop {
        let mut player_answer = String::new();

        io::stdin()
            .read_line(&mut player_answer)
            .expect("failed to collect answer")
            .to_string();

        let player_number_answer: i8 = player_answer.trim().parse().unwrap();

        match player_number_answer.cmp(&answer) {
            Ordering::Equal => {
                println!("Correct! You win!");
                break;
            }
            Ordering::Greater => println!("Wrong! {} is too high!", player_number_answer),
            Ordering::Less => println!("Wrong! {} is too low!", player_number_answer),
        }
    }
}
