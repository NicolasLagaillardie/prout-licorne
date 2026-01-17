use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub(crate) fn main() {
    println!("Guess the number!");

    let secrect_number = rand::rng().random_range(1..=100);

    println!("The secret number: {secrect_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let cast: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, this is not a number");
                continue;
            }
        };

        println!("You guess: {guess}");

        match cast.cmp(&secrect_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
