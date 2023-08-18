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
    let secret_number = env::args()
        .skip(1)
        .take(1)
        .last()
        .expect("No secret number is specified");
    secret_number
        .trim()
        .parse()
        .expect("Secret number is not a number")
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
    use proptest::prelude::*;

    use super::*;

    fn check(secret_number: Number, guess: Number, expected_result: GameResult) {
        let game_result = play(secret_number, guess);
        assert_eq!(game_result, expected_result);
    }

    prop_compose! {
        fn too_small_guessed_number()
            (secret_number in any::<Number>())
            (secret_number in Just(secret_number), guess in ..secret_number)
            -> (Number, Number) {
            (secret_number, guess)
        }

    }

    prop_compose! {
        fn too_big_guessed_number()
            (secret_number in any::<Number>())
            (secret_number in Just(secret_number), guess in (secret_number + 1)..)
            -> (Number, Number) {
            (secret_number, guess)
        }
    }

    proptest! {
        #[test]
        fn guessed_number_is_too_small((secret_number, guess) in too_small_guessed_number()) {
            check(secret_number, guess, GameResult::TooSmall);
        }

        #[test]
        fn guessed_number_is_too_big((secret_number, guess) in too_big_guessed_number()) {
            check(secret_number, guess, GameResult::TooBig);
        }

        #[test]
        fn guessed_number_is_correct(number: Number) {
            check(number, number, GameResult::Win);
        }

    }
}
