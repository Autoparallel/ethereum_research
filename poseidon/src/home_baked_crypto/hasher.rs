use std::str::FromStr;

use super::{fields::FieldSize, *};
use constants::Constants;

// TODO: We should add in here a struct for all the parameters:
// * `Field`
// * `Rounds`
// * `Sbox`
// * `LinearLayer`
// * `Constants`

pub struct Poseidon<F: PrimeField> {
    field_size: FieldSize,
    rounds: Rounds,
    constants: Constants<F>,
    sbox: Sbox,
    linear_layer: LinearLayer<F>,
}

pub struct Rounds {
    full: usize,
    partial: usize,
}

struct Sbox(usize);

pub struct LinearLayer<F> {
    matrix: Vec<Vec<F>>,
}

impl<F> Poseidon<F>
where
    F: PrimeField + FromStr<Err = ()>,
{
    // TODO: Right now this ONLY USES LARGE FIELD CONSTANTS
    pub fn new(field_size: FieldSize) -> Poseidon<F> {
        Poseidon {
            field_size,
            constants: Constants::from(constants::large_field::constants()),
            rounds: Rounds {
                full: 1,
                partial: 0,
            },
            sbox: Sbox(3),
            linear_layer: LinearLayer {
                matrix: vec![vec![F::from_str("1").unwrap(); 3]; 3],
            },
        }
    }

    // TODO: Consider rename to `apply_round_constants`
    /// The `ark` function is a part of the Poseidon hash function.
    /// It adds round constants to the state.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the state vector.
    /// * `c` - A reference to the round constants.
    /// * `it` - The current iteration count.
    pub fn ark(&self, state: &mut Vec<F>, c: &[F], it: usize) {
        for i in 0..state.len() {
            state[i].add_assign(&c[it + i]);
        }
    }

    // TODO: Consider rename to `apply_sbox`
    /// The `sbox` function is a part of the Poseidon hash function.
    /// It applies a nonlinear function to the state.
    ///
    /// # Arguments
    ///
    /// * `n_rounds_f` - The number of full rounds.
    /// * `n_rounds_p` - The number of partial rounds.
    /// * `state` - A mutable reference to the state vector.
    /// * `i` - The current iteration count.
    pub fn sbox(&self, n_rounds_f: usize, n_rounds_p: usize, state: &mut [F], i: usize) {
        if i < n_rounds_f / 2 || i >= n_rounds_f / 2 + n_rounds_p {
            for state_elem in state.iter_mut() {
                let aux = *state_elem;
                state_elem.square();
                state_elem.square();
                state_elem.mul_assign(&aux);
            }
        } else {
            let aux = state[0];
            state[0].square();
            state[0].square();
            state[0].mul_assign(&aux);
        }
    }

    // TODO: Consider rename to `apply_linear_layer`
    /// The `mix` function is a part of the Poseidon hash function.
    /// It applies a linear transformation to the state.
    ///
    /// # Arguments
    ///
    /// * `state` - A reference to the state vector.
    /// * `m` - A reference to the matrix used for the linear transformation.
    ///
    /// # Returns
    ///
    /// * A new state vector after applying the linear transformation.
    pub fn mix(&self, state: &Vec<F>, m: &[Vec<F>]) -> Vec<F> {
        let mut new_state: Vec<F> = Vec::new();
        for i in 0..state.len() {
            new_state.push(F::from_str("0").unwrap());
            for (state_elem, mij) in state.iter().zip(&m[i]) {
                let mut mij = *mij;
                mij.mul_assign(state_elem);
                new_state[i].add_assign(&mij);
            }
        }
        new_state.clone()
    }

    // / The `hash` function is a part of the Poseidon hash function.
    // / It applies the Poseidon hash function to the input.
    // /
    // / # Arguments
    // /
    // / * `inp` - A vector of field elements to be hashed.
    // /
    // / # Returns
    // /
    // / * A field element which is the result of the hash function.
    // / * An error string if the input length is incorrect.
    // pub fn hash(&self, inp: Vec<F>) -> Result<F, String> {
    //     let t = inp.len() + 1;
    //     if inp.is_empty() || inp.len() > self.constants.n_rounds_p.len() {
    //         return Err("Wrong inputs length".to_string());
    //     }
    //     // TODO: Constants now come from the `Poseidon` struct itself.
    //     let n_rounds_f = self.constants.n_rounds_f;
    //     let n_rounds_p = self.constants.n_rounds_p[t - 2];

    //     let mut state = vec![F::zero(); t];
    //     state[1..].clone_Fom_slice(&inp);

    //     for i in 0..(n_rounds_f + n_rounds_p) {
    //         self.ark(&mut state, &self.constants.c[t - 2], i * t);
    //         self.sbox(n_rounds_f, n_rounds_p, &mut state, i);
    //         state = self.mix(&state, &self.constants.m[t - 2]);
    //     }

    //     Ok(state[0])
    // }
}
