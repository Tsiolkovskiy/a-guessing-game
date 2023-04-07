#[allow(unused_imports)]
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess!");

    let mut guess = String::new(); //mutable variable

    io::stdin() //calling stdin function
        .read_line(&mut guess) //reading user input
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
