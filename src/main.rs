mod collections;
mod error_handling;
mod generics;

// Importing files from collections dir
pub use crate::collections::*;

// Importing files from error_handling dir
pub use crate::error_handling::*;

// Importing files from generic_types dir
pub use crate::generics::*;

// Importing traits from generic_types dir
pub use crate::traits::*;

fn main() {}
