#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateFromCalculationParams {
    #[doc = "Tax Calculation ID to be used as input when creating the transaction."]
    pub calculation: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The Unix timestamp representing when the tax liability is assumed or reduced, which determines the liability posting ..."]
    pub posted_at: Option<i64>,
    #[doc = "A custom order or sale identifier, such as 'myOrder_123'."]
    pub reference: String,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateFromCalculationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("calculation", &self.calculation);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("posted_at", self.posted_at.as_ref());
        form.field("reference", &self.reference);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxCalculationParamsLineItemsItemTaxBehavior {
    Exclusive,
    Inclusive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxCalculationParamsLineItemsItemTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxCalculationParamsLineItemsItemTaxBehavior::Exclusive => "exclusive",
            CreateTaxCalculationParamsLineItemsItemTaxBehavior::Inclusive => "inclusive",
            CreateTaxCalculationParamsLineItemsItemTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxCalculationParamsLineItemsItemTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxCalculationParamsLineItemsItemTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateTaxCalculationParamsLineItemsItemTaxBehavior::Exclusive,
            "inclusive" => CreateTaxCalculationParamsLineItemsItemTaxBehavior::Inclusive,
            other => CreateTaxCalculationParamsLineItemsItemTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsLineItemsItemTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct GetTaxAssociationFindParams {
    pub expand: Option<Vec<String>>,
    pub payment_intent: String,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for GetTaxAssociationFindParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("payment_intent", &self.payment_intent);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct ListTaxCalculationLineItemsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for ListTaxCalculationLineItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsCanadaType {
    ProvinceStandard,
    Simplified,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsCanadaType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsCanadaType::ProvinceStandard => "province_standard",
            TaxProductRegistrationsResourceCountryOptionsCanadaType::Simplified => "simplified",
            TaxProductRegistrationsResourceCountryOptionsCanadaType::Standard => "standard",
            TaxProductRegistrationsResourceCountryOptionsCanadaType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsCanadaType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsCanadaType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductRegistrationsResourceCountryOptionsCanadaType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "province_standard" => TaxProductRegistrationsResourceCountryOptionsCanadaType::ProvinceStandard,
            "simplified" => TaxProductRegistrationsResourceCountryOptionsCanadaType::Simplified,
            "standard" => TaxProductRegistrationsResourceCountryOptionsCanadaType::Standard,
            other => TaxProductRegistrationsResourceCountryOptionsCanadaType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsEuropeType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsEuropeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsEuropeType::Ioss => "ioss",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::OssNonUnion => "oss_non_union",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::OssUnion => "oss_union",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::Standard => "standard",
            TaxProductRegistrationsResourceCountryOptionsEuropeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsEuropeType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsEuropeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductRegistrationsResourceCountryOptionsEuropeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ioss" => TaxProductRegistrationsResourceCountryOptionsEuropeType::Ioss,
            "oss_non_union" => TaxProductRegistrationsResourceCountryOptionsEuropeType::OssNonUnion,
            "oss_union" => TaxProductRegistrationsResourceCountryOptionsEuropeType::OssUnion,
            "standard" => TaxProductRegistrationsResourceCountryOptionsEuropeType::Standard,
            other => TaxProductRegistrationsResourceCountryOptionsEuropeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsAttendanceTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=attendance_tax#registration-types) representing th..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax {
    #[doc = "A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=mass_transit_parking_tax#registration-types) repre..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsResortTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=resort_tax#registration-types) representing the lo..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceLineItemTaxBreakdownSourcing {
    Destination,
    Origin,
    Performance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceLineItemTaxBreakdownSourcing {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceLineItemTaxBreakdownSourcing::Destination => "destination",
            TaxProductResourceLineItemTaxBreakdownSourcing::Origin => "origin",
            TaxProductResourceLineItemTaxBreakdownSourcing::Performance => "performance",
            TaxProductResourceLineItemTaxBreakdownSourcing::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn default() -> Self {
        TaxProductResourceLineItemTaxBreakdownSourcing::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceLineItemTaxBreakdownSourcing::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "destination" => TaxProductResourceLineItemTaxBreakdownSourcing::Destination,
            "origin" => TaxProductResourceLineItemTaxBreakdownSourcing::Origin,
            "performance" => TaxProductResourceLineItemTaxBreakdownSourcing::Performance,
            other => TaxProductResourceLineItemTaxBreakdownSourcing::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted {
    #[doc = "The [Tax Transaction](https://docs.stripe.com/api/tax/transaction/object)"]
    #[json(default)]
    pub transaction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceTaxRateDetailsRateType {
    FlatAmount,
    Percentage,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceTaxRateDetailsRateType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceTaxRateDetailsRateType::FlatAmount => "flat_amount",
            TaxProductResourceTaxRateDetailsRateType::Percentage => "percentage",
            TaxProductResourceTaxRateDetailsRateType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceTaxRateDetailsRateType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceTaxRateDetailsRateType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "flat_amount" => TaxProductResourceTaxRateDetailsRateType::FlatAmount,
            "percentage" => TaxProductResourceTaxRateDetailsRateType::Percentage,
            other => TaxProductResourceTaxRateDetailsRateType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxRegistrationObject {
    TaxRegistration,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxRegistrationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxRegistrationObject::TaxRegistration => "tax.registration",
            TaxRegistrationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxRegistrationObject {
    fn default() -> Self {
        TaxRegistrationObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxRegistrationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxRegistrationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.registration" => TaxRegistrationObject::TaxRegistration,
            other => TaxRegistrationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxTransactionObject {
    TaxTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxTransactionObject::TaxTransaction => "tax.transaction",
            TaxTransactionObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxTransactionObject {
    fn default() -> Self {
        TaxTransactionObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.transaction" => TaxTransactionObject::TaxTransaction,
            other => TaxTransactionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxTransactionType {
    Reversal,
    Transaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxTransactionType::Reversal => "reversal",
            TaxTransactionType::Transaction => "transaction",
            TaxTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxTransactionType {
    fn default() -> Self {
        TaxTransactionType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reversal" => TaxTransactionType::Reversal,
            "transaction" => TaxTransactionType::Transaction,
            other => TaxTransactionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTaxRegistrationParams {
    #[doc = "Time at which the registration becomes active."]
    pub active_from: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "If set, the registration stops being active at this time."]
    pub expires_at: Option<json::OwnedValue>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for UpdateTaxRegistrationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
