use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{ApiError, Client, SendRequestInput};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Email {
    /// From: The sender email address. Must have a registered and confirmed Sender Signature.
    pub from: String,

    /// To: Recipient email address. Multiple addresses are comma separated. Max 50.
    pub to: String,

    /// Cc recipient email address. Multiple addresses are comma separated. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,

    /// Bcc recipient email address. Multiple addresses are comma separated. Max 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,

    /// Email subject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    /// The body of the message
    #[serde(flatten)]
    pub body: Body,

    /// Email tag that allows you to categorize outgoing emails and get detailed statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// Reply To override email address. Defaults to the Reply To set in the sender signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,

    /// List of custom headers to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<Header>>,

    /// Activate open tracking for this email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<bool>,

    /// Activate link tracking for links in the HTML or Text bodies of this email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_links: Option<TrackLink>,

    /// List of attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,

    /// Custom metadata key/value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Set message stream ID that's used for sending. If not provided, message will default to the "outbound" transactional stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    Text {
        #[serde(rename = "TextBody")]
        text: String,
    },
    Html {
        #[serde(rename = "HtmlBody")]
        html: String,
    },
    HtmlAndText {
        #[serde(rename = "HtmlBody")]
        html: String,
        #[serde(rename = "TextBody")]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

/// An attachment for emails.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    pub name: String,
    pub content: String,
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
}

/// Whether to activate link tracking for links in the HTML or Text bodies of the emails.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum TrackLink {
    #[default]
    None,
    HtmlAndText,
    HtmlOnly,
    TextOnly,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailResponse {
    pub to: Option<String>,
    pub submitted_at: Option<String>,
    #[serde(rename = "MessageID")]
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
