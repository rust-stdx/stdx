#[cfg(feature = "transfers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTransferParamsSourceType {
    BankAccount,
    Card,
    Fpx,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "transfers")]
impl CreateTransferParamsSourceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTransferParamsSourceType::BankAccount => "bank_account",
            CreateTransferParamsSourceType::Card => "card",
            CreateTransferParamsSourceType::Fpx => "fpx",
            CreateTransferParamsSourceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "transfers")]
impl<'json> json::Decode<'json> for CreateTransferParamsSourceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTransferParamsSourceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_account" => CreateTransferParamsSourceType::BankAccount,
            "card" => CreateTransferParamsSourceType::Card,
            "fpx" => CreateTransferParamsSourceType::Fpx,
            other => CreateTransferParamsSourceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "transfers")]
impl crate::params::FormEncode for CreateTransferParamsSourceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "transfers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTransferReversalParams {
    #[doc = "A positive integer in cents (or local equivalent) representing how much of this transfer to reverse."]
    pub amount: Option<i64>,
    #[doc = "An arbitrary string which you can attach to a reversal object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Boolean indicating whether the application fee should be refunded when reversing this transfer."]
    pub refund_application_fee: Option<bool>,
}

#[cfg(feature = "transfers")]
impl crate::params::FormEncode for CreateTransferReversalParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("refund_application_fee", self.refund_application_fee.as_ref());
    }
}

#[cfg(feature = "transfers")]
#[derive(Debug, Clone, Default)]
pub struct ListTransferReversalsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "transfers")]
impl crate::params::FormEncode for ListTransferReversalsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "transfers")]
#[derive(Debug, Clone, Default)]
pub struct ListTransfersParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "transfers")]
impl crate::params::FormEncode for ListTransfersParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
