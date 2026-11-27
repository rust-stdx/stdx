//! Safe wrapper around the libyaml emitter that writes into a `Vec<u8>`.

use alloc::{string::String, vec::Vec};
use core::{
    ffi::c_void,
    mem::{self, MaybeUninit},
    ptr::{self, addr_of_mut},
    slice,
};

use libyaml_unsafe as sys;

use crate::libyaml::{error::Error, util::Owned};

pub(crate) struct Emitter {
    pin: Owned<EmitterPinned>,
}

struct EmitterPinned {
    sys: sys::yaml_emitter_t,
    output: Vec<u8>,
}

#[derive(Debug)]
pub(crate) enum Event<'a> {
    StreamStart,
    StreamEnd,
    DocumentStart,
    DocumentEnd,
    Scalar(Scalar<'a>),
    SequenceStart(Sequence),
    SequenceEnd,
    MappingStart(Mapping),
    MappingEnd,
}

#[derive(Debug)]
pub(crate) struct Scalar<'a> {
    pub tag: Option<String>,
    pub value: &'a str,
    pub style: ScalarStyle,
}

#[derive(Debug)]
pub(crate) enum ScalarStyle {
    Any,
    Plain,
    SingleQuoted,
    DoubleQuoted,
    Literal,
}

#[derive(Debug)]
pub(crate) struct Sequence {
    pub tag: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Mapping {
    pub tag: Option<String>,
}

impl Emitter {
    pub fn new() -> Emitter {
        let owned = Owned::<EmitterPinned>::new_uninit();
        let pin = unsafe {
            let emitter = addr_of_mut!((*owned.ptr).sys);
            if sys::yaml_emitter_initialize(emitter).fail {
                panic!("malloc error: {}", Error::emit_error(emitter));
            }
            sys::yaml_emitter_set_unicode(emitter, true);
            sys::yaml_emitter_set_width(emitter, -1);
            addr_of_mut!((*owned.ptr).output).write(Vec::new());
            // Pass the `Vec` field itself, not the enclosing `EmitterPinned`:
            // Rust does not guarantee field order, so `write_handler` cannot
            // rely on `output` being the first field.
            sys::yaml_emitter_set_output(emitter, write_handler, addr_of_mut!((*owned.ptr).output).cast());
            Owned::assume_init(owned)
        };
        Emitter {
            pin,
        }
    }

    pub fn emit(&mut self, event: Event) -> Result<(), Error> {
        let mut sys_event = MaybeUninit::<sys::yaml_event_t>::uninit();
        let sys_event = sys_event.as_mut_ptr();
        unsafe {
            let emitter = addr_of_mut!((*self.pin.ptr).sys);
            let initialize_status = match event {
                Event::StreamStart => sys::yaml_stream_start_event_initialize(sys_event, sys::YAML_UTF8_ENCODING),
                Event::StreamEnd => sys::yaml_stream_end_event_initialize(sys_event),
                Event::DocumentStart => {
                    let version_directive = ptr::null_mut();
                    let tag_directives_start = ptr::null_mut();
                    let tag_directives_end = ptr::null_mut();
                    let implicit = true;
                    sys::yaml_document_start_event_initialize(
                        sys_event,
                        version_directive,
                        tag_directives_start,
                        tag_directives_end,
                        implicit,
                    )
                }
                Event::DocumentEnd => {
                    let implicit = true;
                    sys::yaml_document_end_event_initialize(sys_event, implicit)
                }
                Event::Scalar(mut scalar) => {
                    reject_nul_tag(scalar.tag.as_ref())?;
                    let anchor = ptr::null();
                    let tag = scalar.tag.as_mut().map_or_else(ptr::null, |tag| {
                        tag.push('\0');
                        tag.as_ptr()
                    });
                    let value = scalar.value.as_ptr();
                    let length = scalar_length(scalar.value.len())?;
                    let plain_implicit = tag.is_null();
                    let quoted_implicit = tag.is_null();
                    let style = match scalar.style {
                        ScalarStyle::Any => sys::YAML_ANY_SCALAR_STYLE,
                        ScalarStyle::Plain => sys::YAML_PLAIN_SCALAR_STYLE,
                        ScalarStyle::SingleQuoted => sys::YAML_SINGLE_QUOTED_SCALAR_STYLE,
                        ScalarStyle::DoubleQuoted => sys::YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                        ScalarStyle::Literal => sys::YAML_LITERAL_SCALAR_STYLE,
                    };
                    sys::yaml_scalar_event_initialize(
                        sys_event,
                        anchor,
                        tag,
                        value,
                        length,
                        plain_implicit,
                        quoted_implicit,
                        style,
                    )
                }
                Event::SequenceStart(mut sequence) => {
                    reject_nul_tag(sequence.tag.as_ref())?;
                    let anchor = ptr::null();
                    let tag = sequence.tag.as_mut().map_or_else(ptr::null, |tag| {
                        tag.push('\0');
                        tag.as_ptr()
                    });
                    let implicit = tag.is_null();
                    let style = sys::YAML_ANY_SEQUENCE_STYLE;
                    sys::yaml_sequence_start_event_initialize(sys_event, anchor, tag, implicit, style)
                }
                Event::SequenceEnd => sys::yaml_sequence_end_event_initialize(sys_event),
                Event::MappingStart(mut mapping) => {
                    reject_nul_tag(mapping.tag.as_ref())?;
                    let anchor = ptr::null();
                    let tag = mapping.tag.as_mut().map_or_else(ptr::null, |tag| {
                        tag.push('\0');
                        tag.as_ptr()
                    });
                    let implicit = tag.is_null();
                    let style = sys::YAML_ANY_MAPPING_STYLE;
                    sys::yaml_mapping_start_event_initialize(sys_event, anchor, tag, implicit, style)
                }
                Event::MappingEnd => sys::yaml_mapping_end_event_initialize(sys_event),
            };
            if initialize_status.fail {
                return Err(Error::emit_error(emitter));
            }
            if sys::yaml_emitter_emit(emitter, sys_event).fail {
                return Err(Error::emit_error(emitter));
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        unsafe {
            let emitter = addr_of_mut!((*self.pin.ptr).sys);
            if sys::yaml_emitter_flush(emitter).fail {
                return Err(Error::emit_error(emitter));
            }
        }
        Ok(())
    }

    pub fn into_output(self) -> Vec<u8> {
        let this = self;
        mem::take(unsafe { &mut (*this.pin.ptr).output })
    }
}

unsafe fn write_handler(data: *mut c_void, buffer: *mut u8, size: u64) -> i32 {
    let output = unsafe { &mut *(data as *mut Vec<u8>) };
    output.extend_from_slice(unsafe { slice::from_raw_parts(buffer, size as usize) });
    1
}

/// Tags reach libyaml as NUL-terminated C strings, so an interior NUL would
/// silently truncate the tag (values are length-delimited and unaffected).
fn reject_nul_tag(tag: Option<&String>) -> core::result::Result<(), Error> {
    if let Some(tag) = tag {
        if tag.as_bytes().contains(&0) {
            return Err(Error::writer("YAML tag contains a NUL byte"));
        }
    }
    Ok(())
}

/// Converts a Rust string length to libyaml's `c_int` length.
///
/// A negative length tells libyaml to call `strlen`, which would read past the
/// end of a non-NUL-terminated Rust string, so oversized scalars are rejected
/// instead of truncated.
fn scalar_length(length: usize) -> core::result::Result<i32, Error> {
    // libyaml adds a byte for the NUL terminator internally, so leave room in
    // `c_int` instead of letting that addition wrap.
    match i32::try_from(length) {
        Ok(length) if length < i32::MAX => Ok(length),
        _ => Err(Error::writer("YAML scalar exceeds the emitter limit")),
    }
}

impl Drop for EmitterPinned {
    fn drop(&mut self) {
        unsafe { sys::yaml_emitter_delete(&mut self.sys) }
    }
}

#[cfg(test)]
mod tests {
    use super::scalar_length;

    #[test]
    fn rejects_scalars_that_do_not_fit_in_c_int() {
        assert!(scalar_length(i32::MAX as usize - 1).is_ok());
        assert!(scalar_length(i32::MAX as usize).is_err());
        assert!(scalar_length(i32::MAX as usize + 1).is_err());
    }
}
