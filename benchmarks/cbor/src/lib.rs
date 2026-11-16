//! Shared fixtures for the CBOR benchmark suite.

use serde::{Deserialize, Serialize};

/// A structured telemetry record, serde/cbor text-key map flavor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, cbor::Encode, cbor::Decode)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: u8,
    pub target: String,
    pub message: String,
    pub line: u32,
    pub success: bool,
    pub latency_ms: f64,
    pub labels: Vec<String>,
}

/// The same record as a positional array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, cbor::Encode, cbor::Decode)]
#[cbor(array)]
pub struct ArrayEntry(
    pub u64,
    pub u8,
    pub String,
    pub String,
    pub u32,
    pub bool,
    pub f64,
    pub Vec<String>,
);

/// A COSE-style integer-keyed record.
#[derive(Clone, Debug, PartialEq, cbor::Encode, cbor::Decode)]
pub struct IntEntry {
    #[cbor(key = 1)]
    pub alg: i64,
    #[cbor(key = 4)]
    #[cbor(bytes)]
    pub kid: Vec<u8>,
    #[cbor(key = 5)]
    #[cbor(bytes)]
    pub iv: Vec<u8>,
    #[cbor(key = 6)]
    #[cbor(bytes)]
    pub payload: Vec<u8>,
    #[cbor(key = 7)]
    pub count: u64,
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

const TARGETS: [&str; 6] = [
    "net::http::server",
    "db::pool",
    "auth::token",
    "cbor::de",
    "runtime::worker",
    "telemetry::export",
];

const WORDS: [&str; 8] = [
    "request",
    "completed",
    "timeout",
    "retry",
    "cache",
    "miss",
    "queued",
    "flush",
];

const LABELS: [&str; 8] = ["prod", "region=eu", "shard=3", "tls", "h2", "ipv6", "warm", "canary"];

pub fn logs(n: usize) -> Vec<LogEntry> {
    let mut state = 0xC0DE_CAFE;
    (0..n)
        .map(|i| {
            let r = splitmix(&mut state);
            let words = 2 + (r >> 8) as usize % 4;
            let mut message = String::new();
            for w in 0..words {
                if w > 0 {
                    message.push(' ');
                }
                message.push_str(WORDS[(splitmix(&mut state) as usize) % WORDS.len()]);
            }
            let label_count = (r >> 16) as usize % 4;
            let labels = (0..label_count)
                .map(|k| LABELS[(splitmix(&mut state) as usize + k) % LABELS.len()].to_string())
                .collect();
            LogEntry {
                timestamp: 1_700_000_000_000 + i as u64 * 37,
                level: (r >> 24) as u8 % 5,
                target: TARGETS[(r as usize) % TARGETS.len()].to_string(),
                message,
                line: (r >> 32) as u32 % 4096,
                success: r & 1 == 0,
                latency_ms: (r >> 40) as f64 / 1024.0,
                labels,
            }
        })
        .collect()
}

pub fn array_entries(logs: &[LogEntry]) -> Vec<ArrayEntry> {
    logs.iter()
        .map(|log| {
            ArrayEntry(
                log.timestamp,
                log.level,
                log.target.clone(),
                log.message.clone(),
                log.line,
                log.success,
                log.latency_ms,
                log.labels.clone(),
            )
        })
        .collect()
}

pub fn int_entries(n: usize) -> Vec<IntEntry> {
    (0..n)
        .map(|i| IntEntry {
            alg: -7,
            kid: vec![i as u8; 8],
            iv: vec![0x5a; 12],
            payload: (0..64).map(|x| (x as u8).wrapping_add(i as u8)).collect(),
            count: (i as u64) << 32,
        })
        .collect()
}

pub fn blob() -> Vec<u8> {
    (0..4096).map(|i| (i * 37) as u8).collect()
}

// Hand-written minicbor implementations matching the text-key map and the
// integer-keyed map, so comparisons use identical wire shapes.

impl minicbor::Encode<()> for LogEntry {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _: &mut (),
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(8)?;
        e.str("timestamp")?.u64(self.timestamp)?;
        e.str("level")?.u8(self.level)?;
        e.str("target")?.str(&self.target)?;
        e.str("message")?.str(&self.message)?;
        e.str("line")?.u32(self.line)?;
        e.str("success")?.bool(self.success)?;
        e.str("latency_ms")?.f64(self.latency_ms)?;
        e.str("labels")?.array(self.labels.len() as u64)?;
        for label in &self.labels {
            e.str(label)?;
        }
        Ok(())
    }
}

impl<'b> minicbor::Decode<'b, ()> for LogEntry {
    fn decode(d: &mut minicbor::Decoder<'b>, _: &mut ()) -> Result<Self, minicbor::decode::Error> {
        let len = d.map()?.unwrap();
        let mut out = LogEntry {
            timestamp: 0,
            level: 0,
            target: String::new(),
            message: String::new(),
            line: 0,
            success: false,
            latency_ms: 0.0,
            labels: Vec::new(),
        };
        for _ in 0..len {
            match d.str()? {
                "timestamp" => out.timestamp = d.u64()?,
                "level" => out.level = d.u8()?,
                "target" => out.target = d.str()?.to_owned(),
                "message" => out.message = d.str()?.to_owned(),
                "line" => out.line = d.u32()?,
                "success" => out.success = d.bool()?,
                "latency_ms" => out.latency_ms = d.f64()?,
                "labels" => {
                    let n = d.array()?.unwrap();
                    out.labels.reserve(n as usize);
                    for _ in 0..n {
                        out.labels.push(d.str()?.to_owned());
                    }
                }
                _ => d.skip()?,
            }
        }
        Ok(out)
    }
}

impl minicbor::Encode<()> for IntEntry {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _: &mut (),
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(5)?;
        e.u8(1)?.i64(self.alg)?;
        e.u8(4)?.bytes(&self.kid)?;
        e.u8(5)?.bytes(&self.iv)?;
        e.u8(6)?.bytes(&self.payload)?;
        e.u8(7)?.u64(self.count)?;
        Ok(())
    }
}

impl<'b> minicbor::Decode<'b, ()> for IntEntry {
    fn decode(d: &mut minicbor::Decoder<'b>, _: &mut ()) -> Result<Self, minicbor::decode::Error> {
        let len = d.map()?.unwrap();
        let mut out = IntEntry {
            alg: 0,
            kid: Vec::new(),
            iv: Vec::new(),
            payload: Vec::new(),
            count: 0,
        };
        for _ in 0..len {
            match d.u64()? {
                1 => out.alg = d.i64()?,
                4 => out.kid = d.bytes()?.to_vec(),
                5 => out.iv = d.bytes()?.to_vec(),
                6 => out.payload = d.bytes()?.to_vec(),
                7 => out.count = d.u64()?,
                _ => d.skip()?,
            }
        }
        Ok(out)
    }
}

/// A hand-written text-map implementation, used to separate core decoder
/// performance from derive-generated overhead.
#[derive(Clone, Debug, PartialEq)]
pub struct ManualLog(pub LogEntry);

impl cbor::Encode for ManualLog {
    fn encode<W: cbor::Write>(&self, e: &mut cbor::Encoder<W>) -> Result<(), W::Error> {
        let r = &self.0;
        e.map(8)?;
        e.text("timestamp")?;
        e.u64(r.timestamp)?;
        e.text("level")?;
        e.u8(r.level)?;
        e.text("target")?;
        e.text(&r.target)?;
        e.text("message")?;
        e.text(&r.message)?;
        e.text("line")?;
        e.u32(r.line)?;
        e.text("success")?;
        e.bool(r.success)?;
        e.text("latency_ms")?;
        e.f64(r.latency_ms)?;
        e.text("labels")?;
        e.array(r.labels.len())?;
        for label in &r.labels {
            e.text(label)?;
        }
        Ok(())
    }
}

impl<'b> cbor::Decode<'b> for ManualLog {
    fn decode(d: &mut cbor::Decoder<'b>) -> cbor::Result<Self> {
        let len = d.map()?.unwrap();
        let mut out = LogEntry {
            timestamp: 0,
            level: 0,
            target: String::new(),
            message: String::new(),
            line: 0,
            success: false,
            latency_ms: 0.0,
            labels: Vec::new(),
        };
        for _ in 0..len {
            match d.text()? {
                "timestamp" => out.timestamp = d.u64()?,
                "level" => out.level = d.u8()?,
                "target" => out.target = d.text()?.to_owned(),
                "message" => out.message = d.text()?.to_owned(),
                "line" => out.line = d.u32()?,
                "success" => out.success = d.bool()?,
                "latency_ms" => out.latency_ms = d.f64()?,
                "labels" => {
                    let n = d.array()?.unwrap();
                    out.labels.reserve(n);
                    for _ in 0..n {
                        out.labels.push(d.text()?.to_owned());
                    }
                }
                _ => d.skip()?,
            }
        }
        Ok(ManualLog(out))
    }
}

/// Hand-written integer-keyed decoder, to isolate derive overhead.
#[derive(Clone, Debug, PartialEq)]
pub struct ManualInt(pub IntEntry);

impl cbor::Encode for ManualInt {
    fn encode<W: cbor::Write>(&self, e: &mut cbor::Encoder<W>) -> Result<(), W::Error> {
        let r = &self.0;
        e.map(5)?;
        e.u8(1)?;
        e.i64(r.alg)?;
        e.u8(4)?;
        e.bytes(&r.kid)?;
        e.u8(5)?;
        e.bytes(&r.iv)?;
        e.u8(6)?;
        e.bytes(&r.payload)?;
        e.u8(7)?;
        e.u64(r.count)?;
        Ok(())
    }
}

impl<'b> cbor::Decode<'b> for ManualInt {
    fn decode(d: &mut cbor::Decoder<'b>) -> cbor::Result<Self> {
        let len = d.map()?.unwrap();
        let mut out = IntEntry {
            alg: 0,
            kid: Vec::new(),
            iv: Vec::new(),
            payload: Vec::new(),
            count: 0,
        };
        for _ in 0..len {
            match d.u64()? {
                1 => out.alg = d.i64()?,
                4 => out.kid = cbor::DecodeBytes::decode_bytes(d)?,
                5 => out.iv = cbor::DecodeBytes::decode_bytes(d)?,
                6 => out.payload = cbor::DecodeBytes::decode_bytes(d)?,
                7 => out.count = d.u64()?,
                _ => d.skip()?,
            }
        }
        Ok(ManualInt(out))
    }
}
