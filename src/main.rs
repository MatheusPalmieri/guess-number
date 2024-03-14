use std::io;
use rand::Rng;
use std::cmp::Ordering;
use clearscreen::clear;

fn main() {
    clear().expect("Failed to clear screen");

    println!("Game: Guess the number between 1 and 100.\n");
    let mut guesses: u32 = 0;
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            },
            Ordering::Greater => println!("\nToo low!"),
            Ordering::Less => println!("\nToo high!"),
        }

        println!("You've used {} attempts!", guesses);
        println!("\n------------------\n")
    }
}
