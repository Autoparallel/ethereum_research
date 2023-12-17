use super::large_constants;
use ff::PrimeField;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(PrimeField)]
#[PrimeFieldModulus = "131071"]
#[PrimeFieldGenerator = "3"]
pub struct SmallField([u64; 1]);

impl FromStr for SmallField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = str_to_fr::<1>(s);
        Ok(SmallField(chunks))
    }
}

#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct LargeField([u64; 4]);

impl FromStr for SmallField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks = str_to_fr::<4>(s);
        Ok(SmallField(chunks))
    }
}

#[feature(constant_generics)]
#[allow(incomplete_features)]
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
    LargeField(chunks)
}
