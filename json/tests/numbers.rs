#![cfg(feature = "alloc")]

//! Differential tests for the number fast paths.

fn assert_same(text: &str, expected: f64) {
    let actual: f64 = json::from_str(text).unwrap();
    assert_eq!(
        actual.to_bits(),
        expected.to_bits(),
        "value mismatch for {text}: {actual:?} vs {expected:?}"
    );
}

#[test]
fn simple_decimals_match_std() {
    for text in [
        "0",
        "-0",
        "1",
        "-1",
        "0.1",
        "-0.1",
        "1.5",
        "3.141592653589793",
        "1e10",
        "1e-10",
        "1.234e10",
        "1.234e-10",
        "9007199254740991",
        "9007199254740992",
        "9007199254740993",
        "0.000000000000000001",
        "2.2250738585072014e-308",
        "1.7976931348623157e308",
        "1e309",
        "-1e309",
    ] {
        let expected = text.parse::<f64>();
        match expected {
            Ok(expected) if expected.is_finite() => assert_same(text, expected),
            _ => assert!(json::from_str::<f64>(text).is_err(), "accepted {text}"),
        }
    }
}

#[test]
fn random_decimals_match_std() {
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    for _ in 0..20_000 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let digits = 1 + (state % 15) as usize;
        let mut integer = String::new();
        let mut frac = String::new();
        let mut local = state;
        for _ in 0..digits {
            local = local.wrapping_mul(6364136223846793005).wrapping_add(1);
            integer.push((b'0' + (local % 10) as u8) as char);
        }
        if integer.as_bytes()[0] == b'0' && integer.len() > 1 {
            integer.replace_range(0..1, "1");
        }
        let frac_digits = (state >> 32) % 16;
        for _ in 0..frac_digits {
            local = local.wrapping_mul(6364136223846793005).wrapping_add(1);
            frac.push((b'0' + (local % 10) as u8) as char);
        }
        let exponent = ((state >> 48) as i32 % 41) - 20;
        let negative = state & 1 == 1;
        let mut text = String::new();
        if negative {
            text.push('-');
        }
        text.push_str(&integer);
        if !frac.is_empty() {
            text.push('.');
            text.push_str(&frac);
        }
        if exponent != 0 {
            text.push('e');
            text.push_str(&exponent.to_string());
        }
        let expected = text.parse::<f64>().unwrap();
        assert_same(&text, expected);
    }
}
