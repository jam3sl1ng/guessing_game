use std::io; // Standard input/output library

use rand::Rng; // Random number generation library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // The secret number to guess between 1 and 100

    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");
    
    let mut guess = String::new(); // Create a mutable String to hold the user's input

    // Read the input from the user
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guesses: {guess}"); // Print the user's guess
}
