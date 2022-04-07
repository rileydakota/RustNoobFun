use rand::Rng;
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

        match player_number_answer {
            a if a == answer => {
                println!("Correct! You win!");
                break;
            }
            a if a > answer => println!("Wrong! {} is too high!", a),
            a if a < answer => println!("Wrong! {} is too low!", a),
            _ => panic!("unknown error, exiting"),
        }
    }
}
