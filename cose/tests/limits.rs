use cose::{CoseKey, CoseKeySet, Error, Header, HeaderRole, Mac, Sign};

fn push_uint_label(out: &mut Vec<u8>, label: u8) {
    if label < 24 {
        out.push(label);
    } else {
        out.push(0x18);
        out.push(label);
    }
}

fn map_with_labels(count: u8) -> Vec<u8> {
    let mut out = Vec::new();
    if count < 24 {
        out.push(0xa0 | count);
    } else {
        out.push(0xb8);
        out.push(count);
    }
    for label in 1..=count {
        push_uint_label(&mut out, label);
        out.push(0);
    }
    out
}

#[test]
fn header_rejects_empty_unprotected_header() {
    let header = Header::from_slice(&[]).unwrap();
    assert!(matches!(header.validate(HeaderRole::Unprotected), Err(Error::InvalidHeader)));
}

#[test]
fn header_rejects_empty_known_bstr_values() {
    // {4: h''}
    let kid = [0xa1, 0x04, 0x40];
    assert!(matches!(
        Header::from_slice(&kid).unwrap().validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));

    // {5: h''}
    let iv = [0xa1, 0x05, 0x40];
    assert!(matches!(
        Header::from_slice(&iv).unwrap().validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));

    // {6: h''}
    let partial_iv = [0xa1, 0x06, 0x40];
    assert!(matches!(
        Header::from_slice(&partial_iv).unwrap().validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));
}

#[test]
fn header_rejects_malformed_content_type() {
    // {3: ""}
    assert!(matches!(
        Header::from_slice(&[0xa1, 0x03, 0x60])
            .unwrap()
            .validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));
    // {3: "foo"}
    assert!(matches!(
        Header::from_slice(&[0xa1, 0x03, 0x63, b'f', b'o', b'o'])
            .unwrap()
            .validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));
    // {3: " application/json"}
    let mut leading_space = vec![0xa1, 0x03, 0x71];
    leading_space.extend_from_slice(b" application/json");
    assert!(matches!(
        Header::from_slice(&leading_space)
            .unwrap()
            .validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));
}

#[test]
fn header_rejects_out_of_range_algorithm() {
    // {1: 18446744073709551615}
    let header = [0xa1, 0x01, 0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    assert!(matches!(
        Header::from_slice(&header).unwrap().validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));
}

#[test]
fn header_rejects_duplicate_crit_entries() {
    // {1: -7, 2: [1, 1]}
    let header = [0xa2, 0x01, 0x26, 0x02, 0x82, 0x01, 0x01];
    assert!(matches!(
        Header::from_slice(&header).unwrap().validate(HeaderRole::Protected),
        Err(Error::InvalidHeader)
    ));
}

#[test]
fn header_rejects_too_many_entries() {
    let header = map_with_labels(33);
    assert!(matches!(Header::from_slice(&header), Err(Error::TooManyEntries)));
}

#[test]
fn header_rejects_excessive_nesting() {
    // {100: [[[...]]]} with 40 nested arrays.
    let mut header = vec![0xa1, 0x18, 0x64];
    header.extend(core::iter::repeat_n(0x81u8, 40));
    header.push(0x00);
    assert!(Header::from_slice(&header).is_err());
}

#[test]
fn cose_key_rejects_too_many_entries() {
    let key = map_with_labels(33);
    assert!(matches!(CoseKey::from_slice(&key), Err(Error::TooManyEntries)));
}

#[test]
fn key_set_rejects_too_many_keys() {
    // 1025-element array of empty maps.
    let mut set = vec![0x99, 0x04, 0x01];
    set.extend(core::iter::repeat_n(0xa0u8, 1025));
    assert!(matches!(CoseKeySet::from_slice(&set), Err(Error::InvalidStructure)));
}

#[test]
fn sign_rejects_too_many_signatures() {
    // COSE_Sign with 65 signatures.
    let mut sign = vec![0xd8, 0x62, 0x84, 0x40, 0xa0, 0x41, 0x00, 0x99, 0x00, 0x41];
    for _ in 0..65 {
        sign.extend_from_slice(&[0x83, 0x40, 0xa0, 0x40]);
    }
    assert!(matches!(Sign::from_slice(&sign), Err(Error::InvalidStructure)));
}

#[test]
fn mac_rejects_too_many_recipients() {
    // COSE_Mac with 65 recipients.
    let mut mac = vec![0xd8, 0x61, 0x85, 0x40, 0xa0, 0x41, 0x00, 0x41, 0x00, 0x99, 0x00, 0x41];
    for _ in 0..65 {
        mac.extend_from_slice(&[0x83, 0x40, 0xa0, 0x40]);
    }
    assert!(matches!(Mac::from_slice(&mac), Err(Error::InvalidStructure)));
}
