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

pub fn run_default() {
    let c = BigUint::from_str_radix("5860dec3f02dd10d2e6dc83272f9158dc2ae2eafd892baef5c9b8b52ad547f4ad6a79765516410bc21bf8602752b20ccf4ee36ec4e5ee1acc0f2f86880e1b738ae5cf5c35e73a892e78633f1a3fa2ca8880dfdab54aad8571c0ba1a159e7895efae5890470af7ebf80cebf709caf2c448b5e2de378b39e27234113b029a9c690697e0b97a6e1e97909458be686a884c2fd0a74cc85568a10b3344082e8dedd930508a9c62227e25e89aa779fc1ad5ec30a355d84b209d9641e6d092a77de0e71ee19a16a95c0fc2c5ead148a9e3c752cd124dbd2e2d2bf7385e41810bf8c7f44ce8616136d1d269b3f893cb15f182989631daa61f295fa2c2a8e82969b345096", 16).unwrap();
    let n = BigUint::from_str_radix("D6022177E7453C4E8E165F545FC3C514D78DDA75B6BEAE0CB5802FE4310298B9EF119CDE0F78BFFA1737F33AF5904E09A6A4747BD63DC55A5A5F6FB597A424222223B4DD114FE036EE2A628261B3F29D8FC0ACB402F3F1C09DEF5A96ED7844CEBAB5AA8994AA8D72B022405C68DF5E7F8940D9914A62EE0F390E72215CD0CFC6D417735E81DBD334F21D11A7F2FD7FFF040CAB7BCE92D358995159C1381FDBE207ED98B8FE4355170E5E2617BC1C68DDA4C95F227090846DE85095390CDCF9540614DDABE4AC2B01ECDA5AD298EBE0BAC3ABD73854FB5C27133916CDAA105F5F9C902F199129EE1AA0E0FA85B4049B84D8BB021C81E8C7F73D36093B4DA04383", 16).unwrap();

    let precalc = precalculation(20, 65537, &n);

    if let Some(value) = calculation(precalc, &c, &n) {
        let e_big = BigUint::from(65537u64);
        assert!(value.modpow(&e_big, &n) == c);
        println!("ciphertext is found: {value}")
    } else {
        println!("such ciphertext is not found :(")
    }
}

pub fn run_dummy() {
    let c = BigUint::from_str_radix("68bd38258a47d141d7eadcd54277502a3e22e135395963213c8bf8809b9a011c21ff31eeb5d658e2caef1bb482e0061ece9fdf1dcd410b3152e40ca4966605c7", 16).unwrap();
    let n = BigUint::from_str_radix("C2E3FF95BB785E54BD1FC97A44D82DF7A5F6F87D0B33724F4D0899763876DA5BDAA0C41D97DEA7C2A2A79D2F93BB5CC1C0BC774D3D475DAFFB6DEC17CB96EF63", 16).unwrap();

    let precalc = precalculation(20, 65537, &n);

    if let Some(value) = calculation(precalc, &c, &n) {
        let e_big = BigUint::from(65537u64);
        assert!(value.modpow(&e_big, &n) == c);
        println!("ciphertext is found: {value}")
    } else {
        println!("such ciphertext is not found :(")
    }
}
