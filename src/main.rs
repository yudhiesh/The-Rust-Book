mod collections;
mod error_handling;
mod generics;

// Importing files from collections dir
pub use crate::collections::*;

// Importing files from error_handling dir
pub use crate::error_handling::*;

// Importing files from generic_types dir
pub use crate::generics::*;

fn main() {
    let p1 = generic_types::Point { x: 5, y: 0.2 };
    let p2 = generic_types::Point {
        x: "Bye",
        y: "Hello",
    };
    println!("p1 : {:#?}", p1);
    println!("p2 : {:#?}", p2);
    let result = p1.mixup(p2);
    println!("Mixup of is : {:#?}", result);
    println!("x : {:?} y : {:?}", result.x, result.y);
}
