#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Always => "always",
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Limited => "limited",
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Unspecified => "unspecified",
            CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json>
    for CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Always,
            "limited" => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Limited,
            "unspecified" => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Unspecified,
            other => CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customer_sessions")]
impl crate::params::FormEncode
    for CreateCustomerSessionParamsComponentsCustomerSheetFeaturesPaymentMethodAllowRedisplayFiltersItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerSessionObject {
    CustomerSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CustomerSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerSessionObject::CustomerSession => "customer_session",
            CustomerSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl Default for CustomerSessionObject {
    fn default() -> Self {
        CustomerSessionObject::Unknown(String::new())
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json> for CustomerSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_session" => CustomerSessionObject::CustomerSession,
            other => CustomerSessionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSessionResourceComponentsResourceBuyButton {
    #[doc = "Whether the buy button is enabled."]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Always => "always",
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Limited => "limited",
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Unspecified => "unspecified",
            CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json>
    for CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Always,
            "limited" => CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Limited,
            "unspecified" => CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Unspecified,
            other => CustomerSessionResourceComponentsResourceCustomerSheetResourceFeaturesPaymentMethodAllowRedisplayFiltersItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customer_sessions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customer_sessions")]
impl CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::OffSession => "off_session",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::OnSession => "on_session",
            CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customer_sessions")]
impl<'json> json::Decode<'json>
    for CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::OffSession,
            "on_session" => CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::OnSession,
            other => CustomerSessionResourceComponentsResourcePaymentElementResourceFeaturesPaymentMethodSaveUsage::Unknown(other.to_string()),
        })
    }
}
