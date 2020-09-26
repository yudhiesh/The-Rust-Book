// Code to be used in the main functions
//pub fn vector_functions() {
//    //vectors::hello();
//    //vectors::display_vector_elements();
//    //vectors::add_access_elements();
//    //vectors::itr_over_values();
//    //vectors::itr_over_edit_values();
//}
//
//// Use the enum in the vectors file and then print out the values from there
//pub fn print_enum_of_diff_types() {
//    use vectors::*;
//    //vector_functions();
//    let data1 = Data::Text(String::from("Hello"));
//    let data2 = Data::Float(0.32);
//    let data3 = Data::Int(32);
//    //let v = vec![data1, data2, data3];
//    vectors::print_enum(Some(data1));
//    vectors::print_enum(Some(data2));
//    vectors::print_enum(Some(data3));
//}

pub fn hello() {
    println!("Hello")
}

pub fn display_vector_elements() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[3];
    // two methods to print the elements of a vector
    // causes the program to panic
    //print!("3rd element is {}", third);

    // deals with the index being out of bounds
    match v.get(5) {
        Some(value) => print!("3rd element is {}", value),
        None => print!("There is no 3rd element"),
    }
}

//pub fn add_access_elements() {
//    let v = vec![1, 2, 3, 4, 5];
//    let get_element = v.get(0);
//    v.push(2);
//
//    print!("1st element is {:?}", get_element);
//}

pub fn itr_over_values() {
    let v = vec![100, 23, 21];
    for i in &v {
        print!("{} ", i);
    }
}

pub fn itr_over_edit_values() {
    let mut v = vec![100, 23, 21];
    println!("Before: {:?}", v);
    for i in &mut v {
        // square the values
        *i *= *i;
    }
    println!("After: {:?}", v);
}

// Vectors can only store items of the same type
// Which is inconvenient for us
// You can instead use an enum which will store different types in a vector

#[derive(Debug)]
pub enum Data {
    Int(i32),
    Float(f32),
    Text(String),
}
impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Data::Int(v) => v.fmt(f),
            Data::Float(v) => v.fmt(f),
            Data::Text(v) => v.fmt(f),
        }
    }
}
pub fn print_enum(data: Option<Data>) {
    match data {
        Some(Data::Int(val)) => println!("{}", val),
        Some(Data::Text(val)) => println!("{}", val),
        Some(Data::Float(val)) => println!("{}", val),
        None => println!("No value to print"),
    }
}
