/// The goal here is to agrigate the most popular implementations
/// of Poseidon hash function. Then we can parameter sweep our attacks over them.
#[cfg(test)]
pub mod neptune;
#[cfg(test)]
pub mod starknet_crypto;
