#![allow(clippy::too_many_arguments)]
/// From scratch implementations of poseidon primitives for testing purposes.
use num_bigint::BigUint;
use num_traits::Zero;

pub mod home_baked_crypto;
pub mod implemenations;
pub mod python_transcription;
extern crate ff;

// const FIELD_SIZE: u32 = 8;
const STATE_SIZE: usize = 2;
const RATE: usize = STATE_SIZE - 1;
const CAPACITY: usize = 1;
// const ROUNDS: usize = 8;
use lazy_static::lazy_static;

lazy_static! {
    static ref FIELD_MODULUS: BigUint = BigUint::from(31u32);
}

fn permute(state: &mut [BigUint]) {
    let mut new_state = BigUint::zero();
    for element in state.iter() {
        new_state += element;
    }
    new_state %= &*FIELD_MODULUS;
    for element in state.iter_mut() {
        *element = new_state.clone();
    }
}

fn absorb(state: &mut [BigUint], input: &[BigUint]) {
    for block in input.chunks(RATE) {
        for (element, input) in state.iter_mut().zip(block) {
            *element += input;
            *element %= &*FIELD_MODULUS;
        }
        permute(state);
    }
}

fn squeeze(state: &[BigUint], output_len: usize) -> Vec<BigUint> {
    let mut output = Vec::new();
    while output.len() < output_len {
        output.extend_from_slice(&state[..RATE]);
        permute(&mut state.to_owned());
    }
    output
}

pub fn poseidon_hash(input: &[BigUint]) -> BigUint {
    let mut state = vec![BigUint::zero(); STATE_SIZE];
    absorb(&mut state, input);
    let output = squeeze(&state, CAPACITY);
    output[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        println!("field modulus: {}", *FIELD_MODULUS);
        let input = vec![
            BigUint::from(1u32),
            BigUint::from(3u32),
            BigUint::from(3u32),
        ];
        println!("input: {:?}", input);
        let output = poseidon_hash(&input);
        println!("output: {}", output);
    }
}
