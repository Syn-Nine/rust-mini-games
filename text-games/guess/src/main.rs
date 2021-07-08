// Guess the number game
//
// Based on the example in Rust language documentation:
//   https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut tries = 6;

    loop {
        println!("enter a guess, {} tries remaining", tries);

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too large"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }

        tries = tries - 1;
        if tries == 0 {
            println!("Game Over");
            break;
        }
    }
}
