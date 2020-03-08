extern crate rand;

use std::io;
// use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = ::rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        println!("Guess the number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("you guessed: {}", guess);

        let guessed_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("huh? {}", guess);
                continue;
            }
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
