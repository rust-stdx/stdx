#![no_main]

use libfuzzer_sys::fuzz_target;
use std::cmp;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::addr_of_mut;
use libyaml_unsafe::{
    yaml_document_delete, yaml_document_get_root_node, yaml_document_t, yaml_parser_delete,
    yaml_parser_initialize, yaml_parser_load, yaml_parser_set_input, yaml_parser_t,
};

fuzz_target!(|data: &[u8]| unsafe { fuzz_target(data) });

unsafe fn fuzz_target(mut data: &[u8]) {
    let mut parser = MaybeUninit::<yaml_parser_t>::uninit();
    let parser = parser.as_mut_ptr();
    assert!(yaml_parser_initialize(parser).ok);
    yaml_parser_set_input(parser, read_from_slice, addr_of_mut!(data).cast());

    let mut document = MaybeUninit::<yaml_document_t>::uninit();
    let document = document.as_mut_ptr();
    while yaml_parser_load(parser, document).ok {
        let done = yaml_document_get_root_node(document).is_null();
        yaml_document_delete(document);
        if done {
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
