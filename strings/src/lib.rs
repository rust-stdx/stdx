use format_number::Integer;
use ryu::Float;

pub fn itoa<I: Integer>(n: I) -> String {
    format_number::format_int(n).to_string()
}

pub fn ftoa<F: Float>(n: F) -> String {
    let mut buffer = ryu::Buffer::new();
    return buffer.format(n).to_string();
}
