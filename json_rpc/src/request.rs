use json::{Decode, DecodeOwned, Decoder, Encode, Encoder, ErrorKind, RawValueBuf, Result, Write};

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

impl Encode for RequestId {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        match &self.0 {
            Some(id) => id.encode(encoder),
            None => encoder.null(),
        }
    }
}

impl<'a> Decode<'a> for RequestId {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            Ok(RequestId(Some(Id::Null)))
        } else {
            Ok(RequestId(Some(decoder.decode::<Id>()?)))
        }
    }
}

/// A JSON-RPC 2.0 request object.
///
/// Params are stored as an owned raw JSON value to defer deserialization
/// until the method handler is known.
#[derive(Clone, Debug, json::Encode, json::Decode)]
pub struct Request {
    /// The JSON-RPC version — must be `"2.0"`.
    pub jsonrpc: String,
    /// The name of the method to invoke.
    pub method: String,
    /// Structured parameters for the method (optional).
    #[json(skip_serializing_if = "Option::is_none")]
    pub params: Option<RawValueBuf>,
    /// Client-assigned identifier. Absent for notifications.
    #[json(default, skip_serializing_if = "RequestId::is_notification")]
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
    pub fn parse_params<P: DecodeOwned>(&self) -> Result<P> {
        match &self.params {
            Some(raw) => P::decode_owned(raw.as_bytes()),
            None => P::decode_owned(b"{}"),
        }
    }
}

/// A single JSON-RPC request message, or a batch of them.
///
/// Per the spec, a client may send either a single `Request` object
/// or an `Array` of request objects (a batch).
#[derive(Clone, Debug)]
pub enum RequestMessage {
    /// A single request.
    Single(Request),
    /// A batch of requests.
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

impl Encode for RequestMessage {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        match self {
            Self::Single(req) => req.encode(encoder),
            Self::Batch(entries) => entries.encode(encoder),
        }
    }
}

impl<'a> Decode<'a> for RequestMessage {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'[') => Ok(Self::Batch(decoder.decode::<Vec<Request>>()?)),
            None => Err(decoder.error(ErrorKind::UnexpectedEof)),
            Some(_) => Ok(Self::Single(decoder.decode::<Request>()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(text: &str) -> RawValueBuf {
        RawValueBuf::try_from_slice(text.as_bytes()).unwrap()
    }

    #[test]
    fn test_deserialize_request() {
        let text = r#"{"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1}"#;
        let req: Request = json::from_str(text).unwrap();
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "subtract");
        assert!(req.params.is_some());
        assert_eq!(req.id, RequestId(Some(Id::Number(1))));
        assert!(!req.is_notification());
    }

    #[test]
    fn test_deserialize_notification() {
        let text = r#"{"jsonrpc":"2.0","method":"update","params":[1,2,3]}"#;
        let req: Request = json::from_str(text).unwrap();
        assert!(req.is_notification());
        assert_eq!(req.id, RequestId(None));
    }

    #[test]
    fn test_deserialize_request_no_params() {
        let text = r#"{"jsonrpc":"2.0","method":"foobar"}"#;
        let req: Request = json::from_str(text).unwrap();
        assert_eq!(req.method, "foobar");
        assert!(req.params.is_none());
        assert!(req.is_notification());
    }

    #[test]
    fn test_deserialize_request_string_id() {
        let text = r#"{"jsonrpc":"2.0","method":"get_data","id":"abc"}"#;
        let req: Request = json::from_str(text).unwrap();
        assert_eq!(req.id, RequestId(Some(Id::String("abc".into()))));
    }

    #[test]
    fn test_deserialize_request_null_id() {
        let text = r#"{"jsonrpc":"2.0","method":"foo","id":null}"#;
        let req: Request = json::from_str(text).unwrap();
        assert_eq!(req.id, RequestId(Some(Id::Null)));
    }

    #[test]
    fn test_request_parse_params() {
        let req: Request = json::from_str(r#"{"jsonrpc":"2.0","method":"add","params":[1,2],"id":1}"#).unwrap();
        let (a, b): (i64, i64) = req.parse_params().unwrap();
        assert_eq!((a, b), (1, 2));
    }

    #[test]
    fn test_request_parse_params_absent() {
        #[derive(json::Decode)]
        struct PingParams {
            _extra: Option<String>,
        }
        let req: Request = json::from_str(r#"{"jsonrpc":"2.0","method":"ping","id":1}"#).unwrap();
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
        let text = r#"{"jsonrpc":"2.0","method":"test","id":1}"#;
        let message: RequestMessage = json::from_str(text).unwrap();
        assert!(!message.is_batch());
        assert_eq!(message.len(), 1);
        assert!(!message.is_empty());
    }

    #[test]
    fn test_deserialize_batch_request_message() {
        let text = r#"[
            {"jsonrpc":"2.0","method":"a","id":1},
            {"jsonrpc":"2.0","method":"b","id":2}
        ]"#;
        let message: RequestMessage = json::from_str(text).unwrap();
        assert!(message.is_batch());
        assert_eq!(message.len(), 2);
        assert!(!message.is_empty());
    }

    #[test]
    fn test_deserialize_empty_array() {
        let message: RequestMessage = json::from_str("[]").unwrap();
        assert!(message.is_batch());
        assert!(message.is_empty());
        assert_eq!(message.len(), 0);
    }

    #[test]
    fn test_request_message_from_request() {
        let req: Request = json::from_str(r#"{"jsonrpc":"2.0","method":"x","id":1}"#).unwrap();
        let message: RequestMessage = req.into();
        assert!(!message.is_batch());
    }

    #[test]
    fn test_request_serialize() {
        let req = Request {
            jsonrpc: "2.0".into(),
            method: "subtract".into(),
            params: Some(raw("[42,23]")),
            id: RequestId(Some(Id::Number(1))),
        };
        let text = json::to_string(&req);
        let actual: json::Value = json::from_str(&text).unwrap();
        let expected = json::json!({"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1});
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
        let text = json::to_string(&message);
        let actual: json::Value = json::from_str(&text).unwrap();
        assert_eq!(actual.as_array().unwrap().len(), 2);
    }
}
