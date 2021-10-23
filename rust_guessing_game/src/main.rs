// Importing libraries with particular traits
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};
fn main() {
    println!("Guess the number!");
    // Consuming the rand library by declaring a variable that is set to a random number between 1 & 0
    // The random number is within the thread of the OS. Gen_Range is to control the range.
    let secret_number = rand::thread_rng().gen_range(1..111);
    // Printing that number to test that rand works.
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Guessed The Secret Number!");
                break;
            }
        }
    }
}
