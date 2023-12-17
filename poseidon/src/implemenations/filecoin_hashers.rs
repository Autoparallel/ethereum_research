use blstrs::Scalar as Fr;
use ff::{Field, PrimeField};
use filecoin_hashers::poseidon::{PoseidonDomain, PoseidonFunction};

use generic_array::typenum::{marker_traits::Unsigned, U2};
use merkletree::{
    hash::{Algorithm as LightAlgorithm, Hashable},
    merkle::{Element, MerkleTree},
    store::VecStore,
};

#[test]
fn test_poseidon_hasher() {
    let leaves = [
        PoseidonDomain::from(Fr::ONE.to_repr()),
        PoseidonDomain::from(Fr::ZERO.to_repr()),
        PoseidonDomain::from(Fr::TWO_INV.to_repr()),
    ];

    let t = MerkleTree::<PoseidonDomain, PoseidonFunction, VecStore<_>, U2>::new(
        leaves.iter().copied(),
    )
    .expect("merkle tree new failure");

    assert_eq!(t.leafs(), 3);
}
