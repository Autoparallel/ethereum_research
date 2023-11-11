use starknet_crypto::{poseidon_hash, FieldElement};

#[test]
fn starknet_crypto() {
    let input_1 = FieldElement::from_hex_be("0x1").unwrap();
    let input_2 = FieldElement::from_hex_be("0x3").unwrap();
    println!("input_1: {:?}", input_1);
    println!("input_2: {:?}", input_2);
    let output = poseidon_hash(input_1, input_2);
    println!("output: {}", output);
}
