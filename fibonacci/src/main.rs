use std::io::{self, Write};

fn main() {
    println!("Welcome to fibonacci calculator");
    loop {
        let mut input = String::new();

        println!("Type a number or exit");
        print!(">> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("expected user input!");

        let input = input.trim();

        let number: u16 = match input.parse() {
            Ok(v) => v,
            Err(_) => {
                if input == "quit" || input == "exit" {
                    break;
                } else {
                    println!("Invalid input: {input}");
                }
                continue;
            },
        };

        println!("Fibonacci of {number} is: {:.0}", nth_fibonacci(number));
    }

    println!("Goodbye!");
}

fn nth_fibonacci(n: u16) -> f64 {
    if n <= 1 {
        return 1f64;
    }

    let mut current: f64 = 0f64;
    let mut previous1: f64 = 1f64;
    let mut previous2: f64 = 0f64;

    for _ in 2..=n {
        current = previous1 + previous2;
        previous2 = previous1;
        previous1 = current;
    }

    return current;
}
