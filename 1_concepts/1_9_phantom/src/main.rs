use rand::prelude::SliceRandom;
use std::marker::PhantomData;

pub trait GetRandomFact {
    fn get_random_fact() -> String;
}

pub struct Fact<T: GetRandomFact>(PhantomData<T>);

impl<T: GetRandomFact> Fact<T> {
    pub fn get() -> String {
        T::get_random_fact()
    }
}

impl<T> GetRandomFact for Vec<T> {
    fn get_random_fact() -> String {
        ["vec is heap-allocated", "vec may re-allocate on growing"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

fn main() {
    println!("Implement me!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn should_return_random_fact_about_vec() {
        let mut got_facts = HashSet::new();

        let mut i = 0;

        loop {
            let fact = Fact::<Vec<i32>>::get();
            got_facts.insert(fact);

            if got_facts.len() > 1 {
                break;
            }

            if i > 100_000_000 {
                panic!("random facts aren't random enough");
            }

            i += 1;
        }
    }
}
