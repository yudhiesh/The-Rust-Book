// String type in Rust is growable and mutable
// Rust does not allow for indexing into a string
// 1. Different languages have different sizes even for the same length of a word
// 2. Indexing operations in Rust are expected to be of O(1) which is not possible when you need to
//    iterate through the string up until the index if it even exist
// Rust offers a more specific manner which is slicing the strings
//

pub fn print_str1() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

pub fn print_str2() {
    // can only combine String and &str
    // s1 = String
    // s2 = &str
    let s1 = String::from("Hello");
    let s2 = "World";
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);
}

pub fn print_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format here returns a String with the output
    let s = format!("{}--{}--{}", s1, s2, s3);
    println!("{}", s);
}

pub fn slice_strings() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);
}
