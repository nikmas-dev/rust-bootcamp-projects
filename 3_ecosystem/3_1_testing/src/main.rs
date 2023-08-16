use rand::Rng;
use std::fmt::{write, Display, Formatter};
use std::{cmp::Ordering, env, io};

type Number = u32;

#[derive(Debug, PartialEq)]
enum GameResult {
    TooSmall,
    TooBig,
    Win,
}

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match play(secret_number, guess) {
            GameResult::TooSmall => println!("Too small!"),
            GameResult::TooBig => println!("Too big!"),
            GameResult::Win => println!("You won!"),
        }
    }
}

fn play(secret_number: Number, guess: Number) -> GameResult {
    match guess.cmp(&secret_number) {
        Ordering::Less => GameResult::TooSmall,
        Ordering::Greater => GameResult::TooBig,
        Ordering::Equal => GameResult::Win,
    }
}

fn get_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn get_guess_number() -> Option<u32> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(secret_number: Number, guess: Number, expected_result: GameResult) {
        let game_result = play(secret_number, guess);
        assert_eq!(game_result, expected_result);
    }

    #[test]
    fn guessed_number_is_too_small() {
        let secret_number = 10;
        let guess = 5;
        check(secret_number, guess, GameResult::TooSmall);
    }

    #[test]
    fn guessed_number_is_too_big() {
        let secret_number = 10;
        let guess = 15;
        check(secret_number, guess, GameResult::TooBig);
    }

    #[test]
    fn guessed_number_is_correct() {
        let secret_number = 10;
        let guess = 10;
        check(secret_number, guess, GameResult::Win);
    }
}
