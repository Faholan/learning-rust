use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let mut guess = String::new();

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Enter your guess!");

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number too small !"),
            Ordering::Greater => println!("Number too big !"),
            Ordering::Equal => {
                println!("You guessed right !");
                break;
            }
        }
    }
}
