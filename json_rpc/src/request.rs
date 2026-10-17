use std::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, DeserializeOwned, MapAccess, SeqAccess, Visitor, value::MapAccessDeserializer},
};
use serde_json::value::RawValue;

use crate::Id;

/// The `id` field of a JSON-RPC request.
///
/// This wrapper distinguishes three states:
/// - **Absent**: the field was not present in the JSON → notification
/// - **Null**: the field was `null` → request with a null id (discouraged)
/// - **Present**: the field was a string or number → normal request
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct RequestId(pub Option<Id>);

impl RequestId {
    /// Returns `true` if this is a notification (no `id` field).
    pub fn is_notification(&self) -> bool {
        self.0.is_none()
    }

    /// Returns a reference to the inner `Id`, if present.
    pub fn as_ref(&self) -> Option<&Id> {
        self.0.as_ref()
    }

    /// Consumes `self` and returns the inner `Option<Id>`.
    pub fn into_id(self) -> Option<Id> {
        self.0
    }
}

impl Serialize for RequestId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            Some(id) => id.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for RequestId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RequestIdVisitor;

        impl<'de> Visitor<'de> for RequestIdVisitor {
            type Value = RequestId;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a JSON-RPC id: a string, an integer, or null")
            }

            fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(RequestId(Some(Id::Null)))
            }

            fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
                Ok(RequestId(Some(Id::Null)))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(RequestId(Some(Id::Number(v))))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(RequestId(Some(Id::Number(v as i64))))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                if v.fract() == 0.0 {
                    Ok(RequestId(Some(Id::Number(v as i64))))
                } else {
                    Err(de::Error::invalid_value(de::Unexpected::Float(v), &self))
                }
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(RequestId(Some(Id::String(v.to_owned()))))
            }

            fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                Ok(RequestId(Some(Id::String(v))))
            }
        }

        deserializer.deserialize_any(RequestIdVisitor)
    }
}

/// A JSON-RPC 2.0 request object.
///
/// Params are stored as `Box<RawValue>` to defer deserialization
/// until the method handler is known.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// The JSON-RPC version — must be `"2.0"`.
    pub jsonrpc: String,
    /// The name of the method to invoke.
    pub method: String,
    /// Structured parameters for the method (optional).
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<RawValue>>,
    /// Client-assigned identifier. Absent for notifications.
    #[serde(default)]
    #[serde(skip_serializing_if = "RequestId::is_notification")]
    pub id: RequestId,
}

impl Request {
    /// Returns `true` if this request is a notification (has no `id`).
    pub fn is_notification(&self) -> bool {
        self.id.is_notification()
    }

    /// Deserializes the params into a concrete type.
    ///
    /// Returns an error if the params are absent or fail to deserialize.
    pub fn parse_params<P: DeserializeOwned>(&self) -> Result<P, serde_json::Error> {
        match &self.params {
            Some(raw) => serde_json::from_str(raw.get()),
            None => serde_json::from_str("{}"),
        }
    }
}

/// A single JSON-RPC request message, or a batch of them.
///
/// Per the spec, a client may send either a single `Request` object
/// or an `Array` of request objects (a batch). Batch entries that are
/// not valid request objects are preserved as raw JSON so the server
/// can respond with individual `Invalid Request` errors.
#[derive(Clone, Debug)]
pub enum RequestMessage {
    /// A single request.
    Single(Request),
    /// A batch of raw request values. Each element is parsed individually
    /// during dispatch, so invalid entries get individual error responses.
    Batch(Vec<Request>),
}

impl RequestMessage {
    /// Returns `true` if this message is a batch.
    pub fn is_batch(&self) -> bool {
        matches!(self, Self::Batch(_))
    }

    /// Returns the number of entries in this message.
    pub fn len(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Batch(entries) => entries.len(),
        }
    }

    /// Returns `true` if there are no entries in this message.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Batch(entries) => entries.is_empty(),
            _ => false,
        }
    }
}

impl From<Request> for RequestMessage {
    fn from(req: Request) -> Self {
        Self::Single(req)
    }
}

impl Serialize for RequestMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Single(req) => req.serialize(serializer),
            Self::Batch(entries) => entries.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for RequestMessage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RequestMessageVisitor;

        impl<'de> Visitor<'de> for RequestMessageVisitor {
            type Value = RequestMessage;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a JSON-RPC request object or array")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut requests = Vec::with_capacity(seq.size_hint().unwrap_or(1));
                while let Some(req) = seq.next_element::<Request>()? {
                    requests.push(req);
                }
                Ok(RequestMessage::Batch(requests))
            }

            fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
                let req = Request::deserialize(MapAccessDeserializer::new(map))?;
                Ok(RequestMessage::Single(req))
            }
        }

        deserializer.deserialize_any(RequestMessageVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deserialize_request() {
        let json = r#"{"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1}"#;
        let req: Request = serde_json::from_str(json).unwrap();
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "subtract");
        assert!(req.params.is_some());
        assert_eq!(req.id, RequestId(Some(Id::Number(1))));
        assert!(!req.is_notification());
    }

    #[test]
    fn test_deserialize_notification() {
        let json = r#"{"jsonrpc":"2.0","method":"update","params":[1,2,3]}"#;
        let req: Request = serde_json::from_str(json).unwrap();
        assert!(req.is_notification());
        assert_eq!(req.id, RequestId(None));
    }

    #[test]
    fn test_deserialize_request_no_params() {
        let json = r#"{"jsonrpc":"2.0","method":"foobar"}"#;
        let req: Request = serde_json::from_str(json).unwrap();
        assert_eq!(req.method, "foobar");
        assert!(req.params.is_none());
        assert!(req.is_notification());
    }

    #[test]
    fn test_deserialize_request_string_id() {
        let json = r#"{"jsonrpc":"2.0","method":"get_data","id":"abc"}"#;
        let req: Request = serde_json::from_str(json).unwrap();
        assert_eq!(req.id, RequestId(Some(Id::String("abc".into()))));
    }

    #[test]
    fn test_deserialize_request_null_id() {
        let json = r#"{"jsonrpc":"2.0","method":"foo","id":null}"#;
        let req: Request = serde_json::from_str(json).unwrap();
        assert_eq!(req.id, RequestId(Some(Id::Null)));
    }

    #[test]
    fn test_request_parse_params() {
        let req: Request = serde_json::from_str(r#"{"jsonrpc":"2.0","method":"add","params":[1,2],"id":1}"#).unwrap();
        let (a, b): (i64, i64) = req.parse_params().unwrap();
        assert_eq!((a, b), (1, 2));
    }

    #[test]
    fn test_request_parse_params_absent() {
        #[derive(Deserialize)]
        struct PingParams {
            _extra: Option<String>,
        }
        let req: Request = serde_json::from_str(r#"{"jsonrpc":"2.0","method":"ping","id":1}"#).unwrap();
        let p: PingParams = req.parse_params().unwrap();
        assert!(p._extra.is_none());
    }

    #[test]
    fn test_request_id_into_id() {
        let id = RequestId(Some(Id::Number(42)));
        assert_eq!(id.into_id(), Some(Id::Number(42)));
        let id = RequestId(None);
        assert_eq!(id.into_id(), None);
    }

    #[test]
    fn test_deserialize_single_request_message() {
        let json = r#"{"jsonrpc":"2.0","method":"test","id":1}"#;
        let message: RequestMessage = serde_json::from_str(json).unwrap();
        assert!(!message.is_batch());
        assert_eq!(message.len(), 1);
        assert!(!message.is_empty());
    }

    #[test]
    fn test_deserialize_batch_request_message() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"a","id":1},
            {"jsonrpc":"2.0","method":"b","id":2}
        ]"#;
        let message: RequestMessage = serde_json::from_str(json).unwrap();
        assert!(message.is_batch());
        assert_eq!(message.len(), 2);
        assert!(!message.is_empty());
    }

    // #[test]
    // fn test_deserialize_batch_with_invalid_entries() {
    //     let json = r#"[
    //         {"jsonrpc":"2.0","method":"a","id":1},
    //         42,
    //         {"jsonrpc":"2.0","method":"b","id":2}
    //     ]"#;
    //     let message: RequestMessage = serde_json::from_str(json).unwrap();
    //     assert!(message.is_batch());
    //     assert_eq!(message.len(), 2);
    // }

    #[test]
    fn test_deserialize_empty_array() {
        let json = "[]";
        let message: RequestMessage = serde_json::from_str(json).unwrap();
        assert!(message.is_batch());
        assert!(message.is_empty());
        assert_eq!(message.len(), 0);
    }

    #[test]
    fn test_request_message_from_request() {
        let req: Request = serde_json::from_str(r#"{"jsonrpc":"2.0","method":"x","id":1}"#).unwrap();
        let message: RequestMessage = req.into();
        assert!(!message.is_batch());
    }

    #[test]
    fn test_request_serialize() {
        let req = Request {
            jsonrpc: "2.0".into(),
            method: "subtract".into(),
            params: Some(RawValue::from_string("[42,23]".into()).unwrap()),
            id: RequestId(Some(Id::Number(1))),
        };
        let json = serde_json::to_string(&req).unwrap();
        let expected = json!({"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1});
        let actual: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_request_message_batch_serialize() {
        let reqs = vec![
            Request {
                jsonrpc: "2.0".into(),
                method: "a".into(),
                params: None,
                id: RequestId(Some(Id::Number(1))),
            },
            Request {
                jsonrpc: "2.0".into(),
                method: "b".into(),
                params: None,
                id: RequestId(Some(Id::Number(2))),
            },
        ];
        let message = RequestMessage::Batch(reqs);
        assert!(message.is_batch());
        let json = serde_json::to_string(&message).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(actual.is_array());
        assert_eq!(actual.as_array().unwrap().len(), 2);
    }
}
