use ryu::Float;

pub fn itoa<I: core::fmt::Display>(n: I) -> String {
    n.to_string()
}

pub fn ftoa<F: Float>(n: F) -> String {
    let mut buffer = ryu::Buffer::new();
    return buffer.format(n).to_string();
}
