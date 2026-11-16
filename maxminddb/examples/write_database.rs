use std::collections::BTreeMap;

use maxminddb::{
    Decode, Encode, Reader, Value,
    writer::{Writer, WriterOptions},
};

#[derive(Debug, Clone, Decode, Encode)]
struct Record<'a> {
    code: &'a str,
    name: &'a str,
}

fn main() -> Result<(), String> {
    let opts = WriterOptions {
        database_type: "My-Example-DB".to_string(),
        ip_version: 4,
        record_size: 24,
        description: {
            let mut m = BTreeMap::new();
            m.insert("en".to_string(), "Example database".to_string());
            m
        },
        ..Default::default()
    };

    let mut tree = Writer::new(opts).unwrap();

    // insert_value — accepts a raw Value, built with the map! macro
    tree.insert_value(
        "1.2.3.0/24".parse().unwrap(),
        maxminddb::map! {
            "code" => "US",
            "name" => "United States",
        },
    )?;

    let uk = Record {
        code: "GB",
        name: "United Kingdom",
    };

    // insert — accepts any Encode value, e.g. a BTreeMap<String, String>
    tree.insert("5.6.7.0/24".parse().unwrap(), &uk)?;

    let mut buf = Vec::new();
    tree.write_to(&mut buf)?;

    let reader = Reader::from_source(buf).unwrap();

    let us: Record = reader.lookup("1.2.3.4".parse().unwrap()).unwrap();
    println!("1.2.3.4 -> {us:?}");

    let gb: Value = reader.lookup("5.6.7.8".parse().unwrap()).unwrap();
    println!("5.6.7.8 -> {gb:?}");

    Ok(())
}
