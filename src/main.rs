use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pick a number between 1 and 100.");

    let winning_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");

        // Create a mutable variable to store the user's guess.
        let mut guess = String::new();

        // Create a mutable variable to store the number of guesses.
        let mut guessCount = 0;

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: unable to read your guess.");

        let mut guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        guessCount += 1;

        println!("Guess #{guessCount}: you guessed the number {guess}.");

        match guess.cmp(&winning_number) {
            Ordering::Less => println!("Your guess was too low."),
            Ordering::Greater => println!("Your guess was too high."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the winning number!");
                break;
            }
        }
    }
}
