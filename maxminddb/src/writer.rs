use std::{
    collections::{BTreeMap, HashMap},
    hash::{Hash, Hasher},
    io::Write,
    net::IpAddr,
    time::{SystemTime, UNIX_EPOCH},
};

use ipnetwork::IpNetwork;
use serde::Serialize;

use crate::encoder::{self, Value, encode_serialize};

/// The result of a writer operation.
///
/// The writer reports errors as strings rather than as
/// [`MaxMindDBError`](crate::MaxMindDBError) because it constructs databases
/// rather than reading them.
pub type WriterResult<T> = Result<T, String>;

fn map_io_err(e: std::io::Error) -> String {
    e.to_string()
}

const DATA_SECTION_SEPARATOR: &[u8; 16] = &[0u8; 16];
const METADATA_START_MARKER: &[u8] = b"\xab\xcd\xefMaxMind.com";

#[derive(Debug, Clone)]
enum RecordType {
    Empty,
    Data,
    Node,
}

#[derive(Debug, Clone)]
struct DataRecord {
    key: u64,
}

#[derive(Debug, Clone)]
struct ChildRecord {
    record_type: RecordType,
    data: Option<DataRecord>,
    node: Option<Box<Node>>,
}

impl ChildRecord {
    fn empty() -> Self {
        ChildRecord {
            record_type: RecordType::Empty,
            data: None,
            node: None,
        }
    }

    fn data(key: u64) -> Self {
        ChildRecord {
            record_type: RecordType::Data,
            data: Some(DataRecord {
                key,
            }),
            node: None,
        }
    }

    fn node(node: Node) -> Self {
        ChildRecord {
            record_type: RecordType::Node,
            data: None,
            node: Some(Box::new(node)),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    node_num: Option<usize>,
    children: [ChildRecord; 2],
}

impl Node {
    fn new() -> Self {
        Node {
            node_num: None,
            children: [ChildRecord::empty(), ChildRecord::empty()],
        }
    }

    fn insert(&mut self, ip: &[u8], prefix_len: usize, depth: usize, data_key: u64) -> WriterResult<()> {
        if depth == prefix_len {
            self.children[0] = ChildRecord::data(data_key);
            self.children[1] = ChildRecord::data(data_key);
            return Ok(());
        }

        let bit = bit_at(ip, depth);
        let child = &mut self.children[bit as usize];

        match &child.record_type {
            RecordType::Empty => {
                let mut subtree = Node::new();
                subtree.insert(ip, prefix_len, depth + 1, data_key)?;
                *child = ChildRecord::node(subtree);
            }
            RecordType::Data => {
                let existing = child.data.as_ref().unwrap();
                let mut subtree = Node::new();
                subtree.children[0] = ChildRecord::data(existing.key);
                subtree.children[1] = ChildRecord::data(existing.key);
                subtree.insert(ip, prefix_len, depth + 1, data_key)?;
                *child = ChildRecord::node(subtree);
            }
            RecordType::Node => {
                child
                    .node
                    .as_mut()
                    .unwrap()
                    .insert(ip, prefix_len, depth + 1, data_key)?;
            }
        }

        Ok(())
    }

    fn finalize(&mut self, next_node: &mut usize) {
        self.node_num = Some(*next_node);
        *next_node += 1;

        for i in 0..2 {
            if let Some(node) = self.children[i].node.as_mut() {
                node.finalize(next_node);
            }
        }
    }

    fn write_node<W: Write>(
        &self,
        writer: &mut W,
        node_count: usize,
        data_offsets: &BTreeMap<u64, usize>,
        record_size: u16,
    ) -> WriterResult<()> {
        let left_val = self.resolve_record_value(&self.children[0], node_count, data_offsets)?;
        let right_val = self.resolve_record_value(&self.children[1], node_count, data_offsets)?;

        let max_record = 1u64 << record_size;
        if left_val as u64 >= max_record || right_val as u64 >= max_record {
            return Err(format!(
                "record value ({}, {}) exceeds max for {} bit record size",
                left_val, right_val, record_size
            ));
        }

        match record_size {
            24 => writer
                .write_all(&[
                    ((left_val >> 16) & 0xFF) as u8,
                    ((left_val >> 8) & 0xFF) as u8,
                    (left_val & 0xFF) as u8,
                    ((right_val >> 16) & 0xFF) as u8,
                    ((right_val >> 8) & 0xFF) as u8,
                    (right_val & 0xFF) as u8,
                ])
                .map_err(map_io_err)?,
            28 => writer
                .write_all(&[
                    ((left_val >> 16) & 0xFF) as u8,
                    ((left_val >> 8) & 0xFF) as u8,
                    (left_val & 0xFF) as u8,
                    ((((left_val >> 24) & 0x0F) << 4) | ((right_val >> 24) & 0x0F)) as u8,
                    ((right_val >> 16) & 0xFF) as u8,
                    ((right_val >> 8) & 0xFF) as u8,
                    (right_val & 0xFF) as u8,
                ])
                .map_err(map_io_err)?,
            32 => writer
                .write_all(&[
                    ((left_val >> 24) & 0xFF) as u8,
                    ((left_val >> 16) & 0xFF) as u8,
                    ((left_val >> 8) & 0xFF) as u8,
                    (left_val & 0xFF) as u8,
                    ((right_val >> 24) & 0xFF) as u8,
                    ((right_val >> 16) & 0xFF) as u8,
                    ((right_val >> 8) & 0xFF) as u8,
                    (right_val & 0xFF) as u8,
                ])
                .map_err(map_io_err)?,
            s => return Err(format!("unsupported record size: {s}")),
        }

        for child in &self.children {
            if let Some(node) = &child.node {
                node.write_node(writer, node_count, data_offsets, record_size)?;
            }
        }

        Ok(())
    }

    fn resolve_record_value(
        &self,
        record: &ChildRecord,
        node_count: usize,
        data_offsets: &BTreeMap<u64, usize>,
    ) -> WriterResult<usize> {
        match &record.record_type {
            RecordType::Empty => Ok(node_count),
            RecordType::Data => {
                let d = record.data.as_ref().unwrap();
                let offset = data_offsets.get(&d.key).unwrap();
                Ok(node_count + 16 + offset)
            }
            RecordType::Node => Ok(record.node.as_ref().unwrap().node_num.unwrap()),
        }
    }
}

fn bit_at(ip: &[u8], depth: usize) -> u8 {
    (ip[depth >> 3] >> (7 - (depth & 7))) & 1
}

/// Builds a MaxMind DB file from inserted networks.
///
/// The writer accumulates encoded data records, deduplicating equal payloads,
/// and serializes a search tree plus metadata when [`Writer::write_to`] is
/// called.
///
/// # Example
///
/// ```
/// use maxminddb::writer::{Writer, WriterOptions};
///
/// let opts = WriterOptions {
///     database_type: "Test".to_string(),
///     ip_version: 4,
///     record_size: 24,
///     ..Default::default()
/// };
/// let mut writer = Writer::new(opts).unwrap();
/// writer.insert("10.0.0.0/8".parse().unwrap(), "example").unwrap();
///
/// let mut buf = Vec::new();
/// writer.write_to(&mut buf).unwrap();
/// let reader = maxminddb::Reader::from_source(buf).unwrap();
/// assert_eq!(reader.lookup::<String>("10.0.0.1".parse().unwrap()).unwrap(), "example");
/// ```
#[derive(Debug)]
pub struct Writer {
    root: Node,
    data_map: BTreeMap<u64, Vec<u8>>,
    /// Maps a hash of an encoded payload to the data keys with that hash, used
    /// to deduplicate equal payloads without an O(n) scan per insert.
    data_index: HashMap<u64, Vec<u64>>,
    node_count: usize,
    ip_version: u16,
    record_size: u16,
    database_type: String,
    description: BTreeMap<String, String>,
    languages: Vec<String>,
    build_epoch: u64,
    next_data_key: u64,
}

/// Configuration for a [`Writer`].
#[derive(Clone)]
pub struct WriterOptions {
    /// IP version of the database: `4` or `6`. For an IPv6 database, IPv4
    /// networks are stored in the IPv4-mapped subtree.
    pub ip_version: u16,
    /// Search tree record size in bits: `24`, `28`, or `32`. Larger records
    /// allow more nodes at the cost of file size.
    pub record_size: u16,
    /// Value for the `database_type` metadata field.
    pub database_type: String,
    /// Localized descriptions for the `description` metadata field.
    pub description: BTreeMap<String, String>,
    /// Languages present in `description`.
    pub languages: Vec<String>,
    /// Build time as a Unix timestamp for the `build_epoch` metadata field.
    pub build_epoch: u64,
}

impl Default for WriterOptions {
    fn default() -> Self {
        WriterOptions {
            ip_version: 6,
            record_size: 28,
            database_type: String::new(),
            description: BTreeMap::new(),
            languages: Vec::new(),
            build_epoch: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

impl Writer {
    /// Create a writer.
    ///
    /// # Errors
    ///
    /// Returns an error if `ip_version` is not `4` or `6`, or if `record_size`
    /// is not `24`, `28`, or `32`.
    pub fn new(opts: WriterOptions) -> WriterResult<Self> {
        if opts.ip_version != 4 && opts.ip_version != 6 {
            return Err(format!("invalid ip_version: {} (must be 4 or 6)", opts.ip_version));
        }
        if opts.record_size != 24 && opts.record_size != 28 && opts.record_size != 32 {
            return Err(format!("invalid record_size: {} (must be 24, 28, or 32)", opts.record_size));
        }
        Ok(Writer {
            root: Node::new(),
            data_map: BTreeMap::new(),
            data_index: HashMap::new(),
            node_count: 0,
            ip_version: opts.ip_version,
            record_size: opts.record_size,
            database_type: opts.database_type,
            description: opts.description,
            languages: opts.languages,
            build_epoch: opts.build_epoch,
            next_data_key: 1,
        })
    }

    /// Stores an encoded payload, returning the existing key if an equal
    /// payload was already stored.
    fn intern_value(&mut self, encoded: Vec<u8>) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        encoded.hash(&mut hasher);
        let digest = hasher.finish();

        if let Some(keys) = self.data_index.get(&digest) {
            for key in keys {
                if self.data_map.get(key).is_some_and(|existing| existing == &encoded) {
                    return *key;
                }
            }
        }

        let key = self.next_data_key;
        self.next_data_key += 1;
        self.data_map.insert(key, encoded);
        self.data_index.entry(digest).or_default().push(key);
        key
    }

    /// Insert a fully encoded [`Value`] for `network`.
    ///
    /// Equal values are stored once in the data section and referenced by every
    /// network that uses them.
    ///
    /// # Errors
    ///
    /// Returns an error if an IPv6 network is inserted into an IPv4 database or
    /// if the value cannot be encoded.
    ///
    /// # Overwriting
    ///
    /// Inserting a network replaces the subtree at that prefix. Inserting a
    /// *less specific* network after a more specific one therefore discards the
    /// more specific entries, so insert least specific networks first.
    pub fn insert_value(&mut self, network: IpNetwork, value: Value) -> WriterResult<()> {
        let prefix_len = network.prefix() as usize;
        let ip = ip_to_bytes(network.network());

        if self.ip_version == 4 && ip.len() != 4 {
            return Err("cannot insert IPv6 network into an IPv4 tree".to_string());
        }

        let (ip_bytes, actual_prefix_len) = if self.ip_version == 6 && ip.len() == 4 {
            let mut v6 = [0u8; 16];
            v6[12..].copy_from_slice(&ip);
            (v6.to_vec(), prefix_len + 96)
        } else {
            (ip, prefix_len)
        };

        let encoded = encoder::encode_value(&value)?;

        let data_key = self.intern_value(encoded);

        self.node_count = 0;
        self.root.insert(&ip_bytes, actual_prefix_len, 0, data_key)?;

        Ok(())
    }

    /// Insert a `Serialize` value for `network`.
    ///
    /// This is the same as [`Writer::insert_value`] but encodes `value` for you.
    ///
    /// # Errors
    ///
    /// Returns an error if `value` cannot be encoded, if an IPv6 network is
    /// inserted into an IPv4 database, or if the value is out of range for the
    /// MaxMind DB format (for example a negative integer below `i32::MIN`).
    ///
    /// # Overwriting
    ///
    /// See [`Writer::insert_value`]: insert least specific networks first.
    pub fn insert<T: Serialize>(&mut self, network: IpNetwork, value: T) -> WriterResult<()> {
        let encoded = encode_serialize(&value).map_err(|e| format!("serialization failed: {e}"))?;

        let prefix_len = network.prefix() as usize;
        let ip = ip_to_bytes(network.network());

        if self.ip_version == 4 && ip.len() != 4 {
            return Err("cannot insert IPv6 network into an IPv4 tree".to_string());
        }

        let (ip_bytes, actual_prefix_len) = if self.ip_version == 6 && ip.len() == 4 {
            let mut v6 = [0u8; 16];
            v6[12..].copy_from_slice(&ip);
            (v6.to_vec(), prefix_len + 96)
        } else {
            (ip, prefix_len)
        };

        let data_key = self.intern_value(encoded);

        self.node_count = 0;
        self.root.insert(&ip_bytes, actual_prefix_len, 0, data_key)?;

        Ok(())
    }

    /// Serialize the database into `writer`, returning the number of bytes
    /// written.
    ///
    /// This finalizes node numbering, so call it once, after all inserts.
    ///
    /// # Errors
    ///
    /// Returns an error if a record value does not fit the configured record
    /// size, if the node count exceeds `u32`, or if writing to `writer` fails.
    pub fn write_to<W: Write>(&mut self, writer: &mut W) -> WriterResult<u64> {
        let mut next = 1usize;
        for child in &mut self.root.children {
            if let Some(node) = child.node.as_mut() {
                node.finalize(&mut next);
            }
        }
        self.node_count = next;

        let search_tree_size_bytes = self
            .node_count
            .checked_mul(self.record_size as usize)
            .map(|v| v / 4)
            .ok_or_else(|| "search tree size overflow".to_string())?;
        let node_count_u32 =
            u32::try_from(self.node_count).map_err(|_| format!("node count {} exceeds u32", self.node_count))?;

        let mut data_offsets: BTreeMap<u64, usize> = BTreeMap::new();
        let mut data_section = Vec::new();
        for (key, encoded) in &self.data_map {
            let offset = data_section.len();
            data_section.extend_from_slice(encoded);
            data_offsets.insert(*key, offset);
        }

        let node_count = self.node_count;
        self.root
            .write_node(writer, node_count, &data_offsets, self.record_size)?;

        writer.write_all(DATA_SECTION_SEPARATOR).map_err(map_io_err)?;
        writer.write_all(&data_section).map_err(map_io_err)?;
        writer.write_all(METADATA_START_MARKER).map_err(map_io_err)?;

        let metadata = build_metadata_value(
            node_count_u32,
            self.record_size,
            self.ip_version,
            &self.database_type,
            &self.description,
            &self.languages,
            self.build_epoch,
        );
        let metadata_bytes = encoder::encode_value(&metadata)?;
        const METADATA_MAX_SIZE: usize = 128 * 1024;
        if metadata_bytes.len() > METADATA_MAX_SIZE {
            return Err(format!(
                "encoded metadata is {} bytes; the MaxMind DB maximum is {} bytes",
                metadata_bytes.len(),
                METADATA_MAX_SIZE
            ));
        }
        writer.write_all(&metadata_bytes).map_err(map_io_err)?;

        Ok((search_tree_size_bytes
            + DATA_SECTION_SEPARATOR.len()
            + data_section.len()
            + METADATA_START_MARKER.len()
            + metadata_bytes.len()) as u64)
    }
}

fn build_metadata_value(
    node_count: u32,
    record_size: u16,
    ip_version: u16,
    database_type: &str,
    description: &BTreeMap<String, String>,
    languages: &[String],
    build_epoch: u64,
) -> Value {
    let mut desc_map = BTreeMap::new();
    for (k, v) in description {
        desc_map.insert(k.clone(), Value::String(v.clone()));
    }

    let mut lang_array = Vec::new();
    for l in languages {
        lang_array.push(Value::String(l.clone()));
    }

    let mut m = BTreeMap::new();
    m.insert("binary_format_major_version".to_string(), Value::Uint16(2));
    m.insert("binary_format_minor_version".to_string(), Value::Uint16(0));
    m.insert("build_epoch".to_string(), Value::Uint64(build_epoch));
    m.insert("database_type".to_string(), Value::String(database_type.to_string()));
    m.insert("description".to_string(), Value::Map(desc_map));
    m.insert("ip_version".to_string(), Value::Uint16(ip_version));
    m.insert("languages".to_string(), Value::Slice(lang_array));
    m.insert("node_count".to_string(), Value::Uint32(node_count));
    m.insert("record_size".to_string(), Value::Uint16(record_size));
    Value::Map(m)
}

fn ip_to_bytes(address: IpAddr) -> Vec<u8> {
    match address {
        IpAddr::V4(a) => a.octets().to_vec(),
        IpAddr::V6(a) => a.octets().to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_simple_v4() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("country".to_string(), Value::String("US".to_string()));
        tree.insert_value("1.2.3.0/24".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            country: String,
        }
        let result: TestRecord = reader.lookup("1.2.3.4".parse().unwrap()).unwrap();
        assert_eq!(result.country, "US");
    }

    #[test]
    fn test_write_and_read_multiple_networks() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m1 = BTreeMap::new();
        m1.insert("country".to_string(), Value::String("US".to_string()));
        tree.insert_value("1.0.0.0/8".parse().unwrap(), Value::Map(m1)).unwrap();
        let mut m2 = BTreeMap::new();
        m2.insert("country".to_string(), Value::String("FR".to_string()));
        tree.insert_value("2.0.0.0/8".parse().unwrap(), Value::Map(m2)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            country: String,
        }
        let r1: TestRecord = reader.lookup("1.2.3.4".parse().unwrap()).unwrap();
        assert_eq!(r1.country, "US");
        let r2: TestRecord = reader.lookup("2.3.4.5".parse().unwrap()).unwrap();
        assert_eq!(r2.country, "FR");
        assert!(reader.lookup::<TestRecord>("3.4.5.6".parse().unwrap()).is_err());
    }

    #[test]
    fn test_v6_database() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 6,
            record_size: 28,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("country".to_string(), Value::String("FR".to_string()));
        tree.insert_value("2a00:1450:4000::/36".parse().unwrap(), Value::Map(m))
            .unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            country: String,
        }
        let r: TestRecord = reader.lookup("2a00:1450:4000::".parse().unwrap()).unwrap();
        assert_eq!(r.country, "FR");
    }

    #[test]
    fn test_all_record_sizes() {
        for record_size in [24, 28, 32] {
            let opts = WriterOptions {
                database_type: "Test".to_string(),
                ip_version: 4,
                record_size,
                ..Default::default()
            };
            let mut tree = Writer::new(opts).unwrap();
            let mut m = BTreeMap::new();
            m.insert("ip".to_string(), Value::String("test".to_string()));
            tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
            let mut buf = Vec::new();
            tree.write_to(&mut buf).unwrap();
            let reader = crate::Reader::from_source(buf.clone()).unwrap();
            #[derive(serde::Deserialize, Debug)]
            struct TestRecord {
                ip: String,
            }
            let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
            assert_eq!(r.ip, "test", "failed for record_size={record_size}");
        }
    }

    #[test]
    fn test_write_with_metadata() {
        let mut desc = BTreeMap::new();
        desc.insert("en".to_string(), "Test Database".to_string());
        let opts = WriterOptions {
            database_type: "TestIP".to_string(),
            ip_version: 4,
            record_size: 24,
            description: desc,
            languages: vec!["en".to_string()],
            build_epoch: 1000000,
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("data".to_string(), Value::String("hello".to_string()));
        tree.insert_value("192.168.0.0/16".parse().unwrap(), Value::Map(m))
            .unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        assert_eq!(reader.metadata.database_type, "TestIP");
        assert_eq!(reader.metadata.build_epoch, 1000000);
        assert_eq!(reader.metadata.description["en"], "Test Database");
    }

    #[test]
    fn test_bool_value() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("active".to_string(), Value::Bool(true));
        m.insert("inactive".to_string(), Value::Bool(false));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            active: bool,
            inactive: bool,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert!(r.active);
        assert!(!r.inactive);
    }

    #[test]
    fn test_slice_value() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert(
            "tags".to_string(),
            Value::Slice(vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
            ]),
        );
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            tags: Vec<String>,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.tags, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_integer_types() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("u16".to_string(), Value::Uint16(100));
        m.insert("u32".to_string(), Value::Uint32(100000));
        m.insert("i32".to_string(), Value::Int32(-42));
        m.insert("u64".to_string(), Value::Uint64(1 << 40));
        m.insert("u128".to_string(), Value::Uint128(1u128 << 100));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            u16: u16,
            u32: u32,
            i32: i32,
            u64: u64,
            u128: u128,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.u16, 100);
        assert_eq!(r.u32, 100000);
        assert_eq!(r.i32, -42);
        assert_eq!(r.u64, 1 << 40);
        assert_eq!(r.u128, 1u128 << 100);
    }

    #[test]
    fn test_pointer_encoding_size3() {
        let v = Value::Pointer(200000000);
        let buf = encoder::encode_value(&v).unwrap();
        assert_eq!(buf[0], 0b00111000, "size=3 pointer control byte should be 0x38");
        let ptr_val = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
        assert_eq!(ptr_val, 200000000);
    }

    #[test]
    fn test_pointer_encoding_size0() {
        let v = Value::Pointer(500);
        let buf = encoder::encode_value(&v).unwrap();
        assert_eq!(buf[0], 0b00100001);
        assert_eq!(buf.len(), 2);
        let decoded = ((buf[0] as u32 & 0x07) << 8) | buf[1] as u32;
        assert_eq!(decoded, 500);
    }

    #[test]
    fn test_pointer_encoding_size1() {
        let v = Value::Pointer(10000);
        let buf = encoder::encode_value(&v).unwrap();
        assert_eq!((buf[0] >> 5) & 0x07, 0b001);
        assert_eq!((buf[0] >> 3) & 0x03, 0b01);
        assert_eq!(buf.len(), 3);
    }

    #[test]
    fn test_empty_database() {
        let opts = WriterOptions {
            database_type: "Empty".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut buf = Vec::new();
        let n = tree.write_to(&mut buf).unwrap();
        assert!(n > 0);
        let reader = crate::Reader::from_source(buf).unwrap();
        assert_eq!(reader.metadata.database_type, "Empty");
        assert_eq!(reader.metadata.node_count, 1);
    }

    #[test]
    fn test_overlapping_networks() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m1 = BTreeMap::new();
        m1.insert("data".to_string(), Value::String("broad".to_string()));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m1))
            .unwrap();

        let mut m2 = BTreeMap::new();
        m2.insert("data".to_string(), Value::String("specific".to_string()));
        tree.insert_value("10.1.0.0/16".parse().unwrap(), Value::Map(m2))
            .unwrap();

        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();

        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            data: String,
        }

        let r1: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r1.data, "broad");

        let r2: TestRecord = reader.lookup("10.1.0.1".parse().unwrap()).unwrap();
        assert_eq!(r2.data, "specific");
    }

    #[test]
    fn test_nested_map() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut inner = BTreeMap::new();
        inner.insert("code".to_string(), Value::Uint16(1));
        inner.insert("name".to_string(), Value::String("US".to_string()));

        let mut m = BTreeMap::new();
        m.insert("country".to_string(), Value::Map(inner));
        m.insert("active".to_string(), Value::Bool(true));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();

        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();

        #[derive(serde::Deserialize, Debug)]
        struct Country {
            code: u16,
            name: String,
        }
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            country: Country,
            active: bool,
        }

        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.country.code, 1);
        assert_eq!(r.country.name, "US");
        assert!(r.active);
    }

    #[test]
    fn test_data_deduplication() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };

        let mut tree = Writer::new(opts.clone()).unwrap();
        let shared_data = {
            let mut m = BTreeMap::new();
            m.insert("val".to_string(), Value::String("shared".to_string()));
            Value::Map(m)
        };
        tree.insert_value("10.0.0.0/8".parse().unwrap(), shared_data.clone())
            .unwrap();
        tree.insert_value("11.0.0.0/8".parse().unwrap(), shared_data).unwrap();
        let mut buf_dedup = Vec::new();
        tree.write_to(&mut buf_dedup).unwrap();

        let mut tree2 = Writer::new(opts).unwrap();
        let mut m1 = BTreeMap::new();
        m1.insert("val".to_string(), Value::String("first".to_string()));
        let mut m2 = BTreeMap::new();
        m2.insert("val".to_string(), Value::String("second".to_string()));
        tree2
            .insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m1))
            .unwrap();
        tree2
            .insert_value("11.0.0.0/8".parse().unwrap(), Value::Map(m2))
            .unwrap();
        let mut buf_no_dedup = Vec::new();
        tree2.write_to(&mut buf_no_dedup).unwrap();

        assert!(
            buf_dedup.len() < buf_no_dedup.len(),
            "dedup size {} should be < no-dedup size {}",
            buf_dedup.len(),
            buf_no_dedup.len()
        );

        let r1 = crate::Reader::from_source(buf_dedup).unwrap();
        let r2 = crate::Reader::from_source(buf_no_dedup).unwrap();
        assert_eq!(r1.metadata.node_count, r2.metadata.node_count);
    }

    #[test]
    fn test_ipv4_in_ipv6_tree() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 6,
            record_size: 28,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("v4in6".to_string(), Value::Bool(true));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();

        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();

        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            v4in6: bool,
        }

        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert!(r.v4in6);
    }

    #[test]
    fn test_record_size_24_node_format() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("1.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let reader = crate::Reader::from_source(buf.clone()).unwrap();
        let node_count = reader.metadata.node_count;
        let nc = node_count as u64;
        let left = ((buf[0] as u64) << 16) | ((buf[1] as u64) << 8) | (buf[2] as u64);
        let right = ((buf[3] as u64) << 16) | ((buf[4] as u64) << 8) | (buf[5] as u64);
        assert!(left <= nc || left >= nc + 16, "left={left} nc={nc}");
        assert!(right <= nc || right >= nc + 16, "right={right} nc={nc}");
    }

    #[test]
    fn test_record_size_28_node_format() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 28,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("1.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let node_count = crate::Reader::from_source(buf.clone()).unwrap().metadata.node_count as usize;
        assert!(buf.len() > node_count * 7);
    }

    #[test]
    fn test_record_size_32_node_format() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 32,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("1.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let node_count = crate::Reader::from_source(buf.clone()).unwrap().metadata.node_count as usize;
        assert!(buf.len() > node_count * 8);
    }

    #[test]
    fn test_search_tree_size_calculation() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let reader = crate::Reader::from_source(&buf[..]).unwrap();
        let node_count = reader.metadata.node_count as usize;
        let record_size = reader.metadata.record_size as usize;

        // The search tree occupies node_count * record_size / 4 bytes, and must
        // fit in the file ahead of the 16-byte separator and the data section.
        let tree_size = node_count * record_size / 4;
        assert!(tree_size > 0);
        assert!(tree_size + 16 < buf.len());
    }

    #[test]
    fn test_metadata_binary_format_version() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        assert_eq!(reader.metadata.binary_format_major_version, 2);
        assert_eq!(reader.metadata.binary_format_minor_version, 0);
    }

    #[test]
    fn test_metadata_description_optional() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            description: BTreeMap::new(),
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        assert!(reader.metadata.description.is_empty());
    }

    #[test]
    fn test_languages_optional() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            languages: vec![],
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        assert!(reader.metadata.languages.is_empty());
    }

    #[test]
    fn test_insert_multiple_cidrs_then_lookup() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        for i in 0u8..10 {
            let cidr = format!("{i}.0.0.0/8");
            let mut m = BTreeMap::new();
            m.insert("net".to_string(), Value::String(cidr.clone()));
            tree.insert_value(cidr.parse().unwrap(), Value::Map(m)).unwrap();
        }
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            net: String,
        }
        for i in 0u8..10 {
            let ip = format!("{i}.0.0.1");
            let r: TestRecord = reader.lookup(ip.parse().unwrap()).unwrap();
            assert_eq!(r.net, format!("{i}.0.0.0/8"));
        }
    }

    #[test]
    fn test_large_string_value() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let long_str = "x".repeat(100);
        let mut m = BTreeMap::new();
        m.insert("long".to_string(), Value::String(long_str.clone()));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            long: String,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.long, long_str);
    }

    #[test]
    fn test_metadata_start_marker_last_occurrence() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let marker = b"\xab\xcd\xefMaxMind.com";
        let all_positions: Vec<usize> = buf
            .windows(marker.len())
            .enumerate()
            .filter(|(_, w)| *w == marker)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(all_positions.len(), 1);
        let last_pos = all_positions[0];
        assert!(last_pos + marker.len() <= buf.len());
    }

    #[test]
    fn test_write_to_is_idempotent() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(42));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();

        let mut buf1 = Vec::new();
        let n1 = tree.write_to(&mut buf1).unwrap();

        let mut buf2 = Vec::new();
        let n2 = tree.write_to(&mut buf2).unwrap();

        assert_eq!(n1, n2);
        assert_eq!(buf1.len(), buf2.len());
        assert_eq!(buf1, buf2);
    }

    #[test]
    fn test_zero_length_prefix() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("all".to_string(), Value::Bool(true));
        tree.insert_value("0.0.0.0/0".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            all: bool,
        }
        let r: TestRecord = reader.lookup("255.255.255.255".parse().unwrap()).unwrap();
        assert!(r.all);
    }

    #[test]
    fn test_exact_prefix_length() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("host".to_string(), Value::String("single".to_string()));
        tree.insert_value("10.0.0.1/32".parse().unwrap(), Value::Map(m))
            .unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            host: String,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.host, "single");
        assert!(reader.lookup::<TestRecord>("10.0.0.2".parse().unwrap()).is_err());
    }

    #[test]
    fn test_negative_int32() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("neg".to_string(), Value::Int32(-1));
        m.insert("min".to_string(), Value::Int32(i32::MIN));
        m.insert("max".to_string(), Value::Int32(i32::MAX));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            neg: i32,
            min: i32,
            max: i32,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.neg, -1);
        assert_eq!(r.min, i32::MIN);
        assert_eq!(r.max, i32::MAX);
    }

    #[test]
    fn test_float32_value() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("f".to_string(), Value::Float32(1.5));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            f: f32,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert!((r.f - 1.5).abs() < 1e-6);
    }

    #[test]
    fn test_map_with_many_keys() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        for i in 0..20 {
            m.insert(format!("key{i}").to_string(), Value::Uint16(i as u16));
        }
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct ManyKeys {}
        let _: ManyKeys = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
    }

    #[test]
    fn test_write_error_on_invalid_record_size() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 16,
            ..Default::default()
        };
        let result = Writer::new(opts);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_error_on_invalid_ip_version() {
        let result = Writer::new(WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 5,
            record_size: 24,
            ..Default::default()
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ip_version"));
    }

    #[test]
    fn test_write_error_on_invalid_record_size_in_new() {
        let result = Writer::new(WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 20,
            ..Default::default()
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("record_size"));
    }

    #[test]
    fn test_write_error_on_record_size_36() {
        let result = Writer::new(WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 36,
            ..Default::default()
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("record_size"));
    }

    #[test]
    fn test_write_error_ipv6_in_ipv4_tree() {
        let mut tree = Writer::new(WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        })
        .unwrap();
        let result = tree.insert_value("::1/128".parse().unwrap(), Value::Bool(true));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("IPv6"));
    }

    #[test]
    fn test_float64_value() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("f64".to_string(), Value::Float64(std::f64::consts::PI));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct TestRecord {
            f64: f64,
        }
        let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert!((r.f64 - std::f64::consts::PI).abs() < 1e-15);
    }

    #[test]
    fn test_bytes_value() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("raw".to_string(), Value::Bytes(vec![0xde, 0xad, 0xbe, 0xef]));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        assert!(reader.metadata.node_count > 0);
    }

    #[test]
    fn test_pointer_encoding_size2() {
        let v = Value::Pointer(1000000);
        let buf = encoder::encode_value(&v).unwrap();
        assert_eq!((buf[0] >> 5) & 0x07, 0b001);
        assert_eq!((buf[0] >> 3) & 0x03, 0b10);
        assert_eq!(buf.len(), 4);
    }

    #[test]
    fn test_metadata_size_limit() {
        let mut desc = BTreeMap::new();
        desc.insert("en".to_string(), "x".repeat(65536));
        let opts = WriterOptions {
            database_type: "LargeMeta".to_string(),
            ip_version: 4,
            record_size: 24,
            description: desc,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("x".to_string(), Value::Uint16(1));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let marker = b"\xab\xcd\xefMaxMind.com";
        let marker_pos = buf.windows(marker.len()).rposition(|w| w == marker).unwrap();
        let metadata_section_size = buf.len() - marker_pos;
        assert!(
            metadata_section_size <= 128 * 1024,
            "metadata section {} bytes exceeds 128 KiB limit",
            metadata_section_size
        );
    }

    #[test]
    fn test_encoded_size_matches_actual() {
        let cases: Vec<Value> = vec![
            Value::Pointer(0),
            Value::Pointer(2047),
            Value::Pointer(2048),
            Value::Pointer(526335),
            Value::Pointer(526336),
            Value::Pointer(134744063),
            Value::Pointer(134744064),
            Value::Pointer(u32::MAX),
            Value::String("hello".to_string()),
            Value::String("x".repeat(29)),
            Value::String("x".repeat(300)),
            Value::Float64(0.0),
            Value::Float64(-1.5),
            Value::Float64(f64::MAX),
            Value::Bytes(vec![]),
            Value::Bytes(vec![0; 42]),
            Value::Uint16(0),
            Value::Uint16(1),
            Value::Uint16(u16::MAX),
            Value::Uint32(0),
            Value::Uint32(1 << 20),
            Value::Uint32(u32::MAX),
            Value::Int32(0),
            Value::Int32(1),
            Value::Int32(-1),
            Value::Int32(i32::MIN),
            Value::Int32(i32::MAX),
            Value::Uint64(0),
            Value::Uint64(1 << 56),
            Value::Uint64(u64::MAX),
            Value::Uint128(0),
            Value::Uint128(1u128 << 120),
            Value::Uint128(u128::MAX),
            Value::Bool(true),
            Value::Bool(false),
            Value::Float32(0.0),
            Value::Float32(1.25),
            Value::Float32(f32::MAX),
        ];

        for v in cases {
            let encoded = encoder::encode_value(&v).unwrap();
            let size = encoder::encoded_size(&v);
            assert_eq!(
                encoded.len(),
                size,
                "encoded_size mismatch for {v:?}: got {} encoded but size said {size}",
                encoded.len()
            );
        }
    }

    #[test]
    fn test_encoded_size_matches_actual_empty_string() {
        let v = Value::String(String::new());
        let encoded = encoder::encode_value(&v).unwrap();
        assert_eq!(encoded.len(), encoder::encoded_size(&v));
    }

    #[test]
    fn test_values_after_insert_and_write_are_preserved() {
        let v = Value::String("hello".to_string());

        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        tree.insert_value("0.0.0.0/0".parse().unwrap(), v.clone()).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        let val: Value = reader.lookup("1.2.3.4".parse().unwrap()).unwrap();
        assert_eq!(val, v);
    }

    #[test]
    fn test_u128_roundtrip() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        let big = 0xdeadbeefcafebabeu128 << 64 | 0x1234567890abcdefu128;
        m.insert("v".to_string(), Value::Uint128(big));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct R {
            v: u128,
        }
        let r: R = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.v, big);
    }

    #[test]
    fn test_zero_values() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut m = BTreeMap::new();
        m.insert("u16".to_string(), Value::Uint16(0));
        m.insert("u32".to_string(), Value::Uint32(0));
        m.insert("u64".to_string(), Value::Uint64(0));
        m.insert("u128".to_string(), Value::Uint128(0));
        tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct Zeros {
            u16: u16,
            u32: u32,
            u64: u64,
            u128: u128,
        }
        let r: Zeros = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(r.u16, 0);
        assert_eq!(r.u32, 0);
        assert_eq!(r.u64, 0);
        assert_eq!(r.u128, 0);
    }

    #[test]
    fn test_insert_with_serialize_struct() {
        #[derive(serde::Serialize)]
        struct Record {
            country: String,
        }

        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        tree.insert(
            "1.0.0.0/8".parse().unwrap(),
            Record {
                country: "US".into(),
            },
        )
        .unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct Record2 {
            country: String,
        }
        let result: Record2 = reader.lookup("1.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(result.country, "US");
    }

    #[test]
    fn test_insert_with_serialize_btreemap() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let mut map = BTreeMap::new();
        map.insert("value".to_string(), "hello".to_string());
        tree.insert("10.0.0.0/8".parse().unwrap(), map).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();
        let reader = crate::Reader::from_source(buf).unwrap();
        #[derive(serde::Deserialize, Debug)]
        struct R {
            value: String,
        }
        let result: R = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(result.value, "hello");
    }

    #[test]
    fn test_insert_rejects_negative_out_of_range_integer() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let err = tree.insert("10.0.0.0/8".parse().unwrap(), i64::MIN).unwrap_err();
        assert!(err.contains("out of range"), "unexpected error: {err}");
    }

    #[test]
    fn test_insert_accepts_large_positive_integer_as_u64() {
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        let big = i64::MAX;
        tree.insert("10.0.0.0/8".parse().unwrap(), big).unwrap();
        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let reader = crate::Reader::from_source(buf).unwrap();
        // The value is encoded as a bare u64, so decode it as such.
        let decoded: u64 = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
        assert_eq!(decoded, big as u64);
    }

    #[test]
    fn test_encode_rejects_oversized_payload() {
        let too_big = crate::encoder::Value::String("x".repeat(16_843_037));
        let err = crate::encoder::encode_value(&too_big).unwrap_err();
        assert!(err.contains("too large"), "unexpected error: {err}");
    }

    #[test]
    fn test_metadata_size_limit_is_enforced() {
        let mut description = BTreeMap::new();
        // One description string above the 128 KiB metadata budget.
        description.insert("en".to_string(), "x".repeat(200 * 1024));
        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size: 24,
            description,
            languages: vec!["en".to_string()],
            build_epoch: 1,
        };
        let mut tree = Writer::new(opts).unwrap();
        tree.insert("10.0.0.0/8".parse().unwrap(), "v").unwrap();
        let err = tree.write_to(&mut Vec::new()).unwrap_err();
        assert!(err.contains("metadata"), "unexpected error: {err}");
    }
}
