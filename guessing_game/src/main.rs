use std::{cmp::Ordering, io::{self, Write}};
use rand::Rng;
use guessing_game::Guess;

mod guessing_game;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    
    loop {
        print!("Please input your guess: ");
        _ = io::stdout().flush();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess = Guess::new(match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        });

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
