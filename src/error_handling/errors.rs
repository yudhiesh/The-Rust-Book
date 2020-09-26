// Recoverable errors: Exp. file not found where the user should be notified that the file does not
// exist
// Result<T,E>

// Unrecoverable errors: Exp. accessing a location in an array that is beyond the array end
// panic!

// When a program panics it walks back up the stack and then it will clean up the stack which is a
// lot of work
// You can set the panic to abort the entire system and then the OS will do the cleaning up
// Do this when you need a very small project binary file
// RUST_BACKTRACE is used when you want to see the process of cleaning up the stack
// When returning a Result<> you always need to check for the errors
// If you call a function that returns a Result<> with unwrap() it will panic
// Panicking has its own use but is generally not advisable
// If the error makes sense to panic and end the program then its fine
// Else it would be better to handle the error and then do something with it

use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

pub fn read_file(file_path: String) {
    let file = File::open(file_path);
    let file = match file {
        Ok(f) => f,
        // Err here will panic no matter what error is found
        Err(error) => panic!("Unable to open the file {:?}", error),
    };
}

// IO errors have multiple types
// Using the ErrorKind we can get all the errors and take actions on them

pub fn read_file_error(file_path: String) {
    let file = File::open(&file_path);
    let file = match file {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// Better way to handle errors using if and else with closures

pub fn better_read_file_error(file_path: String) {
    let file = File::open(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_path).unwrap_or_else(|error| {
                panic!("Unable to create the file: {:?}", error);
            })
        } else {
            panic!("Unable to open the file: {:?}", error);
        }
    });
}

// Need to handle the error when calling the function
// Quick way is to panic using unwrap()

pub fn even_better_file_errors(file_path: String) -> Result<String, std::io::Error> {
    let f = File::open(&file_path);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // read_to_string() comes from the Read crate
    // it reads the contents of the file and returns a Result<_,_>

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn shorter_way_error(file_path: String) -> Result<String, std::io::Error> {
    // Error could occur when opening the file
    // Shorter syntax
    // let mut s = String::new();
    let mut s = String::new();
    File::open(&file_path)?.read_to_string(&mut s)?;
    // let mut f = File::open(&file_path)?;
    // Error could occur when reading the strings in the file
    //f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn shortest_way(file_path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
