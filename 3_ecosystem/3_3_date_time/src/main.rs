use chrono::{Datelike, NaiveDate};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("invalid date")]
pub struct InvalidDateError;

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

struct User {
    birthdate: NaiveDate,
}

impl User {
    fn with_birthdate(year: i32, month: u32, day: u32) -> Result<Self, InvalidDateError> {
        Ok(Self {
            birthdate: NaiveDate::from_ymd_opt(year, month, day).ok_or(InvalidDateError)?,
        })
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u16 {
        let now = NaiveDate::parse_from_str(NOW, "%Y-%m-%d").unwrap();

        if self.birthdate > now {
            return 0;
        }

        let age = (now.year() - self.birthdate.year()) as u16;

        if now.month() < self.birthdate.month()
            || (now.month() == self.birthdate.month() && now.day() < self.birthdate.day())
        {
            age - 1
        } else {
            age
        }
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d).unwrap();
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in
            vec![((2032, 6, 25), 0), ((3000, 6, 27), 0), ((9999, 6, 27), 0)]
        {
            let user = User::with_birthdate(y, m, d).unwrap();
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn check_is_adult() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), true),
            ((1990, 7, 4), true),
            ((0, 1, 1), true),
            ((1970, 1, 1), true),
            ((2019, 6, 25), false),
            ((2032, 6, 25), false),
            ((3000, 6, 27), false),
            ((9999, 6, 27), false),
        ] {
            let user = User::with_birthdate(y, m, d).unwrap();
            assert_eq!(user.is_adult(), expected);
        }
    }
}
