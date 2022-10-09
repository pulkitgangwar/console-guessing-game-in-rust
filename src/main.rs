#![allow(non_snake_case)]

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let randomNumber = rand::thread_rng().gen_range(1..=100);
    let mut numberOfTries: u64 = 0;
    loop {
        let mut guess = String::new();

        println!("{}", randomNumber);
        println!("Guess a number between 1 to 100");
        io::stdin()
            .read_line(&mut guess)
            .expect("invalid guess provided");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&randomNumber) {
            Ordering::Less => {
                println!("{}", "Less".red());
                numberOfTries += 1;
            }
            Ordering::Greater => {
                println!("{}", "Greater".red());
                numberOfTries += 1;
            }
            Ordering::Equal => {
                println!("{}", "You won!".green());
                println!("{}: {} attempt(s)", "You took ".green(), numberOfTries);
                break;
            }
        }
    }
}
