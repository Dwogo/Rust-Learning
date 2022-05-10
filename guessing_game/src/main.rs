// Imports
use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

// Functions
fn main() {
    
    println!("Hello there!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    let mut number_of_guesses = 1;

    loop {

        println!("Please input your guess.");

        // Rust vars immutible by default at mut to change it
        let mut guess = String::new();
   
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Has similar thing to Go where you can have a variable in between a string
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small!");
                number_of_guesses += 1;
            },
            Ordering::Greater => {
                println!("Too Big");
                number_of_guesses += 1;
            },
            Ordering::Equal => {
                println!("You Win!");
                println!("You guessed {} times", number_of_guesses);
                break;
            },
        }
    }
}
