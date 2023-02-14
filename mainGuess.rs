use std::io;
use rand::Rng; // Rng trait from the rand crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range is a method of the Rng trait

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() 
        .read_line(&mut guess) 
        .expect("Failed to read line"); 

    println!("You guessed: {guess}");

    println!("The secret number is: {secret_number}")
}