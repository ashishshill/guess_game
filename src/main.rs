use colored::*;
use rand::Rng;
// Ordering of the two things being compared to value
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, guess Game!");

    // thread_rng is an associated function that would give us random number generator
    // gen_range is a method that will produce a random number between the value i set
    let secret_number = rand::thread_rng().gen_range(0, 100);
    println!("Your guessed: {}", secret_number);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too Big".blue()),
        };
    }
}
