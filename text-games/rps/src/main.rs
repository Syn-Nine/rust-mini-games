// Rock-Paper-Scissors Game
//
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut p_score = 0;
    let mut c_score = 0;
    let mut tries = 5;
    let max = tries;

    println!("Let's play Rock-Paper-Scissors!");

    let options = [
        "Rock".to_string(),
        "Paper".to_string(),
        "Scissors".to_string(),
    ];
    let success = [2, 0, 1];

    while tries > 0 {
        println!("Best out of {}, {} tries remaining. What is your guess? [r]ock, [p]aper, or [s]cissors?", max, tries);

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess = match guess.as_bytes()[0] {
            b'r' => 0,
            b'p' => 1,
            b's' => 2,
            _ => continue,
        };

        let comp = rand::thread_rng().gen_range(0..3);
        println!("Computer: {}", options[comp]);
        println!("Player: {}", options[guess]);

        // 0 = Player
        // 1 = Computer
        // 2 = Tie
        let mut win = 2; // tie by default

        if success[guess] == comp {
            win = 0;
        } else if comp != guess {
            win = 1;
        }

        tries = tries - 1;
        match win {
            // Player Scores
            0 => {
                println!("Player Score!");
                p_score = p_score + 1;
            }

            // Computer Scores
            1 => {
                println!("Computer Score!");
                c_score = c_score + 1;
            }

            // Tie
            _ => {
                println!("Tie!");
                tries = tries + 1;
            }
        }

        println!("Score: P: {}, C: {}\n", p_score, c_score);
    }

    match p_score.cmp(&c_score) {
        Ordering::Greater => println!("PLAYER WINS GAME!"),
        Ordering::Less => println!("COMPUTER WINS GAME!"),
        Ordering::Equal => println!("GAME TIED!"),
    }
}
