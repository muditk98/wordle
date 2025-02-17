use colored::{ColoredString, Colorize};
use rand::prelude::IndexedRandom;
use wordle::Correctness;

const ANSWERS: &str = include_str!("../answers.txt");
const DICTIONARY: &str = include_str!("../dictionary.txt");
const GUESS_LENGTH: usize = 5;

fn play(answer: &str) {
    let dictionary: Vec<&str> = DICTIONARY.lines().collect();
    let mut tries = 5;
    while tries >= 0 {
        println!("\n{}>", tries);
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();
        if guess.len() != GUESS_LENGTH {
            println!("\nGuess should be 5 characters long.");
            continue;
        } else if !dictionary.contains(&guess.as_str()) {
            println!("\nNot a valid word");
            continue;
        }
        tries -= 1;
        let result = Correctness::check(&answer, &guess);
        for (guess, result) in guess.chars().zip(result) {
            let out = match result {
                Correctness::Correct => ColoredString::from(String::from(guess)).black().on_bright_green().bold(),
                Correctness::Misplaced => ColoredString::from(String::from(guess)).black().on_bright_yellow().bold(),
                Correctness::Incorrect => ColoredString::from(String::from(guess)).white().on_black().bold()
            };
            print!("{}", out);
        }
        if result.iter().all(|x| { *x == Correctness::Correct }) {
            println!("\nYou won!");
            return;
        }
    }
    println!("\nYou lost.");
    println!("Answer is {}", answer);
}

fn main() {
    let answers: Vec<&str> = ANSWERS.lines().collect();
    play(answers.choose(&mut rand::rng()).unwrap());
}
