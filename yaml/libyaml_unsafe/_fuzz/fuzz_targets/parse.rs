#![no_main]

use libfuzzer_sys::fuzz_target;
use std::cmp;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::addr_of_mut;
use libyaml_unsafe::{
    yaml_event_delete, yaml_event_t, yaml_parser_delete, yaml_parser_initialize, yaml_parser_parse,
    yaml_parser_set_input, yaml_parser_t, YAML_STREAM_END_EVENT,
};

fuzz_target!(|data: &[u8]| unsafe { fuzz_target(data) });

unsafe fn fuzz_target(mut data: &[u8]) {
    let mut parser = MaybeUninit::<yaml_parser_t>::uninit();
    let parser = parser.as_mut_ptr();
    assert!(yaml_parser_initialize(parser).ok);
    yaml_parser_set_input(parser, read_from_slice, addr_of_mut!(data).cast());

    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    while yaml_parser_parse(parser, event).ok {
        let type_ = (*event).type_;
        yaml_event_delete(event);
        if type_ == YAML_STREAM_END_EVENT {
            break;
        }
    }
    yaml_parser_delete(parser);
}

unsafe fn read_from_slice(
    data: *mut c_void,
    buffer: *mut u8,
    size: u64,
    size_read: *mut u64,
) -> i32 {
    let data = data.cast::<&[u8]>();
    let input = data.read();
    let n = cmp::min(input.len(), size as usize);
    ptr::copy_nonoverlapping(input.as_ptr(), buffer, n);
    data.write(&input[n..]);
    *size_read = n as u64;
    1
}
