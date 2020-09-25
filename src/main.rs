mod collections;

pub use crate::collections::*;

pub fn hash_map_functions() {
    //hash_map::create_hash_map();
    //hash_map::ownership_hash_map();
    //hash_map::access_values_hash_map();
    //hash_map::insert_if_key_empty();
    hash_map::update_value_hash_map();
}

pub fn string_functions() {
    //strings::print_str1();
    //strings::print_str2();
    //strings::print_format();
    strings::slice_strings();
}

pub fn vector_functions() {
    //vectors::hello();
    //vectors::display_vector_elements();
    //vectors::add_access_elements();
    //vectors::itr_over_values();
    //vectors::itr_over_edit_values();
}

// Use the enum in the vectors file and then print out the values from there
pub fn print_enum_of_diff_types() {
    use vectors::*;
    //vector_functions();
    let data1 = Data::Text(String::from("Hello"));
    let data2 = Data::Float(0.32);
    let data3 = Data::Int(32);
    //let v = vec![data1, data2, data3];
    vectors::print_enum(Some(data1));
    vectors::print_enum(Some(data2));
    vectors::print_enum(Some(data3));
    //for i in v {
    //    println!("{:?}", i);
    //}
}

fn main() {
    hash_map_functions();
}
