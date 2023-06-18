// Use Library for inputs
use std::io;

fn main()
{
    // Std print 
    println!("Enter your Name: ");
    // Declaration of the variable
    // In rust, variables are immutable(unchangeable) by default
    let mut user_input = String::new();
    // Uses the library
    io::stdin()
        // reads the line and stores to the variable
        .read_line(&mut user_input)
        // Warning is generated to handle errors
        .expect("reading input failed");
    println!("Hi {},Welcome to the club!...",user_input);
}