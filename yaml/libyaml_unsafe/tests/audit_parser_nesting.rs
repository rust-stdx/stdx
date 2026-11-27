//! Regression tests for CVE-2026-44842 in the vendored libyaml translation.
//!
//! libyaml 0.2.5 (the commit that `libyaml_unsafe` 0.2.11 was translated from)
//! had no nesting limit. libyaml 0.2.6 added `MAX_NESTING_LEVEL = 1000` and a
//! check in `yaml_parser_increase_flow_level` / `yaml_parser_roll_indent`;
//! this crate now ports that check.
//!
//! The ignored test is the original exhaustive reproducer (up to one million
//! levels). It is slow by design and is not part of the normal suite. The
//! non-ignored test verifies the boundary that the fix enforces.
//!
//! Run the exhaustive reproducer with:
//!     cargo test --release -p libyaml_unsafe --test audit_parser_nesting -- --ignored --nocapture

#![allow(unsafe_op_in_unsafe_fn)]

use std::{ffi::c_void, mem::MaybeUninit, ptr, ptr::addr_of_mut, time::Instant};

use libyaml_unsafe::{
    YAML_STREAM_END_TOKEN, yaml_parser_delete, yaml_parser_initialize, yaml_parser_scan, yaml_parser_set_input,
    yaml_parser_t, yaml_token_delete, yaml_token_t,
};

#[derive(Default)]
struct Reader {
    data: Vec<u8>,
    pos: usize,
}

unsafe fn read_handler(data: *mut c_void, buffer: *mut u8, size: u64, size_read: *mut u64) -> i32 {
    let reader = &mut *(data as *mut Reader);
    let n = std::cmp::min(reader.data.len() - reader.pos, size as usize);
    if n != 0 {
        ptr::copy_nonoverlapping(reader.data.as_ptr().add(reader.pos), buffer, n);
        reader.pos += n;
    }
    *size_read = n as u64;
    1
}

/// Returns (scan succeeded without a scanner error, number of tokens).
fn scan(input: &[u8]) -> (bool, usize) {
    unsafe {
        let mut reader = Reader {
            data: input.to_vec(),
            pos: 0,
        };
        let mut parser = MaybeUninit::<yaml_parser_t>::uninit();
        let parser = parser.as_mut_ptr();
        assert!(yaml_parser_initialize(parser).ok);
        yaml_parser_set_input(parser, read_handler, addr_of_mut!(reader).cast());
        let mut token = MaybeUninit::<yaml_token_t>::uninit();
        let token = token.as_mut_ptr();
        let mut count = 0;
        let mut ok = true;
        loop {
            if !yaml_parser_scan(parser, token).ok {
                ok = false;
                break;
            }
            let type_ = (*token).type_;
            yaml_token_delete(token);
            count += 1;
            if type_ == YAML_STREAM_END_TOKEN {
                break;
            }
        }
        let ok = ok && (&*parser).error as u32 == 0;
        yaml_parser_delete(parser);
        (ok, count)
    }
}

#[test]
#[ignore = "slow exhaustive CVE-2026-44842 reproducer (up to 1M levels); run explicitly"]
fn scanner_accepts_unbounded_nesting() {
    let mut first_over_limit = None;
    for depth in [1000usize, 1001, 2000, 4000, 8000, 16000, 32000, 250_000, 1_000_000] {
        let input = vec![b'['; depth];
        let start = Instant::now();
        let (ok, tokens) = scan(&input);
        let elapsed = start.elapsed();
        println!("depth={depth:6} scanner_ok={ok:5} tokens={tokens:6} time={elapsed:?}");
        if depth > 1000 && ok {
            first_over_limit.get_or_insert(depth);
        }
    }
    // libyaml 0.2.6 rejects anything above MAX_NESTING_LEVEL (1000).
    assert_eq!(
        first_over_limit, None,
        "scanner accepted depth {first_over_limit:?} (limit is 1000)"
    );
}

#[test]
fn scanner_accepts_up_to_1000_and_rejects_above() {
    // Regression test for the ported libyaml 0.2.6 limit: 1000 must still be
    // accepted, and 1001+ must be rejected. Without the fix the last two
    // assertions fail because every depth is accepted.
    for depth in [999usize, 1000] {
        let (ok, _) = scan(&vec![b'['; depth]);
        println!("depth={depth} scanner_ok={ok}");
        assert!(ok, "depth {depth} should be accepted");
    }
    for depth in [1001usize, 1002] {
        let (ok, _) = scan(&vec![b'['; depth]);
        println!("depth={depth} scanner_ok={ok}");
        assert!(!ok, "depth {depth} must be rejected");
    }
}
