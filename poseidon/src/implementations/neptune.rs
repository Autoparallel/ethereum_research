use blstrs::Scalar as Fr;
use ff::PrimeField;
use neptune::sponge::vanilla::{Mode, Sponge, SpongeTrait};
use neptune::{poseidon::PoseidonConstants, round_numbers, Strength};
use typenum::U2;

const SBOX: u8 = 1; // x^5
const FIELD: u8 = 1; // Gf(p)

pub fn neptune() {
    // Define the constants for the Poseidon hash function
    // the posdeidon constants are generic over the field [Fr] and the arity [U2].
    // The arity is the Merkle tree arity and is equal to the rate/capacity ratio
    let constants: PoseidonConstants<Fr, U2> = PoseidonConstants::new_constant_length(0);

    // Create a new Sponge object
    // can be created with two modes
    // In simplex mode, absorbing and squeezing cannot be interleaved. First all elements are absorbed, then all needed
    // elements are squeezed. At most the number of elements which were absorbed can be squeezed. Elements must be absorbed in
    // chunks of R (rate). After every R chunks have been absorbed, the state is permuted. After the final element has been
    // absorbed, any needed padding is added, and the final permutation (or two -- if required by padding) is performed. Then
    // groups of R field elements are squeezed, and the state is permuted after each group of R elements has been squeezed.
    // After squeezing is complete, a simplex sponge is exhausted, and no further absorption is possible.

    // In duplex mode, absorbing and squeezing can be interleaved. The state is permuted after every R elements have been
    // absorbed. This makes R elements available to be squeezed. If elements remain to be squeezed when the state is permuted,
    // remaining unsqueezed elements are queued. Otherwise they would be lost when permuting.
    let mut sponge = Sponge::new_with_constants(&constants, Mode::Duplex);

    println!("Sponge object created: {:#?}", sponge.state);

    let elt: Fr = Fr::from(123);
    let _ = sponge.absorb(&elt, &mut ());
    println!("Sponge object created: {:#?}", sponge.state);

    // ...
}

#[test]
fn test_neptune() {
    neptune();
}
