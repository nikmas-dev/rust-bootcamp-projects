use argon2::Config;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use sha256::digest;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("charset cannot be empty")]
struct EmptyCharsetError;

type Password = String;

fn generate_password(length: usize, charset: &str) -> Result<Password, EmptyCharsetError> {
    let mut rng = rand::thread_rng();

    if charset.is_empty() {
        return Err(EmptyCharsetError);
    }

    Ok((0..length)
        .map(|_| charset.chars().choose(&mut rng).unwrap())
        .collect())
}

fn select_rand_value<T>(values: &[T]) -> Result<&T, EmptyCharsetError> {
    let mut rng = rand::thread_rng();

    if values.is_empty() {
        return Err(EmptyCharsetError);
    }

    Ok(values.choose(&mut rng).unwrap())
}

type AccessToken = String;

fn new_access_token() -> Result<AccessToken, Unspecified> {
    let charset = ('a'..='z')
        .chain('A'..='Z')
        .chain('0'..='9')
        .collect::<String>();

    const TOKEN_LENGTH: usize = 64;

    let rng = SystemRandom::new();
    let mut random_bytes = [0u8; TOKEN_LENGTH];
    rng.fill(&mut random_bytes)?;

    Ok(random_bytes
        .into_iter()
        .map(|byte| charset.chars().nth(byte as usize % charset.len()).unwrap())
        .collect())
}

type Hash = String;

fn get_file_hash(file_path: impl AsRef<Path>) -> io::Result<Hash> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(digest(buffer))
}

fn hash_password(password: &str, salt: &str) -> argon2::Result<Password> {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    mod generate_password {
        use super::*;

        #[test]
        fn returns_password_with_correct_length_and_symbols() {
            let length = 10;
            let charset = "abc";
            let password = generate_password(length, charset).unwrap();
            assert_eq!(password.len(), length);
            assert!(password.chars().all(|c| charset.contains(c)));
        }

        #[test]
        fn returns_error_when_charset_is_empty() {
            let length = 10;
            let charset = "";
            let result = generate_password(length, charset);
            result.unwrap_err();
        }
    }

    mod select_rand_value {
        use super::*;

        #[test]
        fn returns_random_value_from_slice() {
            let values = &[1, 2, 3];
            let value = select_rand_value(values).unwrap();
            assert!(values.contains(value));
        }

        #[test]
        fn returns_error_when_slice_is_empty() {
            let values: [u8; 0] = [];
            let result = select_rand_value(&values);
            result.unwrap_err();
        }
    }

    mod new_access_token {
        use super::*;

        #[test]
        fn returns_token_with_correct_length_and_symbols() {
            let token = new_access_token().unwrap();
            assert_eq!(token.len(), 64);
            assert!(token.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }

    mod get_file_hash {
        use super::*;

        #[test]
        fn successfully_returns_file_hash() {
            get_file_hash("Cargo.toml").unwrap();
        }
    }

    mod hash_password {
        use super::*;

        #[test]
        fn returns_correctly_hashed_password() {
            let password = "password";
            let salt = "simple_salt";
            let hashed_password = hash_password(password, salt).unwrap();
            assert!(argon2::verify_encoded(&hashed_password, password.as_bytes()).unwrap());
        }
    }
}
