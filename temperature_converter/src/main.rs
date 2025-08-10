use std::io::{self, Write};

fn main() {
    println!("Welcome to temperature convertor");
    loop {
        let mut input = String::new();

        println!("Pick a mode: fahrenheit (fah) or celsius (cel)");
        print!(">> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("expected user input!");

        let input = input.trim();

        if input == "fahrenheit" || input == "fah" {
            let celsius = get_number("Type in temperature in celsius: ");
            let temp = celsius * 1.8 + 32f64;
            println!("celsius -> fahrenheit = {temp}");
        } else if input == "celsius" || input == "cel" {
            let fahrenheit = get_number("Type in temperature in fahrenheit: ");
            let temp = (fahrenheit - 32f64) / 1.8;
            println!("fahrenheit -> celsius = {temp}");
        } else if input == "quit" || input == "exit" {
            break;
        } else {
            println!("Invalid option: {input}");
        }
    }

    println!("Goodbye!");
}

fn get_number(message: &str) -> f64 {
    loop {
        let mut input = String::new();
        print!("{message}");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("expected user input!");

        let res: f64 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Please type in a number!");
                continue;
            },
        };

        return res;
    }
}
