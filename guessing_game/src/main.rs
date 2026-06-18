// RngExt is a trait that provides methods for generating random numbers
use rand::RngExt;
// cmp is a trait that provides methods for comparing values
// Ordering is an enum that represents the result of a comparison
// Result is an enum that represents the result of a function that can fail or succeed
use std::cmp::Ordering;
// io is a module that provides input/output functionality
use std::io;

fn main() {
    println!("Guess the number!");
    // generating a random number between 1 and 100
    let secret_number = rand::rng().random_range(1..=100);
    // allowing users to guess multiple numbers using loop
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            // reading the user's input into the guess string
            .read_line(&mut guess)
            .expect("Failed to read line");
        // changing the guess string to a u32
        let guess: u32 = match guess.trim().parse() {
            // two cases: the parse succeeds, or it fails
            // if the parse succeeds, we return the parsed number
            // if it fails, we print an error message and return
            Ok(num) => num,
            Err(_) => {
                // guess.trim() is used to remove any leading or trailing whitespace from the guess string
                println!("{} is not a number. Please type a number", guess.trim());
                // we use `return` to exit the function early
                // return;
                // But, here we don't want to exit the function if user enters an invalid input
                // so we continue to the next iteration of the loop
                // It will go back to the start of the loop and prompt the user for a new guess
                continue;
            }
        };
        println!("You guessed: {}", guess);
        // comparing guess to the secret_number
        match guess.cmp(&secret_number) {
            // if the guess is less than the secret number, we print "To small"
            // if the guess is greater than the secret number, we print "To big"
            // if the guess is equal to the secret number, we print "You win!"
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            // Quitting after correct guess
            Ordering::Equal => {
                println!("You win!");
                // if the guess is equal to the secret number, we break out of the loop
                break;
            }
        }
        println!("The secret number is: {secret_number}");
    }
}
// the stdin function from the io module, which will allow us to handle user input
// read_line puts whatever the user enters into the string we pass to it