use rand::Rng;
use regex::Regex;
use std::borrow::Borrow;
use std::ops::Deref;
use thiserror::Error;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})";

#[derive(Error, Debug)]
#[error("passed string is invalid email address")]
pub struct InvalidEmailError;

#[derive(Debug, Clone)]
pub struct EmailString(String);

impl EmailString {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidEmailError> {
        let value = value.into();

        let email_regex = Regex::new(EMAIL_REGEX).unwrap();
        if !email_regex.is_match(&value) {
            return Err(InvalidEmailError);
        }

        Ok(Self(value))
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for EmailString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailString {
    type Error = InvalidEmailError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for EmailString {
    type Error = InvalidEmailError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub struct Random<T>([T; 3]);

impl<T> Random<T> {
    pub fn new(value_1: T, value_2: T, value_3: T) -> Self {
        Self([value_1, value_2, value_3])
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let value_number = rand::thread_rng().gen_range(0..3);
        &self.0[value_number]
    }
}

fn main() {
    let email = EmailString::new(String::from("hello"));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod email_string {
        use super::*;

        #[test]
        fn should_successfully_create_email_string_from_valid_string() {
            EmailString::new("nikmas@gmail.com").unwrap();
            EmailString::new(String::from("nikmas@gmail.com")).unwrap();
            EmailString::try_from("hello@ukr.net").unwrap();
            EmailString::try_from(String::from("hello@ukr.net")).unwrap();
        }

        #[test]
        fn should_fail_to_create_email_string_from_invalid_string() {
            EmailString::new("nikmas").unwrap_err();
            EmailString::new(String::from("nikmas")).unwrap_err();
            EmailString::try_from("hello").unwrap_err();
            EmailString::try_from(String::from("hello")).unwrap_err();
        }
    }

    mod random {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn should_return_one_of_three_values_randomly() {
            let random = Random::new("a", "b", "c");

            let mut got_values = HashSet::new();

            let mut index = 0;
            loop {
                let value = *random;
                got_values.insert(value);

                if got_values.len() == 3 {
                    break;
                }

                if index == 100_000_000 {
                    panic!("random access seems to work incorrectly");
                }

                index += 1;
            }
        }
    }
}
