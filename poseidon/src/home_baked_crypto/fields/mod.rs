use super::*;
use std::str::FromStr;

use ff::PrimeField;
use num_bigint::BigUint;
use num_traits::{cast::ToPrimitive, Num};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod large_field;
pub mod small_field;

pub enum FieldSize {
    Small,
    Large,
}

fn str_to_fr<const CHUNKS: usize, F: PrimeField>(s: &str) -> [u64; CHUNKS] {
    let int = BigUint::from_str_radix(s, 10).expect("Failed to parse string into `BigUint`!");
    let modulus = BigUint::from_str_radix(F::MODULUS.strip_prefix("0x").unwrap_or(F::MODULUS), 16)
        .expect("Failed to parse modulus into `BigUint`!");
    let element = int.modpow(&BigUint::from(1_u8), &modulus);

    let mut chunks = [0u64; CHUNKS];
    for (index, chunk) in element.to_u64_digits().into_iter().enumerate() {
        if index >= CHUNKS {
            panic!("Input was too large to fit into a `Fr`!");
        }
        chunks[index] = chunk;
    }
    chunks
}
