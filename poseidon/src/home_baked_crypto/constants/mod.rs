use std::str::FromStr;

use ff::PrimeField;

use super::*;

pub mod generate;
pub mod large_field;

#[derive(Debug)]
pub struct Constants<F: PrimeField> {
    pub c: Vec<Vec<F>>,
    pub m: Vec<Vec<Vec<F>>>,
}

impl<F> From<(Vec<Vec<&str>>, Vec<Vec<Vec<&str>>>)> for Constants<F>
where
    F: PrimeField + FromStr<Err = ()>,
{
    fn from((c, m): (Vec<Vec<&str>>, Vec<Vec<Vec<&str>>>)) -> Self {
        Self {
            c: c.into_iter()
                .map(|x| x.into_iter().map(|y| F::from_str(y).unwrap()).collect())
                .collect(),
            m: m.into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|y| y.into_iter().map(|z| F::from_str(z).unwrap()).collect())
                        .collect()
                })
                .collect(),
        }
    }
}

fn append_bits<T: Into<u128>>(vec: &mut Vec<bool>, n: usize, from: T) {
    let val = from.into();
    for i in (0..n).rev() {
        vec.push((val >> i) & 1 != 0);
    }
}

#[allow(dead_code)]
#[inline]
const fn bool_to_u8(bit: bool, offset: usize) -> u8 {
    if bit {
        1u8 << offset
    } else {
        0u8
    }
}

/// Converts a slice of bools into their byte representation, in little endian.
#[allow(dead_code)]
pub(crate) fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    bits.chunks(8)
        .map(|bits| {
            bool_to_u8(bits[7], 7)
                | bool_to_u8(bits[6], 6)
                | bool_to_u8(bits[5], 5)
                | bool_to_u8(bits[4], 4)
                | bool_to_u8(bits[3], 3)
                | bool_to_u8(bits[2], 2)
                | bool_to_u8(bits[1], 1)
                | bool_to_u8(bits[0], 0)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    pub(crate) use blstrs::Scalar as Fr;
    use serde_json::Value;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    #[test]
    fn test_round_constants() {
        // Bls12_381 modulus = 52435875175126190479447740508185965837690552500527637822603658699938581184513
        // In hex: 73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001
        let path = Path::new("parameters/round_constants-1-1-255-9-8-57-73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001.txt");
        let input = File::open(path).unwrap();
        let buffered = BufReader::new(input);
        let line = buffered.lines().nth(8).unwrap().unwrap();
        let replaced = line.replace('\'', "\"");
        let parsed: Vec<Value> = serde_json::from_str(&replaced).unwrap();

        let expected = parsed.iter().map(|x| {
            if let Value::String(s) = x {
                s
            } else {
                panic!("Could not parse round constant string.")
            }
        });

        // let generated = generate_constants::<Fr>(1, 1, 255, 9, 8, 57)
        //     .iter()
        //     .map(|x| {
        //         let s = x.to_string();
        //         let start = s.find('(').unwrap() + 1;
        //         s[start..s.len() - 1].to_string()
        //     })
        //     .collect::<Vec<_>>();

        // assert_eq!(expected.len(), generated.len());

        // generated
        //     .iter()
        //     .zip(expected)
        //     .for_each(|(generated, expected)| assert_eq!(generated, expected));
    }
}
