use std::io;

fn main() {
    println!("Pick a number between 1 and 100.");
    
    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error: unable to read your guess.");

    println!("You guessed the number: {guess}");
}
