#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Disabled => "disabled",
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Enabled => "enabled",
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json> for CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Disabled,
            "enabled" => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Enabled,
            other => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode for CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodRemove {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Disabled => {
                "disabled"
            }
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Enabled => {
                "enabled"
            }
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl Default for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove {
    fn default() -> Self {
        CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Unknown(String::new())
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json>
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => {
                CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Disabled
            }
            "enabled" => {
                CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Enabled
            }
            other => {
                CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodRemove::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourcePricingTable {
    #[doc = "Whether the pricing table is enabled."]
    #[json(default)]
    pub enabled: bool,
}
