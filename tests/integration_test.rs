fn add_two(x: i32) -> i32 {
    x + 2
}

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}
