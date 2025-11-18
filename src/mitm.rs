use std::collections::HashMap;

use num_bigint::BigUint;

pub fn precalculation(l: u32, e: u32, n: &BigUint) -> HashMap<num_bigint::BigUint, u64> {
    let mut result = HashMap::with_capacity(1 << (l / 2));
    let e_big = BigUint::from(e);

    for i in 1..=(1u64 << (l / 2)) {
        result.insert(BigUint::from(i).modpow(&e_big, n), i);
    }

    result
}

pub fn calculation(
    precalc: HashMap<num_bigint::BigUint, u64>,
    c: &BigUint,
    n: &BigUint,
) -> Option<BigUint> {
    for (ele, index) in &precalc {
        let c_s = (c * ele.modinv(n).unwrap()) % n;

        if let Some(j) = precalc.get(&c_s) {
            return Some((index * j).into());
        }
    }
    None
}
