use std::cmp::Ordering;
use std::io;
use std::process::exit;

use rand::Rng;

fn handle_error<T>(err: io::Error) -> T {
    eprintln!("error: {}", err);
    exit(1);
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    let mut guess = String::new();
    loop {
        println!("Please input your gues: ");

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .unwrap_or_else(handle_error);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
