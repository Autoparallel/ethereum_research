use std::f64::{INFINITY, log2, ceil};
use std::cmp::max;
use std::process::exit;

fn calc_round_numbers(prime_bit_len: f64, security_level: i32, t: i32, alpha: i32, security_margin: bool) -> (i32, i32, i32) {
    let mut partial_round = 0;
    let mut full_round = 0;

    let mut min_cost = INFINITY;
    let mut max_cost_rf = 0;
    // Brute-force approach
    for rp_i in 1..500 {
        for rf_i in (4..100).step_by(2) {
            if security_check(prime_bit_len, t, rf_i, rp_i, alpha, security_level) {
                if security_margin {
                    rf_i += 2;
                    rp_i = (ceil(rp_i as f64 * 1.075)) as i32;
                }

                let cost = t * rf_i + rp_i;
                if (cost as f64) < min_cost || ((cost as f64) == min_cost && rf_i < max_cost_rf) {
                    partial_round = ceil(rp_i as f64) as i32;
                    full_round = ceil(rf_i as f64) as i32;
                    min_cost = cost as f64;
                    max_cost_rf = full_round;
                }
            }
        }
    }

    (full_round, partial_round, full_round / 2)
}

fn security_check(prime_bit_len: f64, t: i32, full_round: i32, partial_round: i32, alpha: i32, security_level: i32) -> bool {
    let c = if alpha > 0 { log2(alpha as f64 - 1.0) } else { 2.0 };
    // The minimum full_round necessary to prevent statistical attacks
    let full_round_stat = if security_level <= ((prime_bit_len.floor() - c) * (t + 1) as f64) { 6 } else { 10 };

    if alpha > 0 {
        // The minimum number of rounds necessary to prevent interpolation attacks (full_round = R - partial_round + 1)
        let full_round_inter = ceil(log2(alpha as f64) * min(security_level as f64, prime_bit_len.ceil())) + ceil(
            log2(t as f64)) - partial_round as f64 + 1.0;

        // Groebner first limitation on number of total rounds
        let full_round_GR_1 = (log2(alpha as f64) * min(security_level as f64 / 3.0, prime_bit_len / 2.0)) - partial_round as f64 + 1.0;

        // Groebner second limitation on number of total rounds
        let full_round_GR_2 = min((log2(alpha as f64) * security_level as f64) / (t + 1) as f64,
                              ((log2(alpha as f64) * prime_bit_len) / 2.0)) - partial_round as f64 + t as f64 - 1.0;

        let full_round_max = max(ceil(full_round_stat as f64), max(ceil(full_round_inter), max(ceil(full_round_GR_1),
                             ceil(full_round_GR_2))));
        return full_round as f64 >= full_round_max;

    } else if alpha == -1 {
        // The minimum number of rounds necessary to prevent interpolation attacks (partial_round = R - full_round + 1)
        let partial_round_inter = ceil(0.5 * min(security_level as f64, prime_bit_len.ceil())) + ceil(log2(t as f64)) - (full_round as f64 * log2(t as f64)).floor() + 1.0;

        // Groebner second limitation on number of total rounds
        let partial_round_GR_2 = ceil(0.5 * min(security_level as f64 / (t + 1) as f64, ceil(0.5 * prime_bit_len))) + ceil(
            log2(t as f64)) + t as f64 - 1.0 - (full_round as f64 * log2(t as f64));

        let full_round_max = ceil(full_round_stat as f64);
        let partial_round_max = max(ceil(partial_round_inter), ceil(partial_round_GR_2));
        return full_round as f64 >= full_round_max && partial_round as f64 >= partial_round_max;
    } else {
        println!("Invalid value for alpha = {}. Required alpha > 0 or alpha = -1", alpha);
        exit(1);
    }
}
