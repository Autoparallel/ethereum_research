use super::*;

struct Grain {
    state: Vec<bool>,
    field_size: u16,
}

impl Grain {
    fn new(init_sequence: Vec<bool>, field_size: u16) -> Self {
        assert_eq!(80, init_sequence.len());
        let mut g = Grain {
            state: init_sequence,
            field_size,
        };
        for _ in 0..160 {
            g.generate_new_bit();
        }
        assert_eq!(80, g.state.len());
        g
    }

    fn generate_new_bit(&mut self) -> bool {
        let new_bit =
            self.bit(62) ^ self.bit(51) ^ self.bit(38) ^ self.bit(23) ^ self.bit(13) ^ self.bit(0);
        self.state.remove(0);
        self.state.push(new_bit);
        new_bit
    }

    fn bit(&self, index: usize) -> bool {
        self.state[index]
    }

    fn next_byte(&mut self, bit_count: usize) -> u8 {
        // Accumulate bits from most to least significant, so the most significant bit is the one generated first by the bit stream.
        let mut acc: u8 = 0;
        self.take(bit_count).for_each(|bit| {
            acc <<= 1;
            if bit {
                acc += 1;
            }
        });

        acc
    }

    fn get_next_bytes(&mut self, result: &mut [u8]) {
        let remainder_bits = self.field_size as usize % 8;

        // Prime fields will always have remainder bits,
        // but other field types could be supported in the future.
        if remainder_bits > 0 {
            // If there is an unfull byte, it should be the first.
            // Subsequent bytes are packed into result in the order generated.
            result[0] = self.next_byte(remainder_bits);
        } else {
            result[0] = self.next_byte(8);
        }

        // First byte is already set
        for item in result.iter_mut().skip(1) {
            *item = self.next_byte(8)
        }
    }
}

impl Iterator for Grain {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_bit = self.generate_new_bit();
        while !new_bit {
            let _new_bit = self.generate_new_bit();
            new_bit = self.generate_new_bit();
        }
        new_bit = self.generate_new_bit();
        Some(new_bit)
    }
}

// / From the paper ():
// / The round constants are generated using the Grain LFSR [23] in a self-shrinking
// / mode:
// / 1. Initialize the state with 80 bits b0, b1, . . . , b79, where
// / (a) b0, b1 describe the field,
// / (b) bi for 2 ≤ i ≤ 5 describe the S-Box,
// / (c) bi for 6 ≤ i ≤ 17 are the binary representation of n,
// / (d) bi for 18 ≤ i ≤ 29 are the binary representation of t,
// / (e) bi for 30 ≤ i ≤ 39 are the binary representation of RF ,
// / (f) bi for 40 ≤ i ≤ 49 are the binary representation of RP , and
// / (g) bi for 50 ≤ i ≤ 79 are set to 1.
// / 2. Update the bits using bi+80 = bi+62 ⊕ bi+51 ⊕ bi+38 ⊕ bi+23 ⊕ bi+13 ⊕ bi
// / .
// / 3. Discard the first 160 bits.
// / 4. Evaluate bits in pairs: If the first bit is a 1, output the second bit. If it is a
// / 0, discard the second bit.
// / Using this method, the generation of round constants depends on the specific
// / instance, and thus different round constants are used even if some of the chosen
// / parameters (e.g., n and t) are the same.
// / If a randomly sampled integer is not in Fp, we discard this value and take the
// / next one. Note that cryptographically strong randomness is not needed for the
// / round constants, and other methods can also be used.

// / Following https://extgit.iaik.tugraz.at/krypto/hadeshash/blob/master/code/scripts/create_rcs_grain.sage
// / The script was updated and can currently be found at:
// / https://extgit.iaik.tugraz.at/krypto/hadeshash/blob/master/code/generate_parameters_grain.sage
// pub fn generate_constants<F: PrimeField>(
//     field: u8,
//     sbox: u8,
//     field_size: u16,
//     t: u16,
//     r_f: u16,
//     r_p: u16,
// ) -> Vec<F> {
//     let n_bytes = F::Repr::default().as_ref().len();
//     if n_bytes != 32 {
//         unimplemented!("neptune currently supports 32-byte fields exclusively");
//     }
//     assert_eq!((f32::from(field_size) / 8.0).ceil() as usize, n_bytes);

//     let num_constants = (r_f + r_p) * t;
//     let mut init_sequence: Vec<bool> = Vec::new();
//     append_bits(&mut init_sequence, 2, field); // Bits 0-1
//     append_bits(&mut init_sequence, 4, sbox); // Bits 2-5
//     append_bits(&mut init_sequence, 12, field_size); // Bits 6-17
//     append_bits(&mut init_sequence, 12, t); // Bits 18-29
//     append_bits(&mut init_sequence, 10, r_f); // Bits 30-39
//     append_bits(&mut init_sequence, 10, r_p); // Bits 40-49
//     append_bits(&mut init_sequence, 30, 0b111111111111111111111111111111u128); // Bits 50-79

//     let mut grain = Grain::new(init_sequence, field_size);
//     let mut round_constants: Vec<F> = Vec::new();
//     match field {
//         1 => {
//             for _ in 0..num_constants {
//                 loop {
//                     // Generate 32 bytes and interpret them as a big-endian integer. Bytes are
//                     // big-endian to agree with the integers generated by grain_random_bits in the
//                     // reference implementation:
//                     //
//                     // def grain_random_bits(num_bits):
//                     //     random_bits = [grain_gen.next() for i in range(0, num_bits)]
//                     //     random_int = int("".join(str(i) for i in random_bits), 2)
//                     //     return random_int
//                     use byteorder::{BigEndian, ByteOrder};

//                     let repr = F::Repr::default();
//                     let repr_as_u8 = &mut [0u8; 8];
//                     BigEndian::write_u64(repr_as_u8, repr.as_ref()[0]);
//                     grain.get_next_bytes(repr_as_u8);
//                     repr_as_u8.reverse();
//                     let u64_val = BigEndian::read_u64(repr_as_u8);
//                     let repr_val = <F as ff::PrimeField>::Repr::from(u64_val);
//                     if let Ok(f) = F::from_repr(repr_val) {
//                         round_constants.push(f);
//                         break;
//                     }
//                 }
//             }
//         }
//         _ => {
//             panic!("Only prime fields are supported.");
//         }
//     }
//     round_constants
// }
