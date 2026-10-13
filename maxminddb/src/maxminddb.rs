//! A reader for the MaxMind DB file format, including the GeoIP2 and GeoLite2
//! databases published by MaxMind.
//!
//! # Usage
//!
//! Open a database and look up an address:
//!
//! ```no_run
//! use std::net::IpAddr;
//!
//! use maxminddb::geoip2;
//!
//! # fn main() -> Result<(), maxminddb::MaxMindDBError> {
//! let reader = maxminddb::Reader::open_readfile("GeoIP2-City.mmdb")?;
//! let ip: IpAddr = "89.160.20.128".parse().unwrap();
//! let city: geoip2::City<'_> = reader.lookup(ip)?;
//! println!("{city:?}");
//! # Ok(())
//! # }
//! ```
//!
//! [`Reader::from_source`] accepts any buffer that implements `AsRef<[u8]>`,
//! which is useful when the database is already in memory or memory-mapped by
//! the caller.
//!
//! To iterate over every network in the database, use [`Reader::within`]. The
//! `examples/` directory contains `lookup`, `within`, and `write_database`
//! examples.
//!
//! # Resource limits
//!
//! A MaxMind DB encodes values with pointers, and many pointers may share one
//! target, so a small record can describe a very large or deeply nested
//! structure when expanded. To keep hostile or corrupt databases from consuming
//! unbounded time or memory, decoding is bounded by [`DecodeLimits`]:
//!
//! * a maximum nesting depth of `512` (maps, arrays, and followed pointers),
//! * a maximum of `65_536` decoded values per operation, and
//! * a maximum of `2 MiB` of string and byte payload per operation.
//!
//! These are the values recommended by the MaxMind DB specification. When a
//! limit is exceeded, decoding fails with [`MaxMindDBError::ResourceLimit`]
//! instead of panicking or exhausting memory. The limits apply to metadata
//! decoding at open as well as to record lookups.
//!
//! Use [`Reader::from_source_with_options`] or
//! [`Reader::open_readfile_with_options`] with a [`ReaderOptions`] value to
//! configure the limits.
//!
//! A database that is a valid IPv4-only file but is looked up with an IPv6
//! address or CIDR is rejected with [`MaxMindDBError::InvalidInputError`]; the
//! address family must match the database.
//!
//! # Supported databases
//!
//! Both IPv4 and IPv6 databases are supported, as are the 24-, 28-, and 32-bit
//! search tree record sizes. The [`geoip2`] module provides typed records for
//! the common GeoIP2 and GeoLite2 database shapes. For other databases,
//! deserialize into your own type or into [`Value`].
#![deny(trivial_casts, trivial_numeric_casts, unused_import_braces)]

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    io,
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    path::Path,
};

use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize, de};

// #[cfg(feature = "mmap")]
// pub use memmap2::Mmap;
// #[cfg(feature = "mmap")]
// use memmap2::MmapOptions;
// #[cfg(feature = "mmap")]
// use std::fs::File;

pub mod decoder;
pub mod encoder;
pub mod geoip2;
pub mod writer;
pub use encoder::Value;

#[derive(Debug, PartialEq, Eq)]
pub enum MaxMindDBError {
    AddressNotFoundError(IpAddr),
    InvalidDatabaseError(String),
    IoError(String),
    MapError(String),
    DecodingError(String),
    InvalidNetworkError(String),
    /// The caller supplied input that is incompatible with the database, such as
    /// an IPv6 address or CIDR for an IPv4-only database.
    InvalidInputError(String),
    /// Decoding the database record would have exceeded one of the resource
    /// limits configured via [`ReaderOptions`], so it was stopped.
    ResourceLimit(String),
}

impl From<io::Error> for MaxMindDBError {
    fn from(err: io::Error) -> MaxMindDBError {
        // clean up and clean up MaxMindDBError generally
        MaxMindDBError::IoError(err.to_string())
    }
}

impl Display for MaxMindDBError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            MaxMindDBError::AddressNotFoundError(msg) => write!(fmt, "AddressNotFoundError: {msg}")?,
            MaxMindDBError::InvalidDatabaseError(msg) => write!(fmt, "InvalidDatabaseError: {msg}")?,
            MaxMindDBError::IoError(msg) => write!(fmt, "IoError: {msg}")?,
            MaxMindDBError::MapError(msg) => write!(fmt, "MapError: {msg}")?,
            MaxMindDBError::DecodingError(msg) => write!(fmt, "DecodingError: {msg}")?,
            MaxMindDBError::InvalidNetworkError(msg) => write!(fmt, "InvalidNetworkError: {msg}")?,
            MaxMindDBError::InvalidInputError(msg) => write!(fmt, "InvalidInputError: {msg}")?,
            MaxMindDBError::ResourceLimit(msg) => write!(fmt, "ResourceLimit: {msg}")?,
        }
        Ok(())
    }
}

// Use default implementation for `core::error::Error`
impl core::error::Error for MaxMindDBError {}

impl de::Error for MaxMindDBError {
    fn custom<T: Display>(msg: T) -> Self {
        MaxMindDBError::DecodingError(format!("{msg}"))
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Metadata {
    pub binary_format_major_version: u16,
    pub binary_format_minor_version: u16,
    pub build_epoch: u64,
    pub database_type: String,
    pub description: BTreeMap<String, String>,
    pub ip_version: u16,
    pub languages: Vec<String>,
    pub node_count: u32,
    pub record_size: u16,
}

/// Resource limits applied while decoding a single data entry or the metadata
/// section.
///
/// A MaxMind DB can encode a value as a pointer, and many pointers can share
/// one target, so a small record can describe a structure that is huge, deeply
/// nested, or effectively infinite when expanded. These limits bound the time
/// and memory one decode operation may consume. When a limit is exceeded, the
/// decode fails with [`MaxMindDBError::ResourceLimit`].
///
/// The defaults follow the recommendations in the MaxMind DB specification: a
/// maximum nesting depth of `512`, a maximum of `65_536` decoded values, and a
/// maximum of `2 MiB` of string and byte payload materialized per operation.
/// The largest records MaxMind produces stay far below these values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeLimits {
    /// Maximum nesting depth. Each nested map, array, or followed pointer adds
    /// one level. This also bounds pointer cycles and unbounded pointer chains.
    ///
    /// The decoder is recursive, so raising this value increases stack usage
    /// proportionally. The default of `512` is safe on typical thread stacks.
    pub max_depth: usize,
    /// Maximum number of logical values decoded per operation. Every value,
    /// map key, and map value counts once; following a pointer does not add to
    /// the count.
    pub max_values: usize,
    /// Maximum number of string and byte payload bytes materialized per
    /// operation. Each occurrence of a string or byte value is charged.
    pub max_payload_bytes: usize,
}

impl Default for DecodeLimits {
    fn default() -> Self {
        DecodeLimits {
            max_depth: 512,
            max_values: 65_536,
            max_payload_bytes: 2 * 1024 * 1024,
        }
    }
}

/// Options controlling how a [`Reader`] opens and decodes a database.
///
/// Use [`Reader::from_source_with_options`] or
/// [`Reader::open_readfile_with_options`] to supply custom options. The
/// convenience constructors use `ReaderOptions::default()`, which applies the
/// recommended resource limits described on [`DecodeLimits`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReaderOptions {
    /// Resource limits applied to metadata and record decoding.
    pub limits: DecodeLimits,
}

#[derive(Debug, Clone, Copy)]
struct WithinNode {
    node: usize,
    ip_bytes: [u8; 16],
    prefix_len: usize,
}

#[derive(Debug)]
pub struct Within<'de, T: Deserialize<'de>, S: AsRef<[u8]>> {
    reader: &'de Reader<S>,
    node_count: usize,
    /// Length in bytes of the addresses being iterated (`4` or `16`).
    byte_len: usize,
    stack: Vec<WithinNode>,
    phantom: PhantomData<&'de T>,
}

#[derive(Debug)]
pub struct WithinItem<T> {
    pub ip_net: IpNetwork,
    pub info: T,
}

impl<'de, T: Deserialize<'de>, S: AsRef<[u8]>> Iterator for Within<'de, T, S> {
    type Item = Result<WithinItem<T>, MaxMindDBError>;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_count = self.byte_len * 8;
        while let Some(current) = self.stack.pop() {
            // Skip networks that are aliases for the IPv4 network
            if self.reader.ipv4_start != 0
                && current.node == self.reader.ipv4_start
                && bit_count == 128
                && current.ip_bytes[..12].iter().any(|&b| b != 0)
            {
                continue;
            }

            match current.node.cmp(&self.node_count) {
                Ordering::Greater => {
                    // This is a data node, emit it and we're done (until the following next call)
                    let ip_net =
                        match bytes_and_prefix_to_net(&current.ip_bytes[..self.byte_len], current.prefix_len as u8) {
                            Ok(ip_net) => ip_net,
                            Err(e) => return Some(Err(e)),
                        };
                    // TODO: should this block become a helper method on reader?
                    let rec = match self.reader.resolve_data_pointer(current.node) {
                        Ok(rec) => rec,
                        Err(e) => return Some(Err(e)),
                    };
                    let mut decoder = decoder::Decoder::with_limits(
                        &self.reader.buf.as_ref()[self.reader.pointer_base..],
                        rec,
                        self.reader.limits,
                    );
                    return match T::deserialize(&mut decoder) {
                        Ok(info) => Some(Ok(WithinItem {
                            ip_net,
                            info,
                        })),
                        Err(e) => Some(Err(e)),
                    };
                }
                Ordering::Equal => {
                    // Dead end, nothing to do
                }
                Ordering::Less => {
                    // A well-formed search tree cannot be deeper than the
                    // address width. A corrupt or cyclic tree would otherwise
                    // descend forever and grow the stack without bound.
                    if current.prefix_len >= bit_count {
                        return Some(Err(MaxMindDBError::InvalidDatabaseError(
                            "search tree is deeper than the address size".to_owned(),
                        )));
                    }
                    // In order traversal of our children
                    // right/1-bit
                    let mut right_ip_bytes = current.ip_bytes;
                    right_ip_bytes[current.prefix_len >> 3] |= 1 << ((bit_count - current.prefix_len - 1) % 8);
                    let node = match self.reader.read_node(current.node, 1) {
                        Ok(node) => node,
                        Err(e) => return Some(Err(e)),
                    };
                    self.stack.push(WithinNode {
                        node,
                        ip_bytes: right_ip_bytes,
                        prefix_len: current.prefix_len + 1,
                    });
                    // left/0-bit
                    let node = match self.reader.read_node(current.node, 0) {
                        Ok(node) => node,
                        Err(e) => return Some(Err(e)),
                    };
                    self.stack.push(WithinNode {
                        node,
                        ip_bytes: current.ip_bytes,
                        prefix_len: current.prefix_len + 1,
                    });
                }
            }
        }
        None
    }
}

/// A reader for the MaxMind DB format. The lifetime of the decoded values is tied to the lifetime of the underlying buffer holding the contents of the database file.
#[derive(Debug)]
pub struct Reader<S: AsRef<[u8]>> {
    buf: S,
    pub metadata: Metadata,
    ipv4_start: usize,
    pointer_base: usize,
    limits: DecodeLimits,
}

// #[cfg(feature = "mmap")]
// impl<'de> Reader<Mmap> {
//     /// Open a MaxMind DB database file by memory mapping it.
//     ///
//     /// # Example
//     ///
//     /// ```
//     /// let reader = maxminddb::Reader::open_mmap("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
//     /// ```
//     pub fn open_mmap<P: AsRef<Path>>(database: P) -> Result<Reader<Mmap>, MaxMindDBError> {
//         let file_read = File::open(database)?;
//         let mmap = unsafe { MmapOptions::new().map(&file_read) }?;
//         Reader::from_source(mmap)
//     }
// }

impl Reader<Vec<u8>> {
    /// Open a MaxMind DB database file by loading it into memory.
    ///
    /// Uses [`ReaderOptions::default`] for resource limits. Use
    /// [`Reader::open_readfile_with_options`] to configure them.
    ///
    /// # Example
    ///
    /// ```
    /// let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`MaxMindDBError::IoError`] if the file cannot be read, or an
    /// error from [`Reader::from_source`] if its contents are not a valid
    /// database.
    pub fn open_readfile<P: AsRef<Path>>(database: P) -> Result<Reader<Vec<u8>>, MaxMindDBError> {
        Reader::open_readfile_with_options(database, ReaderOptions::default())
    }

    /// Open a MaxMind DB database file by loading it into memory, using
    /// `options` for resource limits.
    ///
    /// # Errors
    ///
    /// Returns [`MaxMindDBError::IoError`] if the file cannot be read, or an
    /// error from [`Reader::from_source_with_options`] if its contents are not
    /// a valid database or exceed the configured limits.
    pub fn open_readfile_with_options<P: AsRef<Path>>(
        database: P,
        options: ReaderOptions,
    ) -> Result<Reader<Vec<u8>>, MaxMindDBError> {
        use std::fs;

        let buf: Vec<u8> = fs::read(&database)?;
        Reader::from_source_with_options(buf, options)
    }
}

impl<'de, S: AsRef<[u8]>> Reader<S> {
    /// Open a MaxMind DB database from anything that implements AsRef<[u8]>
    ///
    /// Uses [`ReaderOptions::default`] for resource limits. Use
    /// [`Reader::from_source_with_options`] to configure them.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// let buf = fs::read("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    /// let reader = maxminddb::Reader::from_source(buf).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`MaxMindDBError::InvalidDatabaseError`] if the buffer is not a
    /// valid MaxMind DB file, and [`MaxMindDBError::ResourceLimit`] if decoding
    /// the metadata exceeds the configured limits.
    pub fn from_source(buf: S) -> Result<Reader<S>, MaxMindDBError> {
        Reader::from_source_with_options(buf, ReaderOptions::default())
    }

    /// Open a MaxMind DB database from anything that implements `AsRef<[u8]>`,
    /// using `options` for resource limits.
    ///
    /// # Errors
    ///
    /// Returns [`MaxMindDBError::InvalidDatabaseError`] if the buffer is not a
    /// valid MaxMind DB file (including unsupported format versions, invalid
    /// record sizes or IP versions, and a search tree that does not fit in the
    /// buffer), and [`MaxMindDBError::ResourceLimit`] if decoding the metadata
    /// exceeds the configured limits.
    pub fn from_source_with_options(buf: S, options: ReaderOptions) -> Result<Reader<S>, MaxMindDBError> {
        let data_section_separator_size = 16;

        let metadata_start = find_metadata_start(buf.as_ref())?;
        let mut type_decoder = decoder::Decoder::with_limits(&buf.as_ref()[metadata_start..], 0, options.limits);
        let metadata = Metadata::deserialize(&mut type_decoder)?;
        validate_metadata(&metadata)?;

        let search_tree_size = (metadata.node_count as usize)
            .checked_mul(metadata.record_size as usize)
            .map(|v| v / 4)
            .ok_or_else(|| MaxMindDBError::InvalidDatabaseError("search tree size overflow".to_owned()))?;
        if search_tree_size > metadata_start.saturating_sub(data_section_separator_size) {
            return Err(MaxMindDBError::InvalidDatabaseError(
                "the MaxMind DB file's search tree is larger than the file".to_owned(),
            ));
        }

        let mut reader = Reader {
            buf,
            pointer_base: search_tree_size + data_section_separator_size,
            metadata,
            ipv4_start: 0,
            limits: options.limits,
        };
        reader.ipv4_start = reader.find_ipv4_start()?;

        Ok(reader)
    }

    /// Lookup the socket address in the opened MaxMind DB
    ///
    /// Example:
    ///
    /// ```
    /// use maxminddb::geoip2;
    /// use std::net::IpAddr;
    /// use std::str::FromStr;
    ///
    /// let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    ///
    /// let ip: IpAddr = FromStr::from_str("89.160.20.128").unwrap();
    /// let city: geoip2::City = reader.lookup(ip).unwrap();
    /// print!("{:?}", city);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`MaxMindDBError::AddressNotFoundError`] if the address has no
    /// record, [`MaxMindDBError::InvalidInputError`] for an IPv6 address in an
    /// IPv4-only database, and a decoding error (including
    /// [`MaxMindDBError::ResourceLimit`]) if the record cannot be decoded.
    pub fn lookup<T>(&'de self, address: IpAddr) -> Result<T, MaxMindDBError>
    where
        T: Deserialize<'de>,
    {
        self.lookup_prefix(address).map(|(v, _)| v)
    }

    /// Lookup the socket address in the opened MaxMind DB
    ///
    /// Example:
    ///
    /// ```
    /// use maxminddb::geoip2;
    /// use std::net::IpAddr;
    /// use std::str::FromStr;
    ///
    /// let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    ///
    /// let ip: IpAddr = "89.160.20.128".parse().unwrap();
    /// let (city, prefix_len) = reader.lookup_prefix::<geoip2::City>(ip).unwrap();
    /// print!("{:?}, prefix length: {}", city, prefix_len);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Reader::lookup`]. The prefix length of the
    /// matched network is returned alongside the value.
    pub fn lookup_prefix<T>(&'de self, address: IpAddr) -> Result<(T, usize), MaxMindDBError>
    where
        T: Deserialize<'de>,
    {
        if self.metadata.ip_version == 4 && address.is_ipv6() {
            return Err(MaxMindDBError::InvalidInputError(
                "IPv6 address cannot be looked up in an IPv4-only database".to_owned(),
            ));
        }
        let mut ip_bytes_buf = [0_u8; 16];
        let ip_bytes = ip_to_bytes(address, &mut ip_bytes_buf);
        let (pointer, prefix_len) = self.find_address_in_tree(ip_bytes)?;
        if pointer == 0 {
            return Err(MaxMindDBError::AddressNotFoundError(address));
        }

        let rec = self.resolve_data_pointer(pointer)?;
        let mut decoder = decoder::Decoder::with_limits(&self.buf.as_ref()[self.pointer_base..], rec, self.limits);

        T::deserialize(&mut decoder).map(|v| (v, prefix_len))
    }

    /// Iterate over blocks of IP networks in the opened MaxMind DB
    ///
    /// Example:
    ///
    /// ```
    /// use ipnetwork::IpNetwork;
    /// use maxminddb::{geoip2, Within};
    ///
    /// let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    ///
    /// let ip_net = IpNetwork::V6("::/0".parse().unwrap());
    /// let mut iter: Within<geoip2::City, _> = reader.within(ip_net).unwrap();
    /// while let Some(next) = iter.next() {
    ///     let item = next.unwrap();
    ///     println!("ip_net={}, city={:?}", item.ip_net, item.info);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`MaxMindDBError::InvalidInputError`] if `cidr` is an IPv6 CIDR
    /// and the database is IPv4-only. Problems found while walking the search
    /// tree are yielded as `Err` items from the returned iterator rather than
    /// panicking.
    pub fn within<T>(&'de self, cidr: IpNetwork) -> Result<Within<'de, T, S>, MaxMindDBError>
    where
        T: Deserialize<'de>,
    {
        let ip_address = cidr.network();
        if self.metadata.ip_version == 4 && ip_address.is_ipv6() {
            return Err(MaxMindDBError::InvalidInputError(
                "IPv6 CIDR cannot be iterated in an IPv4-only database".to_owned(),
            ));
        }
        let prefix_len = cidr.prefix() as usize;
        let byte_len = match ip_address {
            IpAddr::V4(_) => 4,
            IpAddr::V6(_) => 16,
        };
        let mut ip_bytes_buf = [0_u8; 16];
        let ip_bytes = ip_to_bytes(ip_address, &mut ip_bytes_buf);
        let bit_count = byte_len * 8;

        let mut node = self.start_node(bit_count);
        let node_count = self.metadata.node_count as usize;

        let mut stack: Vec<WithinNode> = Vec::with_capacity(bit_count - prefix_len);

        // Traverse down the tree to the level that matches the cidr mark
        let mut i = 0_usize;
        while i < prefix_len {
            if node >= node_count {
                // We've hit a dead end before we exhausted our prefix
                break;
            }
            let bit = 1 & (ip_bytes[i >> 3] >> (7 - (i % 8))) as usize;
            node = self.read_node(node, bit)?;
            i += 1;
        }

        if node < node_count {
            // Ok, now anything that's below node in the tree is "within", start with the node we
            // traversed to as our to be processed stack.
            let mut node_ip = [0_u8; 16];
            node_ip[..byte_len].copy_from_slice(ip_bytes);
            stack.push(WithinNode {
                node,
                ip_bytes: node_ip,
                prefix_len,
            });
        }
        // else the stack will be empty and we'll be returning an iterator that visits nothing,
        // which makes sense.

        let within: Within<T, S> = Within {
            reader: self,
            node_count,
            byte_len,
            stack,
            phantom: PhantomData,
        };

        Ok(within)
    }

    fn find_address_in_tree(&self, ip_address: &[u8]) -> Result<(usize, usize), MaxMindDBError> {
        // Dispatch on the record size once so the per-bit node reads below do
        // not repeat the branch on the hot lookup path.
        match self.metadata.record_size {
            24 => self.find_address_in_tree_sized::<24>(ip_address),
            28 => self.find_address_in_tree_sized::<28>(ip_address),
            32 => self.find_address_in_tree_sized::<32>(ip_address),
            s => Err(MaxMindDBError::InvalidDatabaseError(format!("unknown record size: {s:?}"))),
        }
    }

    #[inline]
    fn find_address_in_tree_sized<const RECORD_SIZE: usize>(
        &self,
        ip_address: &[u8],
    ) -> Result<(usize, usize), MaxMindDBError> {
        let bit_count = ip_address.len() * 8;
        let mut node = self.start_node(bit_count);

        let node_count = self.metadata.node_count as usize;
        let mut prefix_len = bit_count;

        for i in 0..bit_count {
            if node >= node_count {
                prefix_len = i;
                break;
            }
            let bit = 1 & (ip_address[i >> 3] >> (7 - (i % 8)));

            node = self.read_node_sized::<RECORD_SIZE>(node, bit as usize)?;
        }
        match node_count {
            n if n == node => Ok((0, prefix_len)),
            n if node > n => Ok((node, prefix_len)),
            _ => {
                // All bits consumed but we are at a node (exact prefix match).
                // Read the left child to get the data pointer. For an exact prefix
                // match the writer sets both children to the same data pointer.
                let data_ptr = self.read_node_sized::<RECORD_SIZE>(node, 0)?;
                if data_ptr > node_count {
                    Ok((data_ptr, prefix_len))
                } else {
                    Err(MaxMindDBError::InvalidDatabaseError("invalid node in search tree".to_owned()))
                }
            }
        }
    }

    fn start_node(&self, length: usize) -> usize {
        if length == 128 { 0 } else { self.ipv4_start }
    }

    fn find_ipv4_start(&self) -> Result<usize, MaxMindDBError> {
        if self.metadata.ip_version != 6 {
            return Ok(0);
        }

        // We are looking up an IPv4 address in an IPv6 tree. Skip over the
        // first 96 nodes.
        let mut node: usize = 0_usize;
        for _ in 0_u8..96 {
            if node >= self.metadata.node_count as usize {
                break;
            }
            node = self.read_node(node, 0)?;
        }
        Ok(node)
    }

    fn read_node(&self, node_number: usize, index: usize) -> Result<usize, MaxMindDBError> {
        match self.metadata.record_size {
            24 => self.read_node_sized::<24>(node_number, index),
            28 => self.read_node_sized::<28>(node_number, index),
            32 => self.read_node_sized::<32>(node_number, index),
            s => Err(MaxMindDBError::InvalidDatabaseError(format!(
                "unknown record size: \
                 {s:?}"
            ))),
        }
    }

    #[inline(always)]
    fn read_node_sized<const RECORD_SIZE: usize>(
        &self,
        node_number: usize,
        index: usize,
    ) -> Result<usize, MaxMindDBError> {
        fn oob() -> MaxMindDBError {
            MaxMindDBError::InvalidDatabaseError("search tree node offset beyond end of file".to_owned())
        }
        let buf = self.buf.as_ref();
        let base_offset = node_number
            .checked_mul(RECORD_SIZE)
            .map(|v| v / 4)
            .ok_or_else(|| MaxMindDBError::InvalidDatabaseError("search tree node offset overflow".to_owned()))?;
        let end_of = |start: usize, len: usize| start.checked_add(len).filter(|&end| end <= buf.len());

        let val = if RECORD_SIZE == 24 {
            let offset = base_offset.checked_add(index * 3).ok_or_else(oob)?;
            let end = end_of(offset, 3).ok_or_else(oob)?;
            to_usize(0, &buf[offset..end])
        } else if RECORD_SIZE == 28 {
            let mid = base_offset.checked_add(3).ok_or_else(oob)?;
            if mid >= buf.len() {
                return Err(oob());
            }
            let mut middle = buf[mid];
            if index != 0 {
                middle &= 0x0F
            } else {
                middle = (0xF0 & middle) >> 4
            }
            let offset = base_offset.checked_add(index * 4).ok_or_else(oob)?;
            let end = end_of(offset, 3).ok_or_else(oob)?;
            to_usize(middle, &buf[offset..end])
        } else {
            let offset = base_offset.checked_add(index * 4).ok_or_else(oob)?;
            let end = end_of(offset, 4).ok_or_else(oob)?;
            to_usize(0, &buf[offset..end])
        };
        Ok(val)
    }

    fn resolve_data_pointer(&self, pointer: usize) -> Result<usize, MaxMindDBError> {
        let node_count = self.metadata.node_count as usize;
        if pointer <= node_count {
            return Err(MaxMindDBError::AddressNotFoundError(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))));
        }
        let separator_size = 16;
        if pointer < node_count + separator_size {
            return Err(MaxMindDBError::InvalidDatabaseError(format!(
                "data pointer value {pointer} falls in the invalid range (node_count + 1 to node_count + 15)"
            )));
        }
        let resolved = pointer - node_count - separator_size;

        match self.pointer_base.checked_add(resolved) {
            Some(end) if end < self.buf.as_ref().len() => Ok(resolved),
            _ => Err(MaxMindDBError::InvalidDatabaseError(
                "the MaxMind DB file's search tree \
                 is corrupt"
                    .to_owned(),
            )),
        }
    }
}

// I haven't moved all patterns of this form to a generic function as
// the FromPrimitive trait is unstable
fn to_usize(base: u8, bytes: &[u8]) -> usize {
    bytes.iter().fold(base as usize, |acc, &b| (acc << 8) | b as usize)
}

fn ip_to_bytes(address: IpAddr, buf: &mut [u8; 16]) -> &[u8] {
    match address {
        IpAddr::V4(a) => {
            buf[..4].copy_from_slice(&a.octets());
            &buf[..4]
        }
        IpAddr::V6(a) => {
            buf.copy_from_slice(&a.octets());
            &buf[..16]
        }
    }
}

#[allow(clippy::many_single_char_names)]
fn bytes_and_prefix_to_net(bytes: &[u8], prefix: u8) -> Result<IpNetwork, MaxMindDBError> {
    let (ip, pre) = match bytes.len() {
        4 => (IpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3])), prefix),
        16 => {
            // Only a v4-mapped prefix of at least 96 bits is treated as IPv4.
            // Lower IPv6 networks (for example `::/64`) are kept as IPv6 rather
            // than underflowing `prefix - 96`.
            if prefix >= 96 && bytes[..12].iter().all(|&b| b == 0) {
                // It's actually v4, but in v6 form, convert would be nice if ipnetwork had this
                // logic.
                (
                    IpAddr::V4(Ipv4Addr::new(bytes[12], bytes[13], bytes[14], bytes[15])),
                    prefix - 96,
                )
            } else {
                let a = u16::from(bytes[0]) << 8 | u16::from(bytes[1]);
                let b = u16::from(bytes[2]) << 8 | u16::from(bytes[3]);
                let c = u16::from(bytes[4]) << 8 | u16::from(bytes[5]);
                let d = u16::from(bytes[6]) << 8 | u16::from(bytes[7]);
                let e = u16::from(bytes[8]) << 8 | u16::from(bytes[9]);
                let f = u16::from(bytes[10]) << 8 | u16::from(bytes[11]);
                let g = u16::from(bytes[12]) << 8 | u16::from(bytes[13]);
                let h = u16::from(bytes[14]) << 8 | u16::from(bytes[15]);
                (IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h)), prefix)
            }
        }
        // This should never happen
        _ => return Err(MaxMindDBError::InvalidNetworkError("invalid address".to_owned())),
    };
    IpNetwork::new(ip, pre).map_err(|e| MaxMindDBError::InvalidNetworkError(e.to_string()))
}

/// Rejects metadata that does not describe a readable database.
fn validate_metadata(metadata: &Metadata) -> Result<(), MaxMindDBError> {
    if metadata.binary_format_major_version != 2 {
        return Err(MaxMindDBError::InvalidDatabaseError(format!(
            "unsupported MaxMind DB binary format major version: {}",
            metadata.binary_format_major_version
        )));
    }
    if metadata.ip_version != 4 && metadata.ip_version != 6 {
        return Err(MaxMindDBError::InvalidDatabaseError(format!(
            "unsupported IP version in MaxMind DB metadata: {}",
            metadata.ip_version
        )));
    }
    if !matches!(metadata.record_size, 24 | 28 | 32) {
        return Err(MaxMindDBError::InvalidDatabaseError(format!(
            "unsupported record size in MaxMind DB metadata: {}",
            metadata.record_size
        )));
    }
    if metadata.node_count == 0 {
        return Err(MaxMindDBError::InvalidDatabaseError(
            "the MaxMind DB file has no search tree nodes".to_owned(),
        ));
    }
    Ok(())
}

fn find_metadata_start(buf: &[u8]) -> Result<usize, MaxMindDBError> {
    const METADATA_START_MARKER: &[u8] = b"\xab\xcd\xefMaxMind.com";

    memchr::memmem::rfind(buf, METADATA_START_MARKER)
        .map(|x| x + METADATA_START_MARKER.len())
        .ok_or_else(|| MaxMindDBError::InvalidDatabaseError("Could not find MaxMind DB metadata in file.".to_owned()))
}

#[cfg(test)]
mod reader_test;

#[cfg(test)]
mod robustness_test;

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod alloc_test;

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::MaxMindDBError;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!(
                "{}",
                MaxMindDBError::AddressNotFoundError(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            ),
            "AddressNotFoundError: 127.0.0.1".to_owned(),
        );
        assert_eq!(
            format!("{}", MaxMindDBError::InvalidDatabaseError("something went wrong".to_owned())),
            "InvalidDatabaseError: something went wrong".to_owned(),
        );
        assert_eq!(
            format!("{}", MaxMindDBError::IoError("something went wrong".to_owned())),
            "IoError: something went wrong".to_owned(),
        );
        assert_eq!(
            format!("{}", MaxMindDBError::MapError("something went wrong".to_owned())),
            "MapError: something went wrong".to_owned(),
        );
        assert_eq!(
            format!("{}", MaxMindDBError::DecodingError("something went wrong".to_owned())),
            "DecodingError: something went wrong".to_owned(),
        );
    }
}
