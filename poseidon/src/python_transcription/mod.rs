use std::cmp::max;

use ff::Field;
use num_bigint::BigInt;
use num_traits::{Zero, ToPrimitive, pow, One};



pub struct Poseidon {
    p: BigInt,
    security_level: BigInt,
    alpha: BigInt,
    input_rate: BigInt,
    t: BigInt,
    full_round: Option<BigInt>,
    partial_round: Option<BigInt>,
    mds_matrix: Option<Vec<Vec<BigInt>>>,
    rc_list: Option<Vec<BigInt>>,
    prime_bit_len: Option<BigInt>,
    state: Vec<BigInt>,
    rc_counter: BigInt,
}

impl Poseidon {
    pub fn new(
        p: BigInt,
        security_level: BigInt,
        alpha: BigInt,
        input_rate: BigInt,
        t: BigInt,
        full_round: Option<BigInt>,
        partial_round: Option<BigInt>,
        mds_matrix: Option<Vec<Vec<BigInt>>>,
        rc_list: Option<Vec<BigInt>>,
        prime_bit_len: Option<BigInt>,
    ) -> Poseidon {


        
        if (alpha.clone() % (p.clone() - 1)) != BigInt::one() {
            println!("Not available alpha");
            std::process::exit(1);
        }

        let prime_bit_len = match prime_bit_len {
            Some(val) => val,
            None => BigInt::from((p.bits() as f64).log2().ceil() as i64),
        };

        if BigInt::from(2).pow(security_level.clone().to_usize().unwrap().try_into().unwrap()) > p.clone().pow(t.clone().to_usize().unwrap().try_into().unwrap()) {
            println!("Not secure");
        }

        println!("Initialize Round Numbers");
        let (full_round, partial_round, half_full_round) = match (full_round, partial_round) {
            (Some(fr), Some(pr)) => {
                let half_fr = fr.clone() / 2;
                (fr, pr, half_fr)
            },
            _ => {
                // TODO: Implement rn.calc_round_numbers
                (BigInt::zero(), BigInt::zero(), BigInt::zero())
            }
        };

        // TODO: Implement field_p and rc_field
        let field_p = BigInt::zero();
        let rc_field = BigInt::zero();

        let mds_matrix = match mds_matrix {
            Some(matrix) => {
                if matrix.len() != t.to_usize().unwrap() || matrix[0].len() != t.to_usize().unwrap() {
                    panic!("Invalid size of MDS matrix");
                }
                // TODO: Implement rc.get_field_matrix_from_hex_matrix
                matrix
            },
            None => {
                println!("Initialize MDS matrix");
                // TODO: Implement rc.mds_matrix_generator
                vec![vec![BigInt::zero(); t.to_usize().unwrap()]; t.to_usize().unwrap()]
            }
        };

        let rc_list = match rc_list {
            Some(list) => {
                if list.len() != t.to_usize().unwrap() * (full_round.clone().to_usize().unwrap() + partial_round.clone().to_usize().unwrap()) {
                    panic!("Invalid number of round constants");
                }
                // TODO: Implement field_p conversion
                list
            },
            None => {
                println!("Initialize Round Constant");
                // TODO: Implement rc.calc_round_constants
                vec![BigInt::zero(); t.to_usize().unwrap() * (full_round.clone().to_usize().unwrap() + partial_round.clone().to_usize().unwrap())]
            }
        };

        Poseidon {
            p,
            security_level,
            alpha,
            input_rate,
            t,
            full_round: Some(full_round),
            partial_round: Some(partial_round),
            mds_matrix: Some(mds_matrix),
            rc_list: Some(rc_list),
            prime_bit_len: Some(prime_bit_len),
            state: vec![BigInt::zero()],
            rc_counter: BigInt::zero(),
        }
    }

    pub fn s_box(&self, element: &BigInt) -> BigInt {
        pow(element.clone(), self.alpha.clone().to_usize().unwrap())
    }


    pub fn full_rounds(&mut self) {
        for _r in 0..self.full_round.clone().unwrap().to_usize().unwrap() {
            for i in 0..self.t.to_usize().unwrap() {
                self.state[i] = &self.state[i] + &self.rc_list.as_ref().unwrap()[self.rc_counter.to_usize().unwrap()];
                self.rc_counter += 1;

                self.state[i] = self.s_box(&self.state[i]);
            }

            // apply MDS matrix
            let mds_matrix = self.mds_matrix.as_ref().unwrap();
            let mut new_state = vec![BigInt::zero(); self.t.to_usize().unwrap()];
            for i in 0..self.t.to_usize().unwrap() {
                for j in 0..self.t.to_usize().unwrap() {
                    new_state[i] += &mds_matrix[i][j] * &self.state[j];
                }
            }
            self.state = new_state;
        }
    }

    pub fn partial_rounds(&mut self) {
        for _r in 0..self.partial_round.clone().unwrap().to_usize().unwrap() {
            for i in 0..self.t.to_usize().unwrap() {
                self.state[i] = &self.state[i] + &self.rc_list.as_ref().unwrap()[self.rc_counter.to_usize().unwrap()];
                self.rc_counter += 1;
            }

            self.state[0] = self.s_box(&self.state[0]);

            // apply MDS matrix
            let mds_matrix = self.mds_matrix.as_ref().unwrap();
            let mut new_state = vec![BigInt::zero(); self.t.to_usize().unwrap()];
            for i in 0..self.t.to_usize().unwrap() {
                for j in 0..self.t.to_usize().unwrap() {
                    new_state[i] += &mds_matrix[i][j] * &self.state[j];
                }
            }
            self.state = new_state;
        }
    }
    pub fn run_hash(&mut self, mut input_vec: Vec<BigInt>) -> BigInt {
        if input_vec.len() < self.t.to_usize().unwrap() {
            input_vec.extend(vec![BigInt::zero(); self.t.to_usize().unwrap() - input_vec.len()]);
        }
        self.state = input_vec;
        self.rc_counter = BigInt::zero();

        // First full rounds
        self.full_rounds();

        // Middle partial rounds
        self.partial_rounds();

        // Last full rounds
        self.full_rounds();

        return self.state[1].clone();
    }
}