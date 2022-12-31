use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pick a number between 1 and 100.");

    let winning_number = rand::thread_rng().gen_range(1..=100);

    // Create a mutable variable to store the number of guesses.
    let mut guess_count = 0;

    // Create a mutable variable to store the maximum number of guesses.
    let max_guesses: i32 = 10;

    loop {
        if guess_count == max_guesses {
            println!("You have run out of guesses. The winning number was {winning_number}.");
            break;
        }

        println!("Please enter your guess:");

        // Create a mutable variable to store the user's guess.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: unable to read your guess.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please try again.");
                continue;
            }
        };

        guess_count += 1;

        println!("Guess {guess_count} of {max_guesses}. You guessed the number {guess}.");

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
