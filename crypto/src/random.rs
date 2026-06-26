pub fn random_bytes<const N: usize>() -> [u8; N] {
    let mut buf = [0u8; N];
    getrandom::fill(&mut buf).expect("getrandom failed");
    buf
}

pub fn random_fill(buf: &mut [u8]) {
    getrandom::fill(buf).expect("getrandom failed");
}
