#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CollectInputsParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "List of inputs to be collected from the customer using the Reader."]
    pub inputs: Vec<stripe_types_10::terminal::CollectInputsParamsInputsItem>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        for (index, item) in self.inputs.iter().enumerate() {
            form.field_indexed("inputs", index, item);
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub enum GetTerminalConfigurationResponseExpanded {
    /// Expanded `terminal.configuration`.
    TerminalConfiguration(Box<stripe_types_10::terminal::TerminalConfiguration>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for GetTerminalConfigurationResponseExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "terminal.configuration" => GetTerminalConfigurationResponseExpanded::TerminalConfiguration(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => GetTerminalConfigurationResponseExpanded::Other(value),
        })
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceCollectInputsAction {
    #[doc = "List of inputs to be collected."]
    #[json(default)]
    pub inputs: Vec<stripe_types_10::terminal::TerminalReaderReaderResourceInput>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
