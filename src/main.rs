use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Pick a number between 1 and 100.");

    let winning_number = rand::thread_rng().gen_range(1..=100);
        
    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error: unable to read your guess.");

    let mut guess: u32 = guess.trim().parse().expect("Please type a number between 1 and 100.");

    println!("You guessed the number: {guess}");

    match guess.cmp(&winning_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Congratulations! You guessed the winning number"),
    }
}
