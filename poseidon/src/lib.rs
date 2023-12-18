#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(incomplete_features)]
#![allow(unused_must_use)]
#![feature(associated_type_bounds)]
/// From scratch implementations of poseidon primitives for testing purposes.
use num_bigint::BigUint;
use num_traits::Zero;

pub mod home_baked_crypto;
pub mod implementations;
pub mod python_transcription;
#[macro_use]
extern crate ff;
