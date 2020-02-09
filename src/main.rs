use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Please input your gess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number);
}
