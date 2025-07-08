use std::cmp::Ordering; // For comparing values
use std::io; // Standard input/output library

use rand::Rng; // Random number generation library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // The secret number to guess between 1 and 100

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess: ");
        
        let mut guess = String::new(); // Create a mutable String to hold the user's input

        // Read the input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please enter a number!"); // Parse the input into a u32 (number)

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If the guess is less than the secret number
            Ordering::Greater => println!("Too big!"), // If the guess is greater than the secret number
            Ordering::Equal => { 
                println!("You win!"); // If the guess is equal to the secret number
                break;
            }
        }
    }
}
