use blstrs::Scalar as Fr;
use neptune::poseidon::PoseidonConstants;
use neptune::sponge::vanilla::{Mode, Sponge, SpongeTrait};
use typenum::U2;

pub fn neptune() {
    // Define the constants for the Poseidon hash function
    // the posdeidon constants are generic over the field [Fr] and the arity [U2].
    // The arity is the Merkle tree arity and is equal to the rate/capacity ratio
    let constants: PoseidonConstants<Fr, U2> = PoseidonConstants::new_constant_length(0);

    // Create a new Sponge object
    let sponge = Sponge::new_with_constants(&constants, Mode::Duplex);

    println!("Sponge object created: {:?}", sponge.state);

    // Now you can use the sponge object
    // ...
}

#[test]
fn test_neptune() {
    neptune();
}
