use zmij::Float;

pub fn itoa<I: core::fmt::Display>(n: I) -> String {
    n.to_string()
}

pub fn ftoa<F: Float>(n: F) -> String {
    let mut buffer = zmij::Buffer::new();
    return buffer.format(n).to_string();
}
