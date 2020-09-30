#![allow(non_snake_case)]
mod closures;
mod collections;
mod error_handling;
mod generics;
mod lib;

// Importing files from collections dir
pub use crate::collections::*;

// Importing files from error_handling dir
pub use crate::error_handling::*;

// Importing files from generic_types dir
pub use crate::generics::*;

// Importing traits from generic_types dir
pub use crate::traits::*;

// Importing traits from closures file
pub use crate::closures::anon_func::*;

//Importing traits from iterators file
pub use crate::closures::iterators::*;

fn main() {}
