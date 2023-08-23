use btree_function_macro::btreemap;

#[test]
fn should_create_btreemap() {
    let map = btreemap!("hello" => 1, "world" => 2);

    assert_eq!(map.len(), 2);
    assert_eq!(map["hello"], 1);
    assert_eq!(map["world"], 2);
}
