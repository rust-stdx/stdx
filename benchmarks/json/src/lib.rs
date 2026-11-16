//! Reproducible fixtures for the JSON benchmarks.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, json::Decode)]
pub struct Tweet {
    pub statuses: Vec<Status>,
}

#[derive(Debug, Clone, Deserialize, json::Decode)]
pub struct Status {
    pub created_at: String,
    pub id: u64,
    pub text: String,
    pub retweet_count: u32,
    pub favorite_count: u32,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize, json::Decode)]
pub struct User {
    pub id: u64,
    pub screen_name: String,
}

#[derive(Debug, Clone, Deserialize, json::Decode)]
pub struct Canada {
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Deserialize, json::Decode)]
pub struct Feature {
    pub geometry: Geometry,
}

#[derive(Debug, Clone, Deserialize, json::Decode)]
pub struct Geometry {
    pub coordinates: Vec<Vec<[f64; 2]>>,
}

fn escape_into(out: &mut String, text: &str) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            ch => out.push(ch),
        }
    }
    out.push('"');
}

/// A twitter-like document with nested objects, arrays, Unicode and escapes.
pub fn twitter() -> String {
    let mut out = String::with_capacity(600_000);
    out.push_str("{\"statuses\":[");
    for i in 0..2_000 {
        if i != 0 {
            out.push(',');
        }
        out.push_str("{\"created_at\":\"2026-01-01T00:00:00Z\",\"id\":");
        out.push_str(&i.to_string());
        out.push_str(",\"text\":");
        escape_into(&mut out, &format!("hello \"{i}\"\n日本語 {}", i % 17));
        out.push_str(",\"retweet_count\":");
        out.push_str(&(i % 100).to_string());
        out.push_str(",\"favorite_count\":");
        out.push_str(&(i % 50).to_string());
        out.push_str(",\"user\":{\"id\":");
        out.push_str(&(1_000_000 + i).to_string());
        out.push_str(",\"screen_name\":");
        escape_into(&mut out, &format!("user_{i}"));
        out.push_str("}}");
    }
    out.push_str("]}");
    out
}

/// A GeoJSON-like document with many floating-point coordinates.
pub fn canada() -> String {
    let mut out = String::with_capacity(800_000);
    out.push_str("{\"features\":[{\"geometry\":{\"coordinates\":[[");
    for i in 0..20_000 {
        if i != 0 {
            out.push(',');
        }
        out.push('[');
        out.push_str(&format!("{}.{}", 40 + (i % 30), i % 1000));
        out.push(',');
        out.push_str(&format!("{}.{}", -140 + (i % 50), i % 997));
        out.push(']');
    }
    out.push_str("]]}}]}");
    out
}

/// A CITM-like document with nested objects, short strings and integers.
pub fn citm() -> String {
    let mut out = String::with_capacity(600_000);
    out.push_str("{\"areaNames\":{");
    for i in 0..1_000 {
        if i != 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&i.to_string());
        out.push_str("\":");
        escape_into(&mut out, &format!("Area {i}"));
    }
    out.push_str("},\"events\":{");
    for i in 0..1_000 {
        if i != 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&(10_000 + i).to_string());
        out.push_str("\":{\"id\":");
        out.push_str(&i.to_string());
        out.push_str(",\"name\":");
        escape_into(&mut out, &format!("Event {i}"));
        out.push_str(",\"prices\":[");
        for j in 0..8 {
            if j != 0 {
                out.push(',');
            }
            out.push_str(&(j * 5 + i % 3).to_string());
        }
        out.push_str("]}");
    }
    out.push_str("}}");
    out
}

/// Many short strings.
pub fn strings() -> String {
    let mut out = String::new();
    out.push('[');
    for i in 0..100_000 {
        if i != 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(&format!("string-number-{i}"));
        out.push('"');
    }
    out.push(']');
    out
}

/// Many escaped strings.
pub fn escaped() -> String {
    let mut out = String::new();
    out.push('[');
    for i in 0..50_000 {
        if i != 0 {
            out.push(',');
        }
        out.push_str("\"a\\\\nb\\\"");
        out.push_str(&i.to_string());
        out.push('"');
    }
    out.push(']');
    out
}

/// Many integers.
pub fn ints() -> String {
    let mut out = String::new();
    out.push('[');
    for i in 0..200_000 {
        if i != 0 {
            out.push(',');
        }
        out.push_str(&i.to_string());
    }
    out.push(']');
    out
}

/// Many floats.
pub fn floats() -> String {
    let mut out = String::new();
    out.push('[');
    for i in 0..100_000 {
        if i != 0 {
            out.push(',');
        }
        out.push_str(&format!("{}.{}", i, i % 1000));
    }
    out.push(']');
    out
}
