extern crate rand;

use std::io;
// use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let secret_number = ::rand::thread_rng().gen_range(1, 101);
    let guessed_number : i32 = guess.trim().parse().unwrap();

    println!("You guessed: {}", guessed_number);
    println!("Secret number is: {}", secret_number);
}
