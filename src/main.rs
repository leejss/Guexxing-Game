extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guexxing game start");

    // Generate random number in given range
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Inpu your guess");
        // Create string variable
        let mut guess = String::new();

        // Read line
        io::stdin()
            // Input is converted to String type and passed to a parameter
            .read_line(&mut guess)
            // read_line return Result type.
            // It has two variants, Ok and Err
            // when result is Ok, expect() returns value
            // when result is Err, expect() prints error message.
            .expect("Failed to read line.");
        // Ok value is assigned to guess variable
        // Use stirng method, trim() and parse()
        // parse() return Result type
        let guess: u32 = match guess.trim().parse() {
            // match expression
            Ok(num) => num,
            // Cannot parse
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        // Comparing two numbers using cmp()
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Smaller!"),
            Ordering::Less => println!("Greater!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
