extern crate rand;

/// Contains the implementation of a `small_field` and `large_field` which are the two main types of fields we will be using
/// * `small_field` is a field with modulus "131071"
/// * `large_field` is a field with modulus "21888242871839275222246405745257275088548364400416034343698204186575808495617"
pub mod fields;

/// A module for handling constants for the hash function that are output as field elements.
/// Contains:
/// * An algorithm (Grain LFSR) that generates round constants (TODO: needs testing)
/// * A list of constants for the `large_field` implementation (TODO: add where they were take from)
pub mod constants;

/// A module for handling the hash function itself.
/// Contains:
/// * The `Poseidon` struct which is the main structure for our program. Generic over fields.
pub mod hasher;

use ff::PrimeField;
