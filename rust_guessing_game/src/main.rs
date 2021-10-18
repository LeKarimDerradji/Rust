// Importing libraries with particular traits
use std::io::{self};
use rand::Rng;
fn main() {
    println!("Guess the number!");
    // Consuming the rand library by declaring a variable that is set to a random number between 1 & 0 
    // The random number is within the thread of the OS. Gen_Range is to control the range. 
    let secret_number = rand::thread_rng().gen_range(1..111);
    // Printing that number to test that rand works. 
    println!("The secret number is {}", secret_number);

    println!("Please input your guess: ");

    let mut guess= String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
