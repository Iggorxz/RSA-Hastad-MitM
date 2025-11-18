use num_bigint::BigUint;
use num_traits::Num;

pub fn precalculation(l: u32, e: u32, n: BigUint) -> Vec<num_bigint::BigUint> {
    let mut result = Vec::with_capacity(1 << (l / 2));
    let e_big = BigUint::from(e);

    for i in 0..(1u64 << (l / 2)) {
        result.push(BigUint::from(i).modpow(&e_big, &n));
    }

    result
}
