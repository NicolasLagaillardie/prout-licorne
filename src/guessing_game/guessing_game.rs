use rand::Rng;
use std::cmp::Ordering;
use std::io;

const CONST_SEED: u32 = 30;

fn shadowing() {
    let mut x = 5;

    println!("The first value of x is: {x}");

    x += 5;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The second value of x is: {x}");
}

pub(crate) fn main() {
    shadowing();

    println!("Guess the number!");

    let secrect_number = rand::rng().random_range(1..=100);

    println!("The secret number: {secrect_number}");

    println!("The const number: {CONST_SEED}");

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
