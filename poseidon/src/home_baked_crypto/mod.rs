
/// A module for an algorithm (Grain LFSR) that generates round constants
/// It was cooked a lot by the ai and hasn't been tested or used anywhere
pub mod generate_constants;

/// the main refrence implementation that i have been working around
pub mod large_bit_field;

/// the constants for the refrence implementation were hard coded here
mod large_constants;

/// Probably the most important module to focus on since it will be our implementation over small field sizes
pub mod small_bit_field;
extern crate rand;
