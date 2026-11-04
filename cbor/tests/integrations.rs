#![cfg(any(
    feature = "heapless",
    feature = "small_collections",
    feature = "bytes",
    feature = "derive"
))]

#[cfg(feature = "heapless")]
mod heapless_tests {
    #[cfg(feature = "derive")]
    use cbor::{Decode, Encode};

    #[test]
    fn vector_string_and_map() {
        let mut vector: heapless::Vec<u8, 4> = heapless::Vec::new();
        vector.extend_from_slice(&[1, 2, 3]).unwrap();
        let bytes = cbor::to_vec(&vector);
        assert_eq!(bytes, vec![0x83, 1, 2, 3]);
        assert_eq!(cbor::from_slice::<heapless::Vec<u8, 4>>(&bytes).unwrap(), vector);

        let string: heapless::String<8> = heapless::String::try_from("hello").unwrap();
        let bytes = cbor::to_vec(&string);
        assert_eq!(cbor::from_slice::<heapless::String<8>>(&bytes).unwrap(), string);

        let mut map: heapless::index_map::FnvIndexMap<u8, u8, 4> = heapless::index_map::FnvIndexMap::new();
        map.insert(1, 2).unwrap();
        map.insert(3, 4).unwrap();
        let bytes = cbor::to_vec(&map);
        let back: heapless::index_map::FnvIndexMap<u8, u8, 4> = cbor::from_slice(&bytes).unwrap();
        assert_eq!(back.get(&1), Some(&2));
        assert_eq!(back.get(&3), Some(&4));
    }

    #[test]
    fn capacity_errors() {
        let bytes = [0x84, 1, 2, 3, 4];
        assert!(cbor::from_slice::<heapless::Vec<u8, 3>>(&bytes).is_err());
        let bytes = [0x65, b'h', b'e', b'l', b'l', b'o'];
        assert!(cbor::from_slice::<heapless::String<4>>(&bytes).is_err());
    }

    #[cfg(feature = "derive")]
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Packet {
        #[cbor(bytes)]
        payload: heapless::Vec<u8, 8>,
    }

    #[cfg(feature = "derive")]
    #[test]
    fn bytes_field() {
        let mut payload: heapless::Vec<u8, 8> = heapless::Vec::new();
        payload.extend_from_slice(&[0xde, 0xad]).unwrap();
        let value = Packet {
            payload,
        };
        let bytes = cbor::to_vec(&value);
        assert!(bytes.windows(2).any(|w| w == [0x42, 0xde]));
        assert_eq!(cbor::from_slice::<Packet>(&bytes).unwrap(), value);
    }
}

#[cfg(feature = "small_collections")]
mod small_collections_tests {
    #[cfg(feature = "derive")]
    use cbor::{Decode, Encode};
    use small_collections::{SmallString, SmallVec};

    #[test]
    fn vector_and_string() {
        let vector: SmallVec<u8, 4> = SmallVec::from_slice(&[1, 2, 3]);
        let bytes = cbor::to_vec(&vector);
        assert_eq!(bytes, vec![0x83, 1, 2, 3]);
        assert_eq!(cbor::from_slice::<SmallVec<u8, 4>>(&bytes).unwrap(), vector);

        let mut string = SmallString::<8>::new();
        string.push_str("hello");
        let bytes = cbor::to_vec(&string);
        assert_eq!(cbor::from_slice::<SmallString<8>>(&bytes).unwrap(), string);
    }

    #[cfg(feature = "derive")]
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Packet {
        #[cbor(bytes)]
        payload: SmallVec<u8, 4>,
    }

    #[cfg(feature = "derive")]
    #[test]
    fn bytes_field() {
        let payload = SmallVec::from_slice(&[0xde, 0xad]);
        let value = Packet {
            payload,
        };
        let bytes = cbor::to_vec(&value);
        assert_eq!(cbor::from_slice::<Packet>(&bytes).unwrap(), value);
    }
}

#[cfg(feature = "bytes")]
mod bytes_tests {
    use bytes::{Bytes, BytesMut};

    #[test]
    fn bytes_are_byte_strings() {
        let value = Bytes::from_static(b"\xde\xad");
        let bytes = cbor::to_vec(&value);
        assert_eq!(bytes, vec![0x42, 0xde, 0xad]);
        assert_eq!(cbor::from_slice::<Bytes>(&bytes).unwrap(), value);

        let value = BytesMut::from(&b"\xbe\xef"[..]);
        let bytes = cbor::to_vec(&value);
        assert_eq!(bytes, vec![0x42, 0xbe, 0xef]);
        assert_eq!(cbor::from_slice::<BytesMut>(&bytes).unwrap(), value);
    }
}
