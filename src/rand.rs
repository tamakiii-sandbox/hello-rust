extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let secret_number = ::rand::thread_rng().gen_range(1, 101);
    let guessed_number : i32 = guess.trim().parse()
        .expect("Please type a number!");

    match guessed_number.cmp(&secret_number) {
        Ordering::Less => println!("Too small! {} < {}", guessed_number, secret_number),
        Ordering::Equal => println!("You win! {} = {}", guessed_number, secret_number),
        Ordering::Greater => println!("Too big! {} > {}", guessed_number, secret_number),
    }
}
