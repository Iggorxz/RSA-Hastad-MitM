use num_bigint::BigInt;
use crate::utils;
use std::time::Instant;

pub fn se_attack(ciphertexts: Vec<BigInt>, modules: Vec<BigInt>, exponent: u32) -> (String, u128) {
    let start = Instant::now();
    let c_value = utils::chinese_remainder_theorem(ciphertexts, modules);
    let plaintext = (utils::root(&c_value, exponent)).to_str_radix(16);
    let end = Instant::now();
    let duration = (end.duration_since(start)).as_millis();
    return (plaintext, duration);
}