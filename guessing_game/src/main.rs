// To obetain the user input and then print the result as 
// output, we need to bring the "io" (input/output)
// library into scope.
use std::io; // The "io" library comes from the STanDard library ("std")

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    //  "let mut guess" will introduce a mutable variable named guess
    // After the equal sign, everything is the value that guess is bound to
    // which is the result of calling String::new, a function that returns
    // a new instance of a String. 

    // The :: synthax in the ::new line indicates that new is an 
    // associated function of the String type. 
    // An associated function is implemented on a type, 
    // in this case, String. 
    // Rather than on a particular instance of a String. 

    // The "new" function creates a new, empty String. 
    // To sumarize, the "let mut guess = String::new()"
    // line, creates a mutable variable that is currently bound 
    // to a new, empty instance of a String.
    let mut guess = String::new();

    // The "stdin" function returns an instance of "std::io::Stdin",
    // which is a type that represents a handle to the standard input 
    // for your terminal.
    io::stdin()

    // ".read_line(&mut guess)" calls the "read_line" method on the standard input
    // handle to get input from the user. 
    // We're also passing one argument to "read_line: &mut guess"
    // "read_line" takes whatever the user types into standard input and 
    // append that into a string (wihout overwriting its content)
    // The string argument has to be mutable so the method can change the 
    // string's content by adding the user input. 
        .read_line(&mut guess)

    // Here we're handling potential failure with the Result Type
    // ".read_line" puts what the user types into the string we're passing
    // it. But it also returns a value, in this case, an "io::Result".
    // The "Result" types are enums. For Result, the variants are "Ok" or "Err"
    // The purpose of Results is to encode error-handling information. 
    // Values of the Result type, like values of any type, have methods:
    // An instance of io::Result had an "expect method" that you can call.

    // If the value passed in read_line returns an Err, the program will crash
    // and it will returns the value passed in parameters. 
        .expect("Failed to read line");

    // Here we print the value entered by the user, using a {} placeholder
    // The placeholder returns the value entered in order, after the comma (second parameters) 
    println!("You guessed : {}", guess);
}
