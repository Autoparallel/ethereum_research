use filecoin_hashers::poseidon::PoseidonHasher;


#[test]
fn filecoin_hashers() {
    let input_1 = Fr::from_str("1").unwrap();
    let input_2 = Fr::from_str("3").unwrap();
    println!("input_1: {:?}", input_1);
    println!("input_2: {:?}", input_2);
    let output = PoseidonHasher::hash2(&input_1, &input_2);
    println!("output: {}", output);
}