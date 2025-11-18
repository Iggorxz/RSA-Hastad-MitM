use std::{collections::HashMap, time::Instant};

use hex;
use malachite::strings::ToLowerHexString;
use num_bigint::BigUint;
use num_traits::Num;

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
    let mut a = precalc.iter().collect::<Vec<_>>();
    a.sort_by(|&a, &b| a.1.cmp(b.1));
    for (ele, index) in a {
        let c_s = (c * ele.modinv(n).unwrap()) % n;

        if let Some(j) = precalc.get(&c_s) {
            println!(
                "T = {}, S = {}",
                j.to_lower_hex_string(),
                index.to_lower_hex_string()
            );
            return Some((index * j).into());
        }
    }
    None
}

pub fn bruteforce(l: u32, c: &BigUint, n: &BigUint, e: &BigUint) -> Option<BigUint> {
    for i in 1..=(1u64 << l) {
        let m = BigUint::from(i).modpow(e, n);

        if &m == c {
            return Some(m);
        }
    }
    None
}

pub fn run_default() {
    let c = BigUint::from_str_radix("081ce089a9683931704f5b460e690262a6dae40209b7c3a56a5d24d45a7b886549d283635e2e3026444de31b8af5c22a23eb1248f274a6c04f1e207b301b0f1990f47c0195afcfcda89bf35b2d794c88421336ccabf12c356e57da0eefa21a4ec388fb6b14c3d852061ba4330b21f1beec985d28fe8d8720395dc28c80484881ceb962300ef1481879761789c39a7d237c737c74e1ea6f17cd022bbaf72ec0f78e50794f3697e9b986f4ec4d4395fcf07e30f7fa8433cfea1896b2cb55070b0d2a27fbbbd2ea43996b30bbbcce4e60fed0c41b9ddc73e1f646b188b9a93737c02f55e56d1d6aacfc72fd4555a4e0d946d71661d80d0725fc95ded3973ccb031f", 16).unwrap();
    let n = BigUint::from_str_radix("C72F82479D3B67ADD2795C936CC581AAEB0AD904C27FD4C1EFC22CAEB1671D45044C267479C65BA4E6FD2B7CD524EA41E0357E289C2231C4240CEEABB1876CF975D543F87252D4DBA271F8E5670DCB5D17C56F2656885BF61251000D7D225550E5A5E6EF33CD5ADC635FA050307D9B04A374FC5316E2AB72A0ECA81B753EBED59521F46EF9925EF9A4933C3526FF22D227B7E0FA5676B3044550ADF174DB2539B993090865A7E9925775D61FC6F471E72BC28C104F02343DCDBB16B11369FC462FE65345E6F8E7EA21F7FA4DA17CFB935651381C509AE7995C064E867FC49611AF2A2F8D7A964DE0C9DBB6E591860B5E1D0CECEA74C5135D150828879BD768F9", 16).unwrap();

    let now = Instant::now();
    let precalc = precalculation(20, 65537, &n);
    let elapsed_time = now.elapsed();

    println!(
        "time taken on precalculation: {}s",
        elapsed_time.as_secs_f32()
    );

    let now = Instant::now();
    if let Some(value) = calculation(precalc, &c, &n) {
        let e_big = BigUint::from(65537u64);
        assert!(value.modpow(&e_big, &n) == c);
        println!("ciphertext is found: {}", value.to_lower_hex_string())
    } else {
        println!("such ciphertext is not found :(")
    }

    let elapsed_time = now.elapsed();
    println!("time taken on mitm: {}s", elapsed_time.as_secs_f32());
}

pub fn run_dummy() {
    let c = BigUint::from_str_radix("68bd38258a47d141d7eadcd54277502a3e22e135395963213c8bf8809b9a011c21ff31eeb5d658e2caef1bb482e0061ece9fdf1dcd410b3152e40ca4966605c7", 16).unwrap();
    let n = BigUint::from_str_radix("C2E3FF95BB785E54BD1FC97A44D82DF7A5F6F87D0B33724F4D0899763876DA5BDAA0C41D97DEA7C2A2A79D2F93BB5CC1C0BC774D3D475DAFFB6DEC17CB96EF63", 16).unwrap();

    let now = Instant::now();
    let precalc = precalculation(20, 65537, &n);
    let elapsed_time = now.elapsed();

    println!(
        "time taken on precalculation: {}s",
        elapsed_time.as_secs_f32()
    );

    let now = Instant::now();
    if let Some(value) = calculation(precalc, &c, &n) {
        let e_big = BigUint::from(65537u64);
        assert!(value.modpow(&e_big, &n) == c);
        println!("ciphertext is found: {value}")
    } else {
        println!("such ciphertext is not found :(")
    }

    let elapsed_time = now.elapsed();
    println!("time taken on mitm: {}s", elapsed_time.as_secs_f32());
}

pub fn run_bruteforce() {
    let c = BigUint::from_str_radix("68bd38258a47d141d7eadcd54277502a3e22e135395963213c8bf8809b9a011c21ff31eeb5d658e2caef1bb482e0061ece9fdf1dcd410b3152e40ca4966605c7", 16).unwrap();
    let n = BigUint::from_str_radix("C2E3FF95BB785E54BD1FC97A44D82DF7A5F6F87D0B33724F4D0899763876DA5BDAA0C41D97DEA7C2A2A79D2F93BB5CC1C0BC774D3D475DAFFB6DEC17CB96EF63", 16).unwrap();

    let e = BigUint::from(65537u64);

    let now = Instant::now();
    let value = bruteforce(20, &c, &n, &e);
    let elapsed_time = now.elapsed();
    if let Some(s) = value {
        println!("found: {s}");
    } else {
        println!("not found")
    }
    println!("time taken on bruteforce: {}", elapsed_time.as_secs_f32());
}

pub fn run_bruteforce_dummy() {
    let c = BigUint::from_str_radix("5860dec3f02dd10d2e6dc83272f9158dc2ae2eafd892baef5c9b8b52ad547f4ad6a79765516410bc21bf8602752b20ccf4ee36ec4e5ee1acc0f2f86880e1b738ae5cf5c35e73a892e78633f1a3fa2ca8880dfdab54aad8571c0ba1a159e7895efae5890470af7ebf80cebf709caf2c448b5e2de378b39e27234113b029a9c690697e0b97a6e1e97909458be686a884c2fd0a74cc85568a10b3344082e8dedd930508a9c62227e25e89aa779fc1ad5ec30a355d84b209d9641e6d092a77de0e71ee19a16a95c0fc2c5ead148a9e3c752cd124dbd2e2d2bf7385e41810bf8c7f44ce8616136d1d269b3f893cb15f182989631daa61f295fa2c2a8e82969b345096", 16).unwrap();
    let n = BigUint::from_str_radix("d6022177e7453c4e8e165f545fc3c514d78dda75b6beae0cb5802fe4310298b9ef119cde0f78bffa1737f33af5904e09a6a4747bd63dc55a5a5f6fb597a424222223b4dd114fe036ee2a628261b3f29d8fc0acb402f3f1c09def5a96ed7844cebab5aa8994aa8d72b022405c68df5e7f8940d9914a62ee0f390e72215cd0cfc6d417735e81dbd334f21d11a7f2fd7fff040cab7bce92d358995159c1381fdbe207ed98b8fe4355170e5e2617bc1c68dda4c95f227090846de85095390cdcf9540614ddabe4ac2b01ecda5ad298ebe0bac3abd73854fb5c27133916cdaa105f5f9c902f199129ee1aa0e0fa85b4049b84d8bb021c81e8c7f73d36093b4da04383", 16).unwrap();

    let e = BigUint::from(65537u64);

    let now = Instant::now();
    let value = bruteforce(20, &c, &n, &e);
    let elapsed_time = now.elapsed();
    if let Some(s) = value {
        println!("found: {s}");
    } else {
        println!("not found")
    }
    println!("time taken on bruteforce: {}", elapsed_time.as_secs_f32());
}
