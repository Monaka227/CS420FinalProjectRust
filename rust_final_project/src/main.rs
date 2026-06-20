// extern create rand; <- this is old version(before 2015 edition), don't need it now


use rand::Rng;
use std::cmp::Ordering;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};

fn main() {
    println!("=================================");
    println!("   Welcome to Rust Guessing Game!");
    println!("=================================");

    // input player name
    print!("Enter your name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim();

    // choose difficulty
    println!("\nSelect Difficulty:");
    println!("1. Easy   (1-50,  Max 10 guesses)");
    println!("2. Medium (1-100, Max 7 guesses)");
    println!("3. Hard   (1-500, Max 5 guesses)");

    let (max_range, max_attempts) = loop {
        print!("Choose 1, 2, or 3: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        
        match choice.trim() {
            "1" => break (50, 10),
            "2" => break (100, 7),
            "3" => break (500, 5),
            _ => {
                println!("Invalid choice. Please type 1, 2, or 3.");
                continue;
            }
        }
    };

    // generate random secret number
    let secret_number = rand::thread_rng().gen_range(1..=max_range);   //(1, 101);
    let mut attempts = 0;
    println!("The secret number is: {}", secret_number);
    println!("\nGame Started! Guess a number between 1 and {}.", max_range);
    println!("You have {} attempts. Good luck, {}!\n", max_attempts, username);

    loop{
        attempts += 1;
        if attempts > max_attempts {
            println!("💥 Game Over! You ran out of guesses. The number was {}.", secret_number);
            break;
        }

        print!("Attempt [{}/{}]. Enter your guess: ", attempts, max_attempts);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                attempts -= 1; // don't count invalid input
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🎉 You win, {}! You guessed it in {} attempts.", username, attempts);
                // save high score in file  (show Rust file I/O)
                let score_data = format!("Player: {}, Attempts: {}/{}\n", username, attempts, max_attempts);
                let file_result = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("high_scores.txt");

                match file_result {
                    Ok(mut file) => {
                        if file.write_all(score_data.as_bytes()).is_ok() {
                            println!("💾 Your score has been saved to 'high_scores.txt'!");
                        }
                    }
                    Err(_) => println!("Warning: Could not save your score to file."),
                }
                break;
            }
        }
        println!("---------------------------------");
    }
    // show previous result
    println!("\n--- [Past High Scores] ---");
    if let Ok(scores) = fs::read_to_string("high_scores.txt") {
        print!("{}", scores);
    } else {
        println!("No high scores recorded yet.");
    }
}