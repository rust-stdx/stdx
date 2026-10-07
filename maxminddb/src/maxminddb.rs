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

#[derive(Debug)]
struct WithinNode {
    node: usize,
    ip_bytes: Vec<u8>,
    prefix_len: usize,
}

#[derive(Debug)]
pub struct Within<'de, T: Deserialize<'de>, S: AsRef<[u8]>> {
    reader: &'de Reader<S>,
    node_count: usize,
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
        while let Some(current) = self.stack.pop() {
            let bit_count = current.ip_bytes.len() * 8;

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
                    let ip_net = match bytes_and_prefix_to_net(&current.ip_bytes, current.prefix_len as u8) {
                        Ok(ip_net) => ip_net,
                        Err(e) => return Some(Err(e)),
                    };
                    // TODO: should this block become a helper method on reader?
                    let rec = match self.reader.resolve_data_pointer(current.node) {
                        Ok(rec) => rec,
                        Err(e) => return Some(Err(e)),
                    };
                    let mut decoder = decoder::Decoder::new(&self.reader.buf.as_ref()[self.reader.pointer_base..], rec);
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
                    // In order traversal of our children
                    // right/1-bit
                    let mut right_ip_bytes = current.ip_bytes.clone();
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
                        ip_bytes: current.ip_bytes.clone(),
                        prefix_len: current.prefix_len + 1,
                    });
                }
            }
        }
        None
    }
}

/// A reader for the MaxMind DB format. The lifetime `'data` is tied to the lifetime of the underlying buffer holding the contents of the database file.
#[derive(Debug)]
pub struct Reader<S: AsRef<[u8]>> {
    buf: S,
    pub metadata: Metadata,
    ipv4_start: usize,
    pointer_base: usize,
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
    /// # Example
    ///
    /// ```
    /// let reader = maxminddb::Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    /// ```
    pub fn open_readfile<P: AsRef<Path>>(database: P) -> Result<Reader<Vec<u8>>, MaxMindDBError> {
        use std::fs;

        let buf: Vec<u8> = fs::read(&database)?;
        Reader::from_source(buf)
    }
}

impl<'de, S: AsRef<[u8]>> Reader<S> {
    /// Open a MaxMind DB database from anything that implements AsRef<[u8]>
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs;
    /// let buf = fs::read("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    /// let reader = maxminddb::Reader::from_source(buf).unwrap();
    /// ```
    pub fn from_source(buf: S) -> Result<Reader<S>, MaxMindDBError> {
        let data_section_separator_size = 16;

        let metadata_start = find_metadata_start(buf.as_ref())?;
        let mut type_decoder = decoder::Decoder::new(&buf.as_ref()[metadata_start..], 0);
        let metadata = Metadata::deserialize(&mut type_decoder)?;

        let search_tree_size = (metadata.node_count as usize) * (metadata.record_size as usize) / 4;

        let mut reader = Reader {
            buf,
            pointer_base: search_tree_size + data_section_separator_size,
            metadata,
            ipv4_start: 0,
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
    pub fn lookup_prefix<T>(&'de self, address: IpAddr) -> Result<(T, usize), MaxMindDBError>
    where
        T: Deserialize<'de>,
    {
        let ip_bytes = ip_to_bytes(address);
        let (pointer, prefix_len) = self.find_address_in_tree(&ip_bytes)?;
        if pointer == 0 {
            return Err(MaxMindDBError::AddressNotFoundError(address));
        }

        let rec = self.resolve_data_pointer(pointer)?;
        let mut decoder = decoder::Decoder::new(&self.buf.as_ref()[self.pointer_base..], rec);

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
    pub fn within<T>(&'de self, cidr: IpNetwork) -> Result<Within<'de, T, S>, MaxMindDBError>
    where
        T: Deserialize<'de>,
    {
        let ip_address = cidr.network();
        let prefix_len = cidr.prefix() as usize;
        let ip_bytes = ip_to_bytes(ip_address);
        let bit_count = ip_bytes.len() * 8;

        let mut node = self.start_node(bit_count);
        let node_count = self.metadata.node_count as usize;

        let mut stack: Vec<WithinNode> = Vec::with_capacity(bit_count - prefix_len);

        // Traverse down the tree to the level that matches the cidr mark
        let mut i = 0_usize;
        while i < prefix_len {
            let bit = 1 & (ip_bytes[i >> 3] >> (7 - (i % 8))) as usize;
            node = self.read_node(node, bit)?;
            if node >= node_count {
                // We've hit a dead end before we exhausted our prefix
                break;
            }

            i += 1;
        }

        if node < node_count {
            // Ok, now anything that's below node in the tree is "within", start with the node we
            // traversed to as our to be processed stack.
            stack.push(WithinNode {
                node,
                ip_bytes,
                prefix_len,
            });
        }
        // else the stack will be empty and we'll be returning an iterator that visits nothing,
        // which makes sense.

        let within: Within<T, S> = Within {
            reader: self,
            node_count,
            stack,
            phantom: PhantomData,
        };

        Ok(within)
    }

    fn find_address_in_tree(&self, ip_address: &[u8]) -> Result<(usize, usize), MaxMindDBError> {
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

            node = self.read_node(node, bit as usize)?;
        }
        match node_count {
            n if n == node => Ok((0, prefix_len)),
            n if node > n => Ok((node, prefix_len)),
            _ => {
                // All bits consumed but we are at a node (exact prefix match).
                // Read the left child to get the data pointer. For an exact prefix
                // match the writer sets both children to the same data pointer.
                let data_ptr = self.read_node(node, 0)?;
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
        let buf = self.buf.as_ref();
        let base_offset = node_number * (self.metadata.record_size as usize) / 4;

        let val = match self.metadata.record_size {
            24 => {
                let offset = base_offset + index * 3;
                if offset + 3 > buf.len() {
                    return Err(MaxMindDBError::InvalidDatabaseError(
                        "search tree node offset beyond end of file".to_owned(),
                    ));
                }
                to_usize(0, &buf[offset..offset + 3])
            }
            28 => {
                if base_offset + 4 > buf.len() {
                    return Err(MaxMindDBError::InvalidDatabaseError(
                        "search tree node offset beyond end of file".to_owned(),
                    ));
                }
                let mut middle = buf[base_offset + 3];
                if index != 0 {
                    middle &= 0x0F
                } else {
                    middle = (0xF0 & middle) >> 4
                }
                let offset = base_offset + index * 4;
                if offset + 3 > buf.len() {
                    return Err(MaxMindDBError::InvalidDatabaseError(
                        "search tree node offset beyond end of file".to_owned(),
                    ));
                }
                to_usize(middle, &buf[offset..offset + 3])
            }
            32 => {
                let offset = base_offset + index * 4;
                if offset + 4 > buf.len() {
                    return Err(MaxMindDBError::InvalidDatabaseError(
                        "search tree node offset beyond end of file".to_owned(),
                    ));
                }
                to_usize(0, &buf[offset..offset + 4])
            }
            s => {
                return Err(MaxMindDBError::InvalidDatabaseError(format!(
                    "unknown record size: \
                     {s:?}"
                )));
            }
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

        if self.pointer_base + resolved >= self.buf.as_ref().len() {
            return Err(MaxMindDBError::InvalidDatabaseError(
                "the MaxMind DB file's search tree \
                 is corrupt"
                    .to_owned(),
            ));
        }

        Ok(resolved)
    }
}

// I haven't moved all patterns of this form to a generic function as
// the FromPrimitive trait is unstable
fn to_usize(base: u8, bytes: &[u8]) -> usize {
    bytes.iter().fold(base as usize, |acc, &b| (acc << 8) | b as usize)
}

fn ip_to_bytes(address: IpAddr) -> Vec<u8> {
    match address {
        IpAddr::V4(a) => a.octets().to_vec(),
        IpAddr::V6(a) => a.octets().to_vec(),
    }
}

#[allow(clippy::many_single_char_names)]
fn bytes_and_prefix_to_net(bytes: &[u8], prefix: u8) -> Result<IpNetwork, MaxMindDBError> {
    let (ip, pre) = match bytes.len() {
        4 => (IpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3])), prefix),
        16 => {
            if bytes[0] == 0
                && bytes[1] == 0
                && bytes[2] == 0
                && bytes[3] == 0
                && bytes[4] == 0
                && bytes[5] == 0
                && bytes[6] == 0
                && bytes[7] == 0
                && bytes[8] == 0
                && bytes[9] == 0
                && bytes[10] == 0
                && bytes[11] == 0
            {
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

fn find_metadata_start(buf: &[u8]) -> Result<usize, MaxMindDBError> {
    const METADATA_START_MARKER: &[u8] = b"\xab\xcd\xefMaxMind.com";

    memchr::memmem::rfind(buf, METADATA_START_MARKER)
        .map(|x| x + METADATA_START_MARKER.len())
        .ok_or_else(|| MaxMindDBError::InvalidDatabaseError("Could not find MaxMind DB metadata in file.".to_owned()))
}

#[cfg(test)]
mod reader_test;

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
