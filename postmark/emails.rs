use std::collections::HashMap;

use json::{Decode, DecodeOwned, Decoder, Encode, Encoder, ErrorKind, Result, Write};
use reqwest::Method;

use crate::{ApiError, Client, SendRequestInput};

/// An email to send through Postmark.
#[derive(Clone, Debug, Default)]
pub struct Email {
    /// From: The sender email address. Must have a registered and confirmed Sender Signature.
    pub from: String,

    /// To: Recipient email address. Multiple addresses are comma separated. Max 50.
    pub to: String,

    /// Cc recipient email address. Multiple addresses are comma separated. Max 50.
    pub cc: Option<String>,

    /// Bcc recipient email address. Multiple addresses are comma separated. Max 50.
    pub bcc: Option<String>,

    /// Email subject
    pub subject: Option<String>,

    /// The body of the message
    pub body: Body,

    /// Email tag that allows you to categorize outgoing emails and get detailed statistics.
    pub tag: Option<String>,

    /// Reply To override email address. Defaults to the Reply To set in the sender signature.
    pub reply_to: Option<String>,

    /// List of custom headers to include.
    pub headers: Option<Vec<Header>>,

    /// Activate open tracking for this email.
    pub track_opens: Option<bool>,

    /// Activate link tracking for links in the HTML or Text bodies of this email.
    pub track_links: Option<TrackLink>,

    /// List of attachments
    pub attachments: Option<Vec<Attachment>>,

    /// Custom metadata key/value pairs.
    pub metadata: Option<HashMap<String, String>>,

    /// Set message stream ID that's used for sending. If not provided, message will default to the "outbound" transactional stream.
    pub message_stream: Option<String>,
}

fn write_field<W: Write, T: Encode + ?Sized>(
    encoder: &mut Encoder<W>,
    first: &mut bool,
    key: &str,
    value: &T,
) -> core::result::Result<(), W::Error> {
    encoder.comma(*first)?;
    *first = false;
    encoder.key(key)?;
    value.encode(encoder)
}

fn write_opt<W: Write, T: Encode>(
    encoder: &mut Encoder<W>,
    first: &mut bool,
    key: &str,
    value: &Option<T>,
) -> core::result::Result<(), W::Error> {
    if let Some(value) = value {
        write_field(encoder, first, key, value)?;
    }
    Ok(())
}

impl Encode for Email {
    fn encode<W: Write>(&self, encoder: &mut Encoder<W>) -> core::result::Result<(), W::Error> {
        encoder.begin_object()?;
        let mut first = true;
        write_field(encoder, &mut first, "From", &self.from)?;
        write_field(encoder, &mut first, "To", &self.to)?;
        write_opt(encoder, &mut first, "Cc", &self.cc)?;
        write_opt(encoder, &mut first, "Bcc", &self.bcc)?;
        write_opt(encoder, &mut first, "Subject", &self.subject)?;

        // `Body` is flattened into the email object.
        let body_bytes = json::to_vec(&self.body);
        let body: json::Value = json::from_slice(&body_bytes).expect("Body always encodes as JSON");
        if let json::Value::Object(map) = body {
            for (key, value) in map {
                encoder.comma(first)?;
                first = false;
                encoder.key(key.as_ref())?;
                value.encode(encoder)?;
            }
        }

        write_opt(encoder, &mut first, "Tag", &self.tag)?;
        write_opt(encoder, &mut first, "ReplyTo", &self.reply_to)?;
        write_opt(encoder, &mut first, "Headers", &self.headers)?;
        write_opt(encoder, &mut first, "TrackOpens", &self.track_opens)?;
        write_opt(encoder, &mut first, "TrackLinks", &self.track_links)?;
        write_opt(encoder, &mut first, "Attachments", &self.attachments)?;
        write_opt(encoder, &mut first, "Metadata", &self.metadata)?;
        write_opt(encoder, &mut first, "MessageStream", &self.message_stream)?;
        encoder.end_object()
    }
}

fn optional_string<'a>(object: &json::Map<'a>, key: &'static str) -> Result<Option<String>> {
    match object.get(key) {
        None => Ok(None),
        Some(value) if value.is_null() => Ok(None),
        Some(value) => value
            .as_str()
            .map(|value| Some(value.to_string()))
            .ok_or_else(|| json::Error::message(key, 0)),
    }
}

fn required_string<'a>(object: &json::Map<'a>, key: &'static str) -> Result<String> {
    optional_string(object, key)?.ok_or_else(|| json::Error::new(ErrorKind::MissingField(key), 0))
}

fn optional_field<'a, T: DecodeOwned>(object: &json::Map<'a>, key: &'static str) -> Result<Option<T>> {
    match object.get(key) {
        None => Ok(None),
        Some(value) if value.is_null() => Ok(None),
        Some(value) => json::from_slice_owned::<T>(&json::to_vec(value))
            .map(Some)
            .map_err(|_| json::Error::message(key, 0)),
    }
}

impl<'a> Decode<'a> for Email {
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self> {
        let value = decoder.decode::<json::Value<'a>>()?;
        let object = value
            .as_object()
            .ok_or_else(|| decoder.error(ErrorKind::Message("email must be an object")))?;

        let from = required_string(object, "From")?;
        let to = required_string(object, "To")?;
        let cc = optional_string(object, "Cc")?;
        let bcc = optional_string(object, "Bcc")?;
        let subject = optional_string(object, "Subject")?;

        let text = optional_string(object, "TextBody")?;
        let html = optional_string(object, "HtmlBody")?;
        let body = match (html, text) {
            (Some(html), Some(text)) => Body::HtmlAndText {
                html,
                text,
            },
            (Some(html), None) => Body::Html {
                html,
            },
            (None, Some(text)) => Body::Text {
                text,
            },
            (None, None) => Body::default(),
        };

        let tag = optional_string(object, "Tag")?;
        let reply_to = optional_string(object, "ReplyTo")?;
        let headers = optional_field::<Vec<Header>>(object, "Headers")?;
        let track_opens = object.get("TrackOpens").and_then(json::Value::as_bool);
        let track_links = optional_field::<TrackLink>(object, "TrackLinks")?;
        let attachments = optional_field::<Vec<Attachment>>(object, "Attachments")?;
        let metadata = optional_field::<HashMap<String, String>>(object, "Metadata")?;
        let message_stream = optional_string(object, "MessageStream")?;

        Ok(Email {
            from,
            to,
            cc,
            bcc,
            subject,
            body,
            tag,
            reply_to,
            headers,
            track_opens,
            track_links,
            attachments,
            metadata,
            message_stream,
        })
    }
}

#[derive(Debug, Clone, PartialEq, json::Encode, json::Decode)]
#[json(untagged)]
pub enum Body {
    Text {
        #[json(rename = "TextBody")]
        text: String,
    },
    Html {
        #[json(rename = "HtmlBody")]
        html: String,
    },
    HtmlAndText {
        #[json(rename = "HtmlBody")]
        html: String,
        #[json(rename = "TextBody")]
        text: String,
    },
}
impl Default for Body {
    fn default() -> Self {
        Body::Text {
            text: "".into(),
        }
    }
}

impl Body {
    /// Constructor to create a text-only [`Body`] enum
    pub fn text(text: String) -> Self {
        Body::Text {
            text,
        }
    }
    /// Constructor to create a html-only [`Body`] enum
    pub fn html(html: String) -> Self {
        Body::Html {
            html,
        }
    }
    /// Constructor to create a text and html [`Body`] enum
    pub fn html_and_text(html: String, text: String) -> Self {
        Body::HtmlAndText {
            html,
            text,
        }
    }
}

/// A custom header to include in an email.
#[derive(Clone, Debug, json::Encode, json::Decode)]
#[json(rename_all = "PascalCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

/// An attachment for emails.
#[derive(Clone, Debug, Default, json::Encode, json::Decode)]
#[json(rename_all = "PascalCase")]
pub struct Attachment {
    pub name: String,
    pub content: String,
    pub content_type: String,
    #[json(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}

/// Whether to activate link tracking for links in the HTML or Text bodies of the emails.
#[derive(Clone, Debug, json::Encode, json::Decode, Default)]
pub enum TrackLink {
    #[default]
    None,
    HtmlAndText,
    HtmlOnly,
    TextOnly,
}

#[derive(Clone, Debug, Default, json::Encode, json::Decode)]
#[json(rename_all = "PascalCase")]
pub struct SendEmailResponse {
    pub to: Option<String>,
    pub submitted_at: Option<String>,
    #[json(rename = "MessageID")]
    pub message_id: Option<String>,
    pub error_code: i64,
    pub message: String,
}

impl Client {
    pub async fn send_email(&self, server_token: String, email: Email) -> Result<SendEmailResponse, ApiError> {
        return self
            .send_request(SendRequestInput {
                method: Method::POST,
                url: "/email".to_string(),
                body: email,
                server_token: Some(server_token),
            })
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_roundtrip_flattens_body() {
        let email = Email {
            from: "a@example.com".into(),
            to: "b@example.com".into(),
            subject: Some("hi".into()),
            body: Body::html_and_text("<p>hi</p>".into(), "hi".into()),
            track_opens: Some(true),
            ..Email::default()
        };
        let text = json::to_string(&email);
        assert!(text.contains(r#""From":"a@example.com""#), "{text}");
        assert!(text.contains(r#""HtmlBody":"<p>hi</p>""#), "{text}");
        assert!(text.contains(r#""TextBody":"hi""#), "{text}");
        let back: Email = json::from_str(&text).unwrap();
        assert_eq!(back.body, email.body);
        assert_eq!(back.subject, email.subject);
        assert_eq!(back.track_opens, email.track_opens);
    }
}
