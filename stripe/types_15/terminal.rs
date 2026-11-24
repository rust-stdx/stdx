#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub enum GetTerminalReaderResponseExpanded {
    /// Expanded `terminal.reader`.
    TerminalReader(Box<crate::terminal::TerminalReader>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for GetTerminalReaderResponseExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "terminal.reader" => GetTerminalReaderResponseExpanded::TerminalReader(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => GetTerminalReaderResponseExpanded::Other(value),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReader {
    #[doc = "The most recent action performed by the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::terminal::TerminalReaderReaderResourceReaderAction>,
    #[doc = "The current software version of the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub device_sw_version: Option<String>,
    #[doc = "Device type of the reader."]
    #[json(default)]
    pub device_type: stripe_types_00::terminal::TerminalReaderDeviceType,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The local IP address of the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "Custom label given to the reader for easier identification."]
    #[json(default)]
    pub label: String,
    #[doc = "The last time this reader reported to Stripe backend."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The location identifier of the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::Expandable<stripe_types_06::terminal::TerminalLocation>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::terminal::TerminalReaderObject,
    #[doc = "Serial number of the reader."]
    #[json(default)]
    pub serial_number: String,
    #[doc = "The networking status of the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_02::terminal::TerminalReaderStatus>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl crate::PagedItem for TerminalReader {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceCollectPaymentMethodAction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collect_config: Option<stripe_types_05::terminal::TerminalReaderReaderResourceCollectConfig>,
    #[doc = "Most recent PaymentIntent processed by the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<stripe_types_14::payment_intents::PaymentIntent>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<stripe_types_14::payment_methods::PaymentMethod>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceConfirmPaymentIntentAction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirm_config: Option<stripe_types_01::terminal::TerminalReaderReaderResourceConfirmConfig>,
    #[doc = "Most recent PaymentIntent processed by the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<stripe_types_14::payment_intents::PaymentIntent>>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    #[doc = "Most recent PaymentIntent processed by the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<stripe_types_14::payment_intents::PaymentIntent>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub process_config: Option<stripe_types_06::terminal::TerminalReaderReaderResourceProcessConfig>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceProcessSetupIntentAction {
    #[doc = "ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for futur..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub process_config: Option<stripe_types_03::terminal::TerminalReaderReaderResourceProcessSetupConfig>,
    #[doc = "Most recent SetupIntent processed by the reader."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<crate::Expandable<stripe_types_14::setup_intents::SetupIntent>>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceReaderAction {
    #[doc = "The reader action failed due to an [API error](https://docs.stripe.com/api/errors)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub api_error: Option<stripe_types_14::common::ApiErrors>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collect_inputs: Option<stripe_types_12::terminal::TerminalReaderReaderResourceCollectInputsAction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collect_payment_method: Option<crate::terminal::TerminalReaderReaderResourceCollectPaymentMethodAction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirm_payment_intent: Option<crate::terminal::TerminalReaderReaderResourceConfirmPaymentIntentAction>,
    #[doc = "Failure code, only set if status is `failed`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[doc = "Detailed failure message, only set if status is `failed`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub print_content: Option<stripe_types_05::terminal::TerminalReaderReaderResourcePrintContent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub process_payment_intent: Option<crate::terminal::TerminalReaderReaderResourceProcessPaymentIntentAction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub process_setup_intent: Option<crate::terminal::TerminalReaderReaderResourceProcessSetupIntentAction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_payment: Option<crate::terminal::TerminalReaderReaderResourceRefundPaymentAction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub set_reader_display: Option<stripe_types_08::terminal::TerminalReaderReaderResourceSetReaderDisplayAction>,
    #[doc = "Status of the action performed by the reader."]
    #[json(default)]
    pub status: stripe_types_03::terminal::TerminalReaderReaderResourceReaderActionStatus,
    #[doc = "Type of action performed by the reader."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::terminal::TerminalReaderReaderResourceReaderActionType,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceRefundPaymentAction {
    #[doc = "The amount being refunded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "Charge that is being refunded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<stripe_types_14::charges::Charge>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Payment intent that is being refunded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<stripe_types_14::payment_intents::PaymentIntent>>,
    #[doc = "The reason for the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_01::terminal::TerminalReaderReaderResourceRefundPaymentActionReason>,
    #[doc = "Unique identifier for the refund object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund: Option<crate::Expandable<stripe_types_14::refunds::Refund>>,
    #[doc = "Boolean indicating whether the application fee should be refunded when refunding this charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_payment_config: Option<stripe_types_02::terminal::TerminalReaderReaderResourceRefundPaymentConfig>,
    #[doc = "Boolean indicating whether the transfer should be reversed when refunding this charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}
