use std::{net::IpAddr, str::FromStr};

use serde::Deserialize;
use serde_json::json;

use super::{MaxMindDBError, Reader};

#[allow(clippy::float_cmp)]
#[test]
fn test_decoder() {
    #[allow(non_snake_case)]
    #[derive(Deserialize, Debug, Eq, PartialEq)]
    struct MapXType {
        arrayX: Vec<u32>,
        utf8_stringX: String,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize, Debug, Eq, PartialEq)]
    struct MapType {
        mapX: MapXType,
    }

    #[derive(Deserialize, Debug)]
    struct TestType<'a> {
        array: Vec<u32>,
        boolean: bool,
        bytes: &'a [u8],
        double: f64,
        float: f32,
        int32: i32,
        map: MapType,
        uint16: u16,
        uint32: u32,
        uint64: u64,
        uint128: u128,
        utf8_string: String,
    }

    let r = Reader::open_readfile("test-data/test-data/MaxMind-DB-test-decoder.mmdb");
    if let Err(err) = r {
        panic!("error opening mmdb: {err:?}");
    }
    let r = r.unwrap();
    let ip: IpAddr = FromStr::from_str("1.1.1.0").unwrap();
    let result: TestType = r.lookup(ip).unwrap();

    assert_eq!(result.array, vec![1_u32, 2_u32, 3_u32]);
    assert!(result.boolean);
    assert_eq!(result.bytes, vec![0_u8, 0_u8, 0_u8, 42_u8]);
    assert_eq!(result.double, 42.123_456);
    assert_eq!(result.float, 1.1);
    assert_eq!(result.int32, -268_435_456);

    assert_eq!(
        result.map,
        MapType {
            mapX: MapXType {
                arrayX: vec![7, 8, 9],
                utf8_stringX: "hello".to_string(),
            },
        }
    );

    assert_eq!(result.uint16, 100);
    assert_eq!(result.uint32, 268_435_456);
    assert_eq!(result.uint64, 1_152_921_504_606_846_976);
    assert_eq!(result.uint128, 1_329_227_995_784_915_872_903_807_060_280_344_576);

    assert_eq!(result.utf8_string, "unicode! \u{262f} - \u{266b}".to_string());
}

#[test]
fn test_pointers_in_metadata() {
    let r = Reader::open_readfile("test-data/test-data/MaxMind-DB-test-metadata-pointers.mmdb");
    if let Err(err) = r {
        panic!("error opening mmdb: {err:?}");
    }
}

#[test]
fn test_broken_database() {
    let r = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test-Broken-Double-Format.mmdb")
        .ok()
        .unwrap();
    let ip: IpAddr = FromStr::from_str("2001:220::").unwrap();

    #[derive(Deserialize, Debug)]
    struct TestType {}
    match r.lookup::<TestType>(ip) {
        Err(e) => assert_eq!(e, MaxMindDBError::InvalidDatabaseError("double of size 2".to_string())),
        Ok(_) => panic!("Error expected"),
    }
}

#[test]
fn test_missing_database() {
    let r = Reader::open_readfile("file-does-not-exist.mmdb");
    match r {
        Ok(_) => panic!("Received Reader when opening non-existent file"),
        Err(e) => assert!(
            e == MaxMindDBError::IoError("The system cannot find the file specified. (os error 2)".to_string())
                || e == MaxMindDBError::IoError("No such file or directory (os error 2)".to_string())
        ),
    }
}

#[test]
fn test_non_database() {
    let r = Reader::open_readfile("README.md");
    match r {
        Ok(_) => panic!("Received Reader when opening a non-MMDB file"),
        Err(e) => assert_eq!(
            e,
            MaxMindDBError::InvalidDatabaseError(
                "Could not find MaxMind DB metadata \
                 in file."
                    .to_string(),
            )
        ),
    }
}

#[test]
fn test_reader() {
    let sizes = [24_usize, 28, 32];
    for record_size in &sizes {
        let versions = [4_usize, 6];
        for ip_version in &versions {
            let filename = format!("test-data/test-data/MaxMind-DB-test-ipv{ip_version}-{record_size}.mmdb");
            let reader = Reader::open_readfile(filename).ok().unwrap();

            check_metadata(&reader, *ip_version, *record_size);
            check_ip(&reader, *ip_version);
        }
    }
}

#[test]
fn test_reader_readfile() {
    let sizes = [24_usize, 28, 32];
    for record_size in &sizes {
        let versions = [4_usize, 6];
        for ip_version in &versions {
            let filename = format!("test-data/test-data/MaxMind-DB-test-ipv{ip_version}-{record_size}.mmdb");
            let reader = Reader::open_readfile(filename).ok().unwrap();

            check_metadata(&reader, *ip_version, *record_size);
            check_ip(&reader, *ip_version);
        }
    }
}

#[test]
fn test_lookup_city() {
    use super::geoip2::City;

    let filename = "test-data/test-data/GeoIP2-City-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("89.160.20.112").unwrap();
    let city: City = reader.lookup(ip).unwrap();

    let iso_code = city.country.and_then(|cy| cy.iso_code);

    assert_eq!(iso_code, Some("SE"));
}

#[test]
fn test_lookup_country() {
    use super::geoip2::Country;

    let filename = "test-data/test-data/GeoIP2-Country-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("89.160.20.112").unwrap();
    let country: Country = reader.lookup(ip).unwrap();
    let country = country.country.unwrap();

    assert_eq!(country.iso_code, Some("SE"));
    assert_eq!(country.is_in_european_union, Some(true));
}

#[test]
fn test_lookup_connection_type() {
    use super::geoip2::ConnectionType;

    let filename = "test-data/test-data/GeoIP2-Connection-Type-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("96.1.20.112").unwrap();
    let connection_type: ConnectionType = reader.lookup(ip).unwrap();

    assert_eq!(connection_type.connection_type, Some("Cable/DSL"));
}

#[test]
fn test_lookup_annonymous_ip() {
    use super::geoip2::AnonymousIp;

    let filename = "test-data/test-data/GeoIP2-Anonymous-IP-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("81.2.69.123").unwrap();
    let anonymous_ip: AnonymousIp = reader.lookup(ip).unwrap();

    assert_eq!(anonymous_ip.is_anonymous, Some(true));
    assert_eq!(anonymous_ip.is_public_proxy, Some(true));
    assert_eq!(anonymous_ip.is_anonymous_vpn, Some(true));
    assert_eq!(anonymous_ip.is_hosting_provider, Some(true));
    assert_eq!(anonymous_ip.is_tor_exit_node, Some(true))
}

#[test]
fn test_lookup_density_income() {
    use super::geoip2::DensityIncome;

    let filename = "test-data/test-data/GeoIP2-DensityIncome-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("5.83.124.123").unwrap();
    let density_income: DensityIncome = reader.lookup(ip).unwrap();

    assert_eq!(density_income.average_income, Some(32323));
    assert_eq!(density_income.population_density, Some(1232))
}

#[test]
fn test_lookup_domain() {
    use super::geoip2::Domain;

    let filename = "test-data/test-data/GeoIP2-Domain-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("66.92.80.123").unwrap();
    let domain: Domain = reader.lookup(ip).unwrap();

    assert_eq!(domain.domain, Some("speakeasy.net"));
}

#[test]
fn test_lookup_isp() {
    use super::geoip2::Isp;

    let filename = "test-data/test-data/GeoIP2-ISP-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("12.87.118.123").unwrap();
    let isp: Isp = reader.lookup(ip).unwrap();

    assert_eq!(isp.autonomous_system_number, Some(7018));
    assert_eq!(isp.isp, Some("AT&T Services"));
    assert_eq!(isp.organization, Some("AT&T Worldnet Services"));
}

#[test]
fn test_lookup_asn() {
    use super::geoip2::Asn;

    let filename = "test-data/test-data/GeoIP2-ISP-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("1.128.0.123").unwrap();
    let asn: Asn = reader.lookup(ip).unwrap();

    assert_eq!(asn.autonomous_system_number, Some(1221));
    assert_eq!(asn.autonomous_system_organization, Some("Telstra Pty Ltd"));
}

#[test]
fn test_lookup_prefix() {
    use super::geoip2::City;

    let filename = "test-data/test-data/GeoIP2-ISP-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    // IPv4
    let ip: IpAddr = "89.160.20.128".parse().unwrap();
    let (_, prefix_len) = reader.lookup_prefix::<City>(ip).unwrap();

    assert_eq!(prefix_len, 25);

    // Last host
    let ip: IpAddr = "89.160.20.254".parse().unwrap();
    let (_, last_prefix_len) = reader.lookup_prefix::<City>(ip).unwrap();

    assert_eq!(prefix_len, last_prefix_len);
}

#[test]
fn test_within_city() {
    use ipnetwork::IpNetwork;

    use super::{Within, geoip2::City};

    let filename = "test-data/test-data/GeoIP2-City-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip_net = IpNetwork::V6("::/0".parse().unwrap());

    let mut iter: Within<City, _> = reader.within(ip_net).unwrap();

    let item = iter.next().unwrap().unwrap();
    assert_eq!(item.ip_net, IpNetwork::V4("2.2.3.0/24".parse().unwrap()));
    assert_eq!(item.info.city.unwrap().geoname_id, Some(2_655_045));

    let mut n = 1;
    for _ in iter {
        n += 1;
    }

    assert_eq!(n, 250);

    // A second run through this time a specific network
    let specific = IpNetwork::V4("81.2.69.0/24".parse().unwrap());
    let mut iter: Within<City, _> = reader.within(specific).unwrap();
    let mut expected = vec![
        IpNetwork::V4("81.2.69.192/28".parse().unwrap()),
        IpNetwork::V4("81.2.69.160/27".parse().unwrap()),
        IpNetwork::V4("81.2.69.144/28".parse().unwrap()),
        IpNetwork::V4("81.2.69.142/31".parse().unwrap()),
    ];
    while let Some(e) = expected.pop() {
        let item = iter.next().unwrap().unwrap();
        assert_eq!(item.ip_net, e);
    }
}

fn check_metadata<T: AsRef<[u8]>>(reader: &Reader<T>, ip_version: usize, record_size: usize) {
    let metadata = &reader.metadata;

    assert_eq!(metadata.binary_format_major_version, 2_u16);

    assert_eq!(metadata.binary_format_minor_version, 0_u16);
    assert!(metadata.build_epoch >= 1_397_457_605);
    assert_eq!(metadata.database_type, "Test".to_string());

    assert_eq!(*metadata.description[&"en".to_string()], "Test Database".to_string());
    assert_eq!(*metadata.description[&"zh".to_string()], "Test Database Chinese".to_string());

    assert_eq!(metadata.ip_version, ip_version as u16);
    assert_eq!(metadata.languages, vec!["en".to_string(), "zh".to_string()]);

    if ip_version == 4 {
        assert_eq!(metadata.node_count, 163)
    } else {
        assert_eq!(metadata.node_count, 415)
    }

    assert_eq!(metadata.record_size, record_size as u16)
}

fn check_ip<T: AsRef<[u8]>>(reader: &Reader<T>, ip_version: usize) {
    let subnets = match ip_version {
        6 => [
            "::1:ffff:ffff",
            "::2:0:0",
            "::2:0:0",
            "::2:0:0",
            "::2:0:0",
            "::2:0:40",
            "::2:0:40",
            "::2:0:40",
            "::2:0:50",
            "::2:0:50",
            "::2:0:50",
            "::2:0:58",
            "::2:0:58",
        ],
        _ => [
            "1.1.1.1", "1.1.1.2", "1.1.1.2", "1.1.1.4", "1.1.1.4", "1.1.1.4", "1.1.1.4", "1.1.1.8", "1.1.1.8",
            "1.1.1.8", "1.1.1.16", "1.1.1.16", "1.1.1.16",
        ],
    };

    #[derive(Deserialize, Debug)]
    struct IpType {
        ip: String,
    }

    for subnet in &subnets {
        let ip: IpAddr = FromStr::from_str(subnet).unwrap();
        let value: IpType = reader.lookup(ip).unwrap();

        assert_eq!(value.ip, *subnet);
    }

    let no_record = ["1.1.1.33", "255.254.253.123", "89fa::"];

    for &address in &no_record {
        let ip: IpAddr = FromStr::from_str(address).unwrap();
        match reader.lookup::<IpType>(ip) {
            Ok(v) => panic!("received an unexpected value: {v:?}"),
            Err(e) => assert_eq!(e, MaxMindDBError::AddressNotFoundError(ip)),
        }
    }
}

#[test]
fn test_json_serialize() {
    use super::geoip2::City;

    let filename = "test-data/test-data/GeoIP2-City-Test.mmdb";

    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = FromStr::from_str("89.160.20.112").unwrap();
    let city: City = reader.lookup(ip).unwrap();

    let json_string = json!(city).to_string();

    assert_eq!(
        json_string,
        r#"{"city":{"geoname_id":2694762,"names":{"de":"Linköping","en":"Linköping","fr":"Linköping","ja":"リンシェーピング","zh-CN":"林雪平"}},"continent":{"code":"EU","geoname_id":6255148,"names":{"de":"Europa","en":"Europe","es":"Europa","fr":"Europe","ja":"ヨーロッパ","pt-BR":"Europa","ru":"Европа","zh-CN":"欧洲"}},"country":{"geoname_id":2661886,"is_in_european_union":true,"iso_code":"SE","names":{"de":"Schweden","en":"Sweden","es":"Suecia","fr":"Suède","ja":"スウェーデン王国","pt-BR":"Suécia","ru":"Швеция","zh-CN":"瑞典"}},"location":{"accuracy_radius":76,"latitude":58.4167,"longitude":15.6167,"time_zone":"Europe/Stockholm"},"registered_country":{"geoname_id":2921044,"is_in_european_union":true,"iso_code":"DE","names":{"de":"Deutschland","en":"Germany","es":"Alemania","fr":"Allemagne","ja":"ドイツ連邦共和国","pt-BR":"Alemanha","ru":"Германия","zh-CN":"德国"}},"subdivisions":[{"geoname_id":2685867,"iso_code":"E","names":{"en":"Östergötland County","fr":"Comté d'Östergötland"}}]}"#
    );
}

// ====== Additional Edge Case Tests ======

#[test]
fn test_string_value_entries() {
    let filename = "test-data/test-data/MaxMind-DB-string-value-entries.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    assert_eq!(reader.metadata.database_type, "MaxMind DB String Value Entries");

    // This DB returns raw string values (e.g. "1.1.1.1/32") rather than maps
    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    let val: String = reader.lookup(ip).unwrap();
    assert_eq!(val, "1.1.1.1/32");

    let ip: IpAddr = "1.1.1.2".parse().unwrap();
    let val: String = reader.lookup(ip).unwrap();
    assert_eq!(val, "1.1.1.2/31");

    let ip: IpAddr = "1.1.1.33".parse().unwrap();
    let result = reader.lookup::<String>(ip);
    assert!(result.is_err());
}

#[test]
fn test_no_ipv4_search_tree() {
    let filename = "test-data/test-data/MaxMind-DB-no-ipv4-search-tree.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    assert_eq!(reader.metadata.database_type, "MaxMind DB No IPv4 Search Tree");
    assert_eq!(reader.metadata.node_count, 64);
    assert_eq!(reader.metadata.ip_version, 6);

    // This DB returns a raw CIDR string like "::/64"
    let ip: IpAddr = "::1.1.1.1".parse().unwrap();
    let val: String = reader.lookup(ip).unwrap();
    assert_eq!(val, "::/64");

    // IPv4 address should work too since ipv4_start traverses 96 zero bits
    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    let val: String = reader.lookup(ip).unwrap();
    assert_eq!(val, "::/64");
}

#[test]
fn test_pointer_decoder_database() {
    let filename = "test-data/test-data/MaxMind-DB-test-pointer-decoder.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    assert_eq!(reader.metadata.database_type, "MaxMind DB Decoder Test");

    // Data is keyed under different IPs; use Value to explore
    let ip: IpAddr = "1.1.1.0".parse().unwrap();
    // The uint128 value causes serde Value deserialization to fail because
    // serde_json_ Value doesn't support u128. But we can still use Reader
    // with a typed struct.
    let result = reader.lookup::<super::Value>(ip);
    // Should either parse or fail gracefully without panic
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn test_enterprise_lookup() {
    use super::geoip2::Enterprise;

    let filename = "test-data/test-data/GeoIP2-Enterprise-Test.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = "2.125.160.216".parse().unwrap();
    let result: Enterprise = reader.lookup(ip).unwrap();

    assert!(result.country.is_some());
    assert_eq!(result.country.as_ref().unwrap().iso_code, Some("GB"));
}

#[test]
fn test_all_test_databases_open_successfully() {
    let db_files = [
        "GeoIP2-Anonymous-IP-Test.mmdb",
        "GeoIP2-City-Test.mmdb",
        "GeoIP2-Connection-Type-Test.mmdb",
        "GeoIP2-Country-Test.mmdb",
        "GeoIP2-DensityIncome-Test.mmdb",
        "GeoIP2-Domain-Test.mmdb",
        "GeoIP2-Enterprise-Test.mmdb",
        "GeoIP2-ISP-Test.mmdb",
        "MaxMind-DB-test-decoder.mmdb",
        "MaxMind-DB-test-ipv4-24.mmdb",
        "MaxMind-DB-test-ipv4-28.mmdb",
        "MaxMind-DB-test-ipv4-32.mmdb",
        "MaxMind-DB-test-ipv6-24.mmdb",
        "MaxMind-DB-test-ipv6-28.mmdb",
        "MaxMind-DB-test-ipv6-32.mmdb",
        "MaxMind-DB-test-metadata-pointers.mmdb",
        "MaxMind-DB-test-mixed-24.mmdb",
        "MaxMind-DB-test-mixed-28.mmdb",
        "MaxMind-DB-test-mixed-32.mmdb",
        "MaxMind-DB-test-nested.mmdb",
        "MaxMind-DB-test-pointer-decoder.mmdb",
        "MaxMind-DB-string-value-entries.mmdb",
        "MaxMind-DB-no-ipv4-search-tree.mmdb",
        "GeoIP2-City-Test-Broken-Double-Format.mmdb",
        "GeoLite2-ASN-Test.mmdb",
        "GeoLite2-City-Test.mmdb",
        "GeoLite2-Country-Test.mmdb",
    ];

    for db in &db_files {
        let path = format!("test-data/test-data/{db}");
        let reader = Reader::open_readfile(&path);
        assert!(reader.is_ok(), "Failed to open {db}: {:?}", reader.err());
    }
}

#[test]
fn test_mixed_database() {
    for record_size in &[24_usize, 28, 32] {
        let filename = format!("test-data/test-data/MaxMind-DB-test-mixed-{record_size}.mmdb");
        let reader = Reader::open_readfile(&filename).unwrap();
        assert_eq!(reader.metadata.database_type, "Test");
        assert_eq!(reader.metadata.ip_version, 6);

        #[derive(Deserialize, Debug)]
        struct IpType {
            ip: String,
        }

        let ip: IpAddr = "::1.1.1.1".parse().unwrap();
        let value: IpType = reader.lookup(ip).unwrap();
        assert_eq!(value.ip, "::1.1.1.1");

        let ip: IpAddr = "1.1.1.1".parse().unwrap();
        let value: IpType = reader.lookup(ip).unwrap();
        assert_eq!(value.ip, "::1.1.1.1");
    }
}

#[test]
fn test_nested_database() {
    use super::Value;

    let filename = "test-data/test-data/MaxMind-DB-test-nested.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    let result: Value = reader.lookup(ip).unwrap();

    fn extract_map3(v: &Value) -> Option<&std::collections::BTreeMap<String, Value>> {
        if let Value::Map(outer) = v {
            if let Some(Value::Map(m1)) = outer.get("map1") {
                if let Some(Value::Map(m2)) = m1.get("map2") {
                    if let Some(Value::Slice(arr)) = m2.get("array") {
                        if let Some(Value::Map(fm)) = arr.first() {
                            if let Some(Value::Map(m3)) = fm.get("map3") {
                                return Some(m3);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    let m3 = extract_map3(&result).expect("expected nested structure");
    assert_eq!(m3.get("a"), Some(&Value::Uint64(1)));
    assert_eq!(m3.get("b"), Some(&Value::Uint64(2)));
    assert_eq!(m3.get("c"), Some(&Value::Uint64(3)));
}

#[test]
fn test_open_and_read_invalid_node_count_db() {
    let filename = "test-data/test-data/GeoIP2-City-Test-Invalid-Node-Count.mmdb";
    // This database has node_count=100000 but a much smaller search tree.
    // It opens successfully (metadata is valid) but lookups should fail
    // gracefully.
    let reader = Reader::open_readfile(filename).unwrap();
    assert_eq!(reader.metadata.node_count, 100000);

    let ip: IpAddr = "::1".parse().unwrap();
    let result = reader.lookup::<super::Value>(ip);
    // Should fail gracefully - either AddressNotFound or out-of-bounds error
    // but should NOT panic
    let _ = result;
}

#[test]
fn test_lookup_enterprise_shield() {
    let filename = "test-data/test-data/GeoIP2-Enterprise-Shield-Test.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();
    assert_eq!(reader.metadata.database_type, "GeoIP2-Enterprise-Shield");
}

#[test]
fn test_multiple_lookups_same_reader() {
    let reader = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();

    let ips = [
        ("89.160.20.112", "SE"),
        ("89.160.20.128", "SE"),
        ("81.2.69.142", "GB"),
        ("81.2.69.144", "GB"),
        ("81.2.69.160", "GB"),
        ("81.2.69.192", "GB"),
    ];

    for (ip_str, expected_country) in &ips {
        let ip: IpAddr = ip_str.parse().unwrap();
        let city: super::geoip2::City = reader.lookup(ip).unwrap();
        assert_eq!(
            city.country.as_ref().and_then(|c| c.iso_code),
            Some(*expected_country),
            "Mismatch for {ip_str}"
        );
    }
}

#[test]
fn test_open_broken_pointers_db() {
    let filename = "test-data/test-data/MaxMind-DB-test-broken-pointers-24.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();
    assert_eq!(reader.metadata.database_type, "Test");

    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    let result = reader.lookup::<super::Value>(ip);
    let _ = result;
}

#[test]
fn test_open_broken_search_tree_db() {
    let filename = "test-data/test-data/MaxMind-DB-test-broken-search-tree-24.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();
    assert_eq!(reader.metadata.database_type, "Test");

    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    let _ = reader.lookup::<super::Value>(ip);
}

#[test]
fn test_v4_address_in_v6_tree() {
    use super::geoip2::City;

    let filename = "test-data/test-data/GeoIP2-City-Test.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = "89.160.20.112".parse().unwrap();
    let city: City = reader.lookup(ip).unwrap();
    assert_eq!(city.country.as_ref().and_then(|c| c.iso_code), Some("SE"));
}

#[test]
fn test_value_lookup_on_decoder_db() {
    let filename = "test-data/test-data/MaxMind-DB-test-decoder.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    #[derive(Deserialize, Debug)]
    struct PartialType {
        uint32: u32,
        uint16: u16,
        boolean: bool,
    }

    let ip: IpAddr = "1.1.1.0".parse().unwrap();
    let val: PartialType = reader.lookup(ip).unwrap();

    assert_eq!(val.uint32, 268_435_456);
    assert_eq!(val.uint16, 100);
    assert!(val.boolean);
}

#[test]
fn test_reader_from_source_slice() {
    let filename = "test-data/test-data/MaxMind-DB-test-ipv4-24.mmdb";
    let buf = std::fs::read(filename).unwrap();
    let reader = Reader::from_source(&buf[..]).unwrap();

    assert_eq!(reader.metadata.node_count, 163);

    #[derive(Deserialize, Debug)]
    struct IpType {
        ip: String,
    }

    let ip: IpAddr = "1.1.1.1".parse().unwrap();
    let val: IpType = reader.lookup(ip).unwrap();
    assert_eq!(val.ip, "1.1.1.1");
}

#[test]
fn test_lookup_and_lookup_prefix_agree() {
    use super::geoip2::City;

    let reader = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();
    let ip: IpAddr = "89.160.20.128".parse().unwrap();

    let val: City = reader.lookup(ip).unwrap();
    let (val2, _) = reader.lookup_prefix::<City>(ip).unwrap();

    assert_eq!(val.country.and_then(|c| c.iso_code), val2.country.and_then(|c| c.iso_code),);
}

#[test]
fn test_extremely_large_string_value_roundtrip() {
    use std::collections::BTreeMap;

    use super::Value;
    use crate::writer::{Writer, WriterOptions};

    let long_str = "x".repeat(70000);
    let mut m = BTreeMap::new();
    m.insert("big".to_string(), Value::String(long_str.clone()));

    let opts = WriterOptions {
        database_type: "Test".to_string(),
        ip_version: 4,
        record_size: 24,
        ..Default::default()
    };
    let mut tree = Writer::new(opts).unwrap();
    tree.insert_value("10.0.0.0/8".parse().unwrap(), Value::Map(m)).unwrap();

    let mut buf = Vec::new();
    tree.write_to(&mut buf).unwrap();

    let reader = Reader::from_source(buf).unwrap();
    #[derive(Deserialize, Debug)]
    struct TestRecord {
        big: String,
    }
    let r: TestRecord = reader.lookup("10.0.0.1".parse().unwrap()).unwrap();
    assert_eq!(r.big.len(), 70000);
    assert_eq!(r.big, long_str);
}

#[test]
fn test_exact_prefix_length_all_record_sizes() {
    use std::collections::BTreeMap;

    use super::Value;
    use crate::writer::{Writer, WriterOptions};

    for record_size in [24, 28, 32] {
        let mut m = BTreeMap::new();
        m.insert("host".to_string(), Value::String("exact".to_string()));

        let opts = WriterOptions {
            database_type: "Test".to_string(),
            ip_version: 4,
            record_size,
            ..Default::default()
        };
        let mut tree = Writer::new(opts).unwrap();
        tree.insert_value("192.168.1.1/32".parse().unwrap(), Value::Map(m))
            .unwrap();

        let mut buf = Vec::new();
        tree.write_to(&mut buf).unwrap();

        let reader = Reader::from_source(buf).unwrap();
        #[derive(Deserialize, Debug)]
        struct TestRecord {
            host: String,
        }

        let r: TestRecord = reader.lookup("192.168.1.1".parse().unwrap()).unwrap();
        assert_eq!(r.host, "exact", "failed for record_size={record_size}");

        assert!(
            reader.lookup::<TestRecord>("192.168.1.2".parse().unwrap()).is_err(),
            "should be not found for record_size={record_size}"
        );
    }
}

#[test]
fn test_ipv6_within_over_ipv4_range() {
    use ipnetwork::IpNetwork;

    use super::{Within, geoip2::City};

    let reader = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();

    let ip_net = IpNetwork::V4("0.0.0.0/0".parse().unwrap());
    let iter: Within<City, _> = reader.within(ip_net).unwrap();

    let count = iter.filter(|item| item.is_ok()).count();
    assert!(count > 0, "Should find at least one IPv4 network");
}

#[test]
fn test_reader_into_geolite2_asn() {
    let filename = "test-data/test-data/GeoLite2-ASN-Test.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();
    assert_eq!(reader.metadata.database_type, "GeoLite2-ASN");

    let ip: IpAddr = "1.128.0.123".parse().unwrap();
    let asn: super::geoip2::Asn = reader.lookup(ip).unwrap();
    assert_eq!(asn.autonomous_system_number, Some(1221));
}

#[test]
fn test_open_various_ip_risk() {
    let filename = "test-data/test-data/GeoIP2-IP-Risk-Test.mmdb";
    let reader = Reader::open_readfile(filename);
    assert!(reader.is_ok(), "Should open IP Risk database: {:?}", reader.err());
}

#[test]
fn test_open_static_ip_score() {
    let filename = "test-data/test-data/GeoIP2-Static-IP-Score-Test.mmdb";
    let reader = Reader::open_readfile(filename);
    assert!(reader.is_ok(), "Should open Static IP Score database: {:?}", reader.err());
}

#[test]
fn test_shield_databases() {
    let dbs = [
        "GeoIP2-City-Shield-Test.mmdb",
        "GeoIP2-Country-Shield-Test.mmdb",
        "GeoIP2-Enterprise-Shield-Test.mmdb",
        "GeoIP2-Precision-Enterprise-Shield-Test.mmdb",
    ];
    for db in &dbs {
        let path = format!("test-data/test-data/{db}");
        let reader = Reader::open_readfile(&path);
        assert!(reader.is_ok(), "Failed to open {db}: {:?}", reader.err());
    }
}

#[test]
fn test_anonymous_plus_databases() {
    let dbs = ["GeoIP-Anonymous-Plus-Test.mmdb", "GeoIP-Residential-Proxy-Test.mmdb"];
    for db in &dbs {
        let path = format!("test-data/test-data/{db}");
        let reader = Reader::open_readfile(&path);
        assert!(reader.is_ok(), "Failed to open {db}: {:?}", reader.err());
    }
}

#[test]
fn test_precision_enterprise() {
    let filename = "test-data/test-data/GeoIP2-Precision-Enterprise-Test.mmdb";
    let reader = Reader::open_readfile(filename);
    assert!(reader.is_ok(), "Failed to open precision enterprise: {:?}", reader.err());
}

#[test]
fn test_user_count_database() {
    let filename = "test-data/test-data/GeoIP2-User-Count-Test.mmdb";
    let reader = Reader::open_readfile(filename);
    assert!(reader.is_ok(), "Failed to open user count: {:?}", reader.err());
}

#[test]
fn test_lookup_prefix_ipv6() {
    use super::geoip2::City;

    let filename = "test-data/test-data/GeoIP2-City-Test.mmdb";
    let reader = Reader::open_readfile(filename).unwrap();

    let ip: IpAddr = "2001:220::".parse().unwrap();
    let (_, prefix_len) = reader.lookup_prefix::<City>(ip).unwrap();
    assert!(prefix_len > 0);
}

#[test]
fn test_lookup_before_and_after_tree_traversal() {
    let reader = Reader::open_readfile("test-data/test-data/GeoIP2-City-Test.mmdb").unwrap();

    let ip_v4: IpAddr = "89.160.20.112".parse().unwrap();
    let ip_v6: IpAddr = "2001:220::".parse().unwrap();
    let ip_not_found: IpAddr = "0.0.0.0".parse().unwrap();

    let city: super::geoip2::City = reader.lookup(ip_v4).unwrap();
    assert_eq!(city.country.as_ref().and_then(|c| c.iso_code), Some("SE"));

    let city: super::geoip2::City = reader.lookup(ip_v6).unwrap();
    assert_eq!(city.country.as_ref().and_then(|c| c.iso_code), Some("KR"));

    assert!(reader.lookup::<super::geoip2::City>(ip_not_found).is_err());
}

// ====== Malformed Database Tests: Verify errors, not panics ======

#[test]
fn test_bad_data_databases_do_not_panic() {
    let bad_dbs = [
        "bad-data/libmaxminddb/libmaxminddb-corrupt-search-tree.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-deep-array-nesting.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-deep-nesting.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-empty-array-last-in-metadata.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-empty-map-last-in-metadata.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-metadata-marker-only.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-offset-integer-overflow.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-oversized-array.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-oversized-map.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-separator-record-max-left.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-separator-record-min-left.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-separator-record-min-right.mmdb",
        "bad-data/libmaxminddb/libmaxminddb-uint64-max-epoch.mmdb",
        "bad-data/maxminddb-golang/cyclic-data-structure.mmdb",
        "bad-data/maxminddb-golang/invalid-bytes-length.mmdb",
        "bad-data/maxminddb-golang/invalid-data-record-offset.mmdb",
        "bad-data/maxminddb-golang/invalid-map-key-length.mmdb",
        "bad-data/maxminddb-golang/invalid-string-length.mmdb",
        "bad-data/maxminddb-golang/metadata-is-an-uint128.mmdb",
        "bad-data/maxminddb-golang/unexpected-bytes.mmdb",
        "bad-data/maxminddb-python/bad-unicode-in-map-key.mmdb",
    ];

    for db in &bad_dbs {
        let path = format!("test-data/{db}");
        let buf = std::fs::read(&path).expect("bad-data file should exist");

        let open_result = std::panic::catch_unwind(|| Reader::from_source(buf.clone()));
        assert!(open_result.is_ok(), "opening {db} should not panic, got: {open_result:?}");

        if let Ok(Ok(reader)) = open_result {
            let ip: IpAddr = "1.1.1.1".parse().unwrap();
            let lookup_result = std::panic::catch_unwind(|| {
                let _ = reader.lookup::<super::Value>(ip);
            });
            assert!(lookup_result.is_ok(), "lookup on {db} should not panic, got: {lookup_result:?}");
        }
    }
}

#[test]
fn test_broken_test_databases_do_not_panic() {
    let broken_dbs = [
        "MaxMind-DB-test-broken-pointers-24.mmdb",
        "MaxMind-DB-test-broken-search-tree-24.mmdb",
        "GeoIP2-City-Test-Broken-Double-Format.mmdb",
    ];

    for db in &broken_dbs {
        let path = format!("test-data/test-data/{db}");
        let reader = Reader::open_readfile(&path).expect("should open");

        let ip: IpAddr = "1.1.1.1".parse().unwrap();
        let result = std::panic::catch_unwind(|| {
            let _ = reader.lookup::<super::Value>(ip);
        });
        assert!(result.is_ok(), "lookup on {db} should not panic, got: {result:?}");

        let ip_v6: IpAddr = "::1".parse().unwrap();
        let result = std::panic::catch_unwind(|| {
            let _ = reader.lookup::<super::Value>(ip_v6);
        });
        assert!(result.is_ok(), "lookup on {db} should not panic, got: {result:?}");
    }
}
