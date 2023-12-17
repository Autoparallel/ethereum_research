use super::*;
use std::str::FromStr;

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod large_field;
pub mod small_field;

pub enum FieldSize {
    Small,
    Large,
}

fn str_to_fr<const CHUNKS: usize>(s: &str) -> [u64; CHUNKS] {
    let int =
        BigUint::parse_bytes(s.as_bytes(), 10).expect("Failed to parse string into `BigUint`!");

    let mut chunks = [0u64; CHUNKS];
    for (index, chunk) in int.to_u64_digits().into_iter().enumerate() {
        if index >= CHUNKS {
            panic!("Input was too large to fit into a `Fr`!");
        }
        chunks[index] = chunk;
    }
    chunks
}
