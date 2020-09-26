use std::collections::HashMap;

// Code for the main function
//pub fn hash_map_functions() {
//    //hash_map::create_hash_map();
//    //hash_map::ownership_hash_map();
//    //hash_map::access_values_hash_map();
//    //hash_map::insert_if_key_empty();
//    hash_map::update_value_hash_map();
//}

pub fn create_hash_map() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![50, 20];

    // zip over both vecs and collect their values into a hash map
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // {:#?} prints contents in a human readable form
    print!("{:#?}", scores);
}

pub fn ownership_hash_map() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    print!("{:#?}", map);
    // we cannot use the inserted values as they are borrowed
    //print!("{} {}", field_value, field_name);
}

pub fn access_values_hash_map() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let team_name_fake = String::from("Green");
    let score = scores.get(&team_name);
    // score returns a Some(&10)
    // because get() returns an Option<&V>
    // use unwrap() to get the specific value and panic if the value does not exist
    // but panicking is not ideal as it crashes the program
    // ideally the program should use a match
    //println!("{:?}", score.unwrap());
    for (key, value) in &scores {
        print!("{} {} ", key, value);
    }
}

pub fn insert_if_key_empty() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // entry() takes the key of the value you want to check as a parameter
    // if the key is has an empty value then it will
    scores.entry(String::from("Yellow")).or_insert(150);
    scores.entry(String::from("Blue")).or_insert(150);
    scores.entry(String::from("Green")).or_insert(100);
    scores.entry(String::from("Green")).or_insert(10);
    println!("{:?}", scores);
}

pub fn update_value_hash_map() {
    let text = "hello world wonderful world what are you doing hee";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

// Adding values to the same key will update the value

//fn match_hash_map(x: Option<i32>) -> Option<i32> {
//    match x {
//        None => None,
//        Some(i) => Some(i),
//    }
//}
