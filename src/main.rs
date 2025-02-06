use rand::prelude::IndexedRandom;
use wordle::Correctness;

const ANSWERS: &str = include_str!("../answers.txt");
const DICTIONARY: &str = include_str!("../dictionary.txt");
const GUESS_LENGTH: usize = 5;

fn check(answer: &str, guess: &str) -> Vec<Correctness> {
    let mut result = Vec::new();
    for i in 0..GUESS_LENGTH {
        if answer.chars().nth(i) == guess.chars().nth(i) {
            result.push(Correctness::Correct(guess.chars().nth(i).unwrap()));
        } else if answer.contains(guess.chars().nth(i).unwrap()) {
            result.push(Correctness::Misplaced(guess.chars().nth(i).unwrap()));
        } else {
            result.push(Correctness::Incorrect(guess.chars().nth(i).unwrap()));
        }
    }
    result
}

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
        let result = check(&answer, &guess);
        for guess in result.iter() {
            print!("{}", guess.colored_string());
        }
        if result.iter().all(|x| {
            match x {
                Correctness::Correct(_) => {true},
                _ => false
            }
        }) {
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
