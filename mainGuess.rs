use std::io;
use rand::Rng; // Rng trait from the rand crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // gen_range is a method of the Rng trait

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line"); 

            
        let guess: u32 = guess.trim().parse() {
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
            }
        }
    }
}