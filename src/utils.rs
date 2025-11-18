use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use std::cmp::Ordering;

pub fn chinese_remainder_theorem(ciphertexts: Vec<BigInt>, modules: Vec<BigInt>) -> BigInt {
    let mut c = BigInt::zero();
    let mut n = BigInt::one();
    let mut n_vector: Vec<BigInt> = Vec::new();
    let mut y_vector: Vec<BigInt> = Vec::new();

    for x in &modules {
        n *= x;
    }

    for x in &modules {
        let n_value = &n / x;
        n_vector.push(n_value);
    }

    for i in 0..n_vector.len() {
        let result = n_vector[i].extended_gcd(&modules[i]);
        let y_coefficient = result.x;
        y_vector.push(y_coefficient);
    }

    for i in 0..n_vector.len() {
        let addend = &ciphertexts[i] * &n_vector[i] * &y_vector[i];
        c += addend;
    }

    c = c.mod_floor(&n);
    c
}

pub fn root(num: &BigInt, deg: u32) -> BigInt {
    let bit_len = num.bits();
    let upper_bits = ((bit_len + deg as u64 - 1) / deg as u64) + 1;
    let mut low = BigInt::one();
    let mut high = BigInt::one() << (upper_bits as usize);

    while low < high {
        let mid: BigInt = (&low + &high) >> 1_i32;
        let p = mid.pow(deg);

        match p.cmp(num) {
            Ordering::Equal => return mid,
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }

    low - 1
}
