// Imports
use std::io::stdin;

// Functions
fn main() {
    
    println!("Hello there!");
    println!("Please input your guess.");
    
    // Rust vars immutible by default at mut to change it
    let mut guess = String::new();
   
    // Receive user input
    stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Has similar thing to Go where you can have a variable in between a string
    println!("You guessed: {}", guess);
}
