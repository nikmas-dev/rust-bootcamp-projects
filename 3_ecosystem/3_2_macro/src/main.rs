#[macro_export]
macro_rules! btreemap {
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

fn main() {
    let map = btreemap!("hello" => 1, "world" => 2);
    println!("{:?}", map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_btreemap() {
        let map = btreemap!("hello" => 1, "world" => 2);

        assert_eq!(map.len(), 2);
        assert_eq!(map["hello"], 1);
        assert_eq!(map["world"], 2);
    }
}
