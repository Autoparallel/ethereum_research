



/// Convert matrix in hex representation to matrix in field representation.
///
/// # Arguments
///
/// * `field_p` - A field field_p of type galois.GF(p).
/// * `mds_matrix` - 2-dim array of size t*t. Consist of elements in hex.
///
/// # Returns
///
/// * 2-dim array of size t*t. Consist of field elements.
fn get_field_matrix_from_hex_matrix(field_p: GF, mds_matrix: Vec<Vec<String>>) -> Vec<Vec<GF>> {
    let n = mds_matrix.len();
    let mut mds_matrix_field = vec![vec![field_p.zero(); n]; n];
    for i in 0..n {
        for j in 0..n {
            mds_matrix_field[i][j] = GF::from_str_radix(&mds_matrix[i][j][2..], 16).unwrap();
        }
    }
    mds_matrix_field
}

/// Help function for tests. Convert matrix in field representation to matrix in hex representation.
///
/// # Arguments
///
/// * `m` - 2-dim array of size t*t. Consist of field elements.
/// * `size` - The size of the hex string.
///
/// # Returns
///
/// * 2-dim array of size t*t. Consist of elements in hex.
fn get_hex_matrix_from_field_matrix(m: Vec<Vec<GF>>, size: usize) -> Vec<Vec<String>> {
    let n = m.len();
    let mut hex_matrix = vec![vec![String::new(); n]; n];
    for i in 0..n {
        for j in 0..n {
            hex_matrix[i][j] = format!("0x{:0>size$x}", m[i][j].to_int(), size = size);
        }
    }
    hex_matrix
}

/// The function generates the initial state for Grain LFSR  in a self-shrinking mode.
///
/// Initialize the state with 80 bits b0, b1, . . . , b79, where
///
/// (a) b0, b1 describe the field,
/// (b) bi for 2 ≤ i ≤ 5 describe the S-Box,
/// (c) bi for 6 ≤ i ≤ 17 are the binary representation of prime_bit_len,
/// (d) bi for 18 ≤ i ≤ 29 are the binary representation of t,
/// (e) bi for 30 ≤ i ≤ 39 are the binary representation of RF ,
/// (f) bi for 40 ≤ i ≤ 49 are the binary representation of RP , and
/// (g) bi for 50 ≤ i ≤ 79 are set to 1.
///
/// # Arguments
///
/// * `alpha` - The power of S-box.
/// * `p` - The prime field modulus.
/// * `prime_bit_len` - The number of bits of the Poseidon prime field modulus.
/// * `t` - The size of Poseidon's inner state
/// * `full_round` - Number of full rounds
/// * `partial_round` - Number of partial rounds
///
/// # Returns
///
/// * Initialized state with 80 elements of type int.
fn init_state_for_grain(alpha: i32, p: i32, prime_bit_len: usize, t: usize, full_round: usize, partial_round: usize) -> Vec<i32> {
    let mut init_state = vec![];
    let exp_flag = match alpha {
        3 => 0,
        5 => 1,
        -1 => 2,
        _ => 3,
    };

    init_state.extend(format!("{:02b}", p % 2).chars().map(|c| c.to_digit(10).unwrap() as i32));
    init_state.extend(format!("{:04b}", exp_flag).chars().map(|c| c.to_digit(10).unwrap() as i32));
    init_state.extend(format!("{:012b}", prime_bit_len).chars().map(|c| c.to_digit(10).unwrap() as i32));
    init_state.extend(format!("{:012b}", t).chars().map(|c| c.to_digit(10).unwrap() as i32));
    init_state.extend(format!("{:010b}", full_round).chars().map(|c| c.to_digit(10).unwrap() as i32));
    init_state.extend(format!("{:010b}", partial_round).chars().map(|c| c.to_digit(10).unwrap() as i32));
    init_state.extend(vec![1; 30]);

    init_state
}


fn calc_round_constants(t: usize, full_round: usize, partial_round: usize, p: i32, field_p: GF, alpha: i32, prime_bit_len: usize) -> Vec<GF> {
    let rc_number = t * (full_round + partial_round);

    let mut state = init_state_for_grain(alpha, p, prime_bit_len, t, full_round, partial_round);
    let mut rc_field = vec![];
    for _ in 0..160 {
        let new_bit = state[62] ^ state[51] ^ state[38] ^ state[23] ^ state[13] ^ state[0];
        state.remove(0);
        state.push(new_bit);
    }

    while rc_field.len() < rc_number {
        let (new_state, bits) = calc_next_bits(state, prime_bit_len);
        state = new_state;

        let rc_int = i32::from_str_radix(&bits.iter().collect::<String>(), 2).unwrap();
        if rc_int < p {
            rc_field.push(GF::from(rc_int));
        }
    }

    rc_field
}

/// Function generate new LFSR state after shifts new field_size number generated
///
/// Update the bits using bi+80 = bi+62 ⊕ bi+51 ⊕ bi+38 ⊕ bi+23 ⊕ bi+13 ⊕ bi.
/// Evaluate bits in pairs: If the first bit is a 1, output the second bit. If it is a 0, discard the second bit.
///
/// # Arguments
///
/// * `state` - Current LFSR state
/// * `prime_bit_len` - The number of bits of the Poseidon prime field modulus.
///
/// # Returns
///
/// * New LFSR state after shifts and new field_size number generated.
fn calc_next_bits(state: Vec<i32>, prime_bit_len: usize) -> (Vec<i32>, Vec<i32>) {
    let mut bits = vec![];
    let mut new_state = state;
    while bits.len() < prime_bit_len {
        let new_bit_1 = new_state[62] ^ new_state[51] ^ new_state[38] ^ new_state[23] ^ new_state[13] ^ new_state[0];
        new_state.remove(0);
        new_state.push(new_bit_1);

        let new_bit_2 = new_state[62] ^ new_state[51] ^ new_state[38] ^ new_state[23] ^ new_state[13] ^ new_state[0];
        new_state.remove(0);
        new_state.push(new_bit_2);

        if new_bit_1 == 1 {
            bits.push(new_bit_2);
        }
    }

    (new_state, bits)
}

/// This function generates a maximum distance separable (MDS) matrix,
/// which is used in linear layer of Poseidon hush function.
///
/// # Arguments
///
/// * `field_p` - A field field_p of type galois.GF(p).
/// * `t` - The size of Poseidon's inner state
///
/// # Returns
///
/// * 2-dim array of size t*t consist of filed elements.
fn mds_matrix_generator(field_p: GF, t: usize) -> Vec<Vec<GF>> {
    let x_vec: Vec<GF> = (0..t).map(|ele| GF::from(ele as i32)).collect();
    let y_vec: Vec<GF> = (t..2*t).map(|ele| GF::from(ele as i32)).collect();

    let mut mds_matrix = vec![vec![field_p.zero(); t]; t];
    for i in 0..t {
        for j in 0..t {
            mds_matrix[i][j] = (x_vec[i] + y_vec[j]).inv();
        }
    }

    mds_matrix
}

/// Given the round constants and MDS matrix for a Poseidon instance, we are able to derive optimized round constants
/// for the corresponding optimized Poseidon algorithm. A description of the optimisation can be found
/// here  https://github.com/filecoin-project/neptune/tree/master/spec under 'Optimized Round Constants'.
///
/// Each full round is associated with t field elements, while each partial round is associated with one field element.
///
/// # Arguments
///
/// * `rc` - List of pre-generated round constants of size full_round + partial_round.
/// * `half_full_round` - half the number of full rounds
/// * `partial_round` - Number of partial rounds
/// * `mds_matrix` - Pre-generated MDS matrix of size t*t consist of filed elements
///
/// # Returns
///
/// * List of field elements of size t*full_round + partial_round.
fn optimized_rc(rc: Vec<GF>, half_full_round: usize, partial_round: usize, mds_matrix: Vec<Vec<GF>>) -> Vec<GF> {
    let mut opt_rc_field = vec![];
    let m_inv = inv(mds_matrix);

    opt_rc_field.extend(rc[0].clone());
    for r in 1..half_full_round {
        let buf = dot(&rc[r], &m_inv);
        opt_rc_field.extend(buf);
    }

    let mut partial_const = vec![];
    let final_round = half_full_round + partial_round;
    let mut acc = rc[final_round].clone();
    for r in 0..partial_round {
        let acc_1 = dot(&acc, &m_inv);
        partial_const.push(acc_1[0]);
        acc_1[0] = GF::zero();
        acc = add(&acc_1, &rc[final_round - r - 1]);
    }

    opt_rc_field.extend(dot(&acc, &m_inv));
    opt_rc_field.extend(partial_const.into_iter().rev().collect::<Vec<_>>());

    let start = half_full_round + partial_round;
    for r in 1..half_full_round {
        opt_rc_field.extend(dot(&rc[start + r], &m_inv));
    }

    opt_rc_field
}

/// A description of the optimisation can be found
/// here https://github.com/filecoin-project/neptune/tree/master/spec under 'Sparse MDS Matrices'.
///
/// # Arguments
///
/// * `mds_matrix` - 2-dim array
/// * `partial_round` - Number of partial rounds
/// * `field_p` - A field field_p of type galois.GF(p).
///
/// # Returns
///
/// * 2-dim array correspond to pre-sparse matrix of size t*t
/// * list of 2-dim arrays correspond to sparce matrices each of which is of the size of t*t.
fn optimized_matrix(mds_matrix: Vec<Vec<GF>>, partial_round: usize, field_p: GF) -> (Vec<Vec<GF>>, Vec<Vec<Vec<GF>>>) {
    let mut sparse_matrices = vec![];
    let mut m = mds_matrix.clone();
    for _ in 0..partial_round {
        let (m_1, m_2) = sparse_factorize(m, field_p);
        sparse_matrices.push(m_2);
        m = dot(&mds_matrix, &m_1);
    }
    let pre_matrix = m;
    sparse_matrices.reverse();

    (pre_matrix, sparse_matrices)
}

/// A description of the optimisation can be found
/// here https://github.com/filecoin-project/neptune/tree/master/spec under 'Sparse MDS Matrices'.
///
/// # Arguments
///
/// * `m` - 2-dim array. Current matrix for calculating the two new matrices.
/// * `field_p` - A field field_p of type galois.GF(p).
///
/// # Returns
///
/// * Two 2-dim arrays of size t*t.
fn sparse_factorize(m: Vec<Vec<GF>>, field_p: GF) -> (Vec<Vec<GF>>, Vec<Vec<GF>>) {
    let mut m_1 = m.clone();
    for i in 0..m_1.len() {
        m_1[0][i] = GF::zero();
        m_1[i][0] = GF::zero();
    }
    m_1[0][0] = GF::one();

    let w = m[1..].iter().map(|row| row[0]).collect::<Vec<_>>();
    let m_cap = m[1..].iter().map(|row| row[1..].to_vec()).collect::<Vec<_>>();
    let m_inv = inv(m_cap);
    let w_cap = dot(&m_inv, &w);

    let mut m_2 = identity(m.len(), field_p);
    m_2[0] = m[0].clone();
    for i in 1..m_2.len() {
        m_2[i][0] = w_cap[i-1];
    }

    assert_eq!(m, dot(&m_1, &m_2));
    (m_1, m_2)
}
