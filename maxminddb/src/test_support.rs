//! Helpers for building small, deliberately malformed MaxMind DB files in tests.
//!
//! This module is only compiled for tests. It lets robustness tests construct
//! databases whose metadata, search tree, or data section contain the kind of
//! hostile or corrupt content a real file might have.

/// A value to encode into a MaxMind DB data or metadata section.
#[derive(Debug, Clone)]
pub enum Val {
    Str(&'static str),
    U16(u16),
    U32(u32),
    U64(u64),
    Map(Vec<(&'static str, Val)>),
    Array(Vec<Val>),
    /// A pointer to a byte offset within the section currently being encoded.
    Ptr(usize),
    /// Raw bytes emitted verbatim, for malformed encodings.
    Raw(Vec<u8>),
}

fn enc_size(size: usize, out: &mut Vec<u8>) {
    if size < 29 {
        // size is fully encoded in the control byte
    } else if size < 29 + 256 {
        out.push((size - 29) as u8);
    } else if size < 29 + 256 + 65536 {
        let v = size - 285;
        out.push((v >> 8) as u8);
        out.push(v as u8);
    } else {
        let v = size - 65821;
        out.push((v >> 16) as u8);
        out.push((v >> 8) as u8);
        out.push(v as u8);
    }
}

fn enc_header(type_num: u8, size: usize, out: &mut Vec<u8>) {
    let size_bits = if size < 29 {
        size as u8
    } else if size < 29 + 256 {
        29
    } else if size < 29 + 256 + 65536 {
        30
    } else {
        31
    };
    if (1..=7).contains(&type_num) {
        out.push(type_num << 5 | size_bits);
    } else {
        out.push(size_bits);
        out.push(type_num - 7);
    }
    enc_size(size, out);
}

/// Encode `v` at the end of `out`.
pub fn enc_val(v: &Val, out: &mut Vec<u8>) {
    match v {
        Val::Str(s) => {
            enc_header(2, s.len(), out);
            out.extend_from_slice(s.as_bytes());
        }
        Val::U16(n) => {
            let b = n.to_be_bytes();
            let size = if *n == 0 {
                0
            } else if *n <= 0xff {
                1
            } else {
                2
            };
            enc_header(5, size, out);
            out.extend_from_slice(&b[2 - size..]);
        }
        Val::U32(n) => {
            let b = n.to_be_bytes();
            let mut size = 4;
            while size > 0 && b[4 - size] == 0 {
                size -= 1;
            }
            enc_header(6, size, out);
            out.extend_from_slice(&b[4 - size..]);
        }
        Val::U64(n) => {
            let b = n.to_be_bytes();
            let mut size = 8;
            while size > 0 && b[8 - size] == 0 {
                size -= 1;
            }
            enc_header(9, size, out);
            out.extend_from_slice(&b[8 - size..]);
        }
        Val::Map(entries) => {
            enc_header(7, entries.len(), out);
            for (k, val) in entries {
                enc_val(&Val::Str(k), out);
                enc_val(val, out);
            }
        }
        Val::Array(items) => {
            enc_header(11, items.len(), out);
            for item in items {
                enc_val(item, out);
            }
        }
        Val::Ptr(target) => {
            let t = *target;
            if t < 2048 {
                out.push(1 << 5 | ((t >> 8) as u8 & 0x07));
                out.push(t as u8);
            } else if t < 526_336 {
                let u = t - 2048;
                out.push(1 << 5 | 0x08 | ((u >> 16) as u8 & 0x07));
                out.push((u >> 8) as u8);
                out.push(u as u8);
            } else {
                let u = t - 526_336;
                out.push(1 << 5 | 0x10 | ((u >> 24) as u8 & 0x07));
                out.push((u >> 16) as u8);
                out.push((u >> 8) as u8);
                out.push(u as u8);
            }
        }
        Val::Raw(bytes) => out.extend_from_slice(bytes),
    }
}

/// A minimal MaxMind DB file description.
#[derive(Debug, Clone)]
pub struct Mmdb {
    /// Number of search tree nodes. May be overridden in the metadata.
    pub node_count: u32,
    /// Record size used to lay out the search tree (24 or 32).
    pub record_size: u16,
    pub ip_version: u16,
    /// `[left, right]` record value for each node. Values `>= node_count` are
    /// data pointers of the form `node_count + 16 + data_offset`.
    pub nodes: Vec<[u32; 2]>,
    pub data: Vec<u8>,
    pub extra_metadata: Vec<(&'static str, Val)>,
    pub database_type: &'static str,
    /// Metadata overrides used to build invalid files.
    pub meta_record_size: Option<u16>,
    pub meta_ip_version: Option<u16>,
    pub meta_node_count: Option<u32>,
    pub meta_major_version: Option<u16>,
}

impl Default for Mmdb {
    fn default() -> Self {
        Mmdb {
            node_count: 0,
            record_size: 24,
            ip_version: 4,
            nodes: Vec::new(),
            data: Vec::new(),
            extra_metadata: Vec::new(),
            database_type: "Test",
            meta_record_size: None,
            meta_ip_version: None,
            meta_node_count: None,
            meta_major_version: None,
        }
    }
}

impl Mmdb {
    /// Assemble the encoded database file.
    pub fn build(&self) -> Vec<u8> {
        let mut file = Vec::new();
        for node in &self.nodes {
            match self.record_size {
                24 => {
                    file.extend_from_slice(&node[0].to_be_bytes()[1..]);
                    file.extend_from_slice(&node[1].to_be_bytes()[1..]);
                }
                32 => {
                    file.extend_from_slice(&node[0].to_be_bytes());
                    file.extend_from_slice(&node[1].to_be_bytes());
                }
                r => panic!("unsupported record size {r}"),
            }
        }
        file.extend_from_slice(&[0_u8; 16]);
        file.extend_from_slice(&self.data);
        file.extend_from_slice(b"\xab\xcd\xefMaxMind.com");

        let mut meta = vec![
            ("binary_format_major_version", Val::U16(self.meta_major_version.unwrap_or(2))),
            ("binary_format_minor_version", Val::U16(0)),
            ("build_epoch", Val::U64(0)),
            ("database_type", Val::Str(self.database_type)),
            ("description", Val::Map(vec![("en", Val::Str("test"))])),
            ("ip_version", Val::U16(self.meta_ip_version.unwrap_or(self.ip_version))),
            ("languages", Val::Array(vec![Val::Str("en")])),
            ("node_count", Val::U32(self.meta_node_count.unwrap_or(self.node_count))),
            ("record_size", Val::U16(self.meta_record_size.unwrap_or(self.record_size))),
        ];
        meta.extend(self.extra_metadata.iter().map(|(k, v)| (*k, v.clone())));
        enc_val(&Val::Map(meta), &mut file);
        file
    }
}

/// The record value that points at `data_offset` in the data section.
pub fn data_ptr(node_count: u32, data_offset: u32) -> u32 {
    node_count + 16 + data_offset
}

/// Build a one-node database whose data section is `data`, reachable for every
/// address.
pub fn db_with_data(data: Vec<u8>, ip_version: u16) -> Vec<u8> {
    let node_count = 1;
    Mmdb {
        node_count,
        record_size: 24,
        ip_version,
        nodes: vec![[data_ptr(node_count, 0), data_ptr(node_count, 0)]],
        data,
        ..Default::default()
    }
    .build()
}

/// Build a one-node database whose data section is the encoding of `record`.
pub fn db_with_record(record: &Val, ip_version: u16) -> Vec<u8> {
    let mut data = Vec::new();
    enc_val(record, &mut data);
    db_with_data(data, ip_version)
}

/// Build an array encoding nested `depth` levels deep, followed by a uint16.
pub fn nested_arrays(depth: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(depth * 2 + 1);
    for _ in 0..depth {
        data.push(0x01); // size 1
        data.push(0x04); // extended type: 4 + 7 = 11 (array)
    }
    enc_val(&Val::U16(0), &mut data);
    data
}
