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

fn str_to_fr<const BYTES: usize, F: PrimeField>(s: &str) -> [u8; BYTES] {
    let int = BigUint::from_str_radix(s, 10).expect("Failed to parse string into `BigUint`!");
    println!("int: {:?}", int);
    let modulus = BigUint::from_str_radix(F::MODULUS.strip_prefix("0x").unwrap_or(F::MODULUS), 16)
        .expect("Failed to parse modulus into `BigUint`!");
    println!("modulus: {:?}", modulus);
    let element = int.modpow(&BigUint::from(1_u8), &modulus);
    println!("element: {:?}", element);
    let bytes_vec = element.to_bytes_le();
    let mut bytes = [0u8; BYTES];
    for (index, byte) in bytes_vec.iter().enumerate() {
        if index >= BYTES {
            panic!("Input was too large to fit into a `Fr`!");
        }
        bytes[index] = *byte;
    }
    bytes

    // let mut chunks = [0u64; CHUNKS];
    // println!("Converting element: {:?}", element);
    // for (index, chunk) in element.to_u64_digits().into_iter().enumerate() {
    //     println!("chunk: {:?}", chunk);
    //     if index >= CHUNKS {
    //         panic!("Input was too large to fit into a `Fr`!");
    //     }
    //     chunks[index] = chunk;
    // }
    // println!("Chunks: {:?}", chunks);
    // chunks
}
