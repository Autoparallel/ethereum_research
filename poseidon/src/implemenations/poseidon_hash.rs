// // extern crate pairing;
// extern crate rand;

// // use pairing::bn256::{Bn256, Fr};
// use poseidon_hash::bn256::Bn256PoseidonParams;
// use poseidon_hash::group_hash::BlakeHasher;
// use poseidon_hash::poseidon_hash;
// use rand::{thread_rng, Rand};

// pub fn poseidon_hash_rate_2_80_bits(input: &[Fr]) -> Fr {
//     let rate = 2u32;
//     let capacity = 1u32;
//     let num_full = 8u32;
//     let num_partial = 33u32;
//     let security_level = 80u32;
//     let params = Bn256PoseidonParams::new_for_params::<BlakeHasher>(
//         capacity,
//         rate,
//         num_partial,
//         num_full,
//         security_level,
//     );
//     let mut hash = poseidon_hash::<Bn256>(&params, input);
//     println!("hash: {:?}", hash);
//     hash.pop().unwrap()
// }

// this test will call the above function on some implementations

// #[test]
// fn call_poseidon_hash() {
//     let rng = &mut thread_rng();
//     let input = Fr::rand(rng);
//     poseidon_hash_rate_2_80_bits(&[input]);
// }
