use neptune::sponge::vanilla::{Mode, Sponge, SpongeTrait};
use neptune::poseidon::PoseidonConstants;
use blstrs::Scalar as Fr;
use typenum::U2;

fn neptune() {
    // Define the constants for the Poseidon hash function
    let constants: PoseidonConstants<Fr, U2> = PoseidonConstants::new_constant_length(0);

    // Create a new Sponge object
    let sponge = Sponge::new_with_constants(&constants, Mode::Duplex);

    println!("Sponge object created: {:?}", sponge.state);

    // Now you can use the sponge object
    // ...
}