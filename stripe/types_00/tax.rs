#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxCalculationParamsCustomerDetailsAddressSource {
    Billing,
    Shipping,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxCalculationParamsCustomerDetailsAddressSource {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxCalculationParamsCustomerDetailsAddressSource::Billing => "billing",
            CreateTaxCalculationParamsCustomerDetailsAddressSource::Shipping => "shipping",
            CreateTaxCalculationParamsCustomerDetailsAddressSource::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxCalculationParamsCustomerDetailsAddressSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxCalculationParamsCustomerDetailsAddressSource::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing" => CreateTaxCalculationParamsCustomerDetailsAddressSource::Billing,
            "shipping" => CreateTaxCalculationParamsCustomerDetailsAddressSource::Shipping,
            other => CreateTaxCalculationParamsCustomerDetailsAddressSource::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsCustomerDetailsAddressSource {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::InboundGoods => "inbound_goods",
            CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::Standard => "standard",
            CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound_goods" => CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::InboundGoods,
            "standard" => CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::Standard,
            other => CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme {
    InboundGoods,
    SmallSeller,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::InboundGoods => "inbound_goods",
            CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::SmallSeller => "small_seller",
            CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::Standard => "standard",
            CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound_goods" => CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::InboundGoods,
            "small_seller" => CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::SmallSeller,
            "standard" => CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::Standard,
            other => CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsUsType {
    AdmissionsTax,
    AttendanceTax,
    EntertainmentTax,
    GrossReceiptsTax,
    HospitalityTax,
    LocalAmusementTax,
    LocalLeaseTax,
    LuxuryTax,
    MassTransitParkingTax,
    ParkingTax,
    ResortTax,
    StateCommunicationsTax,
    StateRetailDeliveryFee,
    StateSalesTax,
    TourismTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsUsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsUsType::AdmissionsTax => "admissions_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::AttendanceTax => "attendance_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::EntertainmentTax => "entertainment_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::GrossReceiptsTax => "gross_receipts_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::HospitalityTax => "hospitality_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::LocalAmusementTax => "local_amusement_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::LocalLeaseTax => "local_lease_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::LuxuryTax => "luxury_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::MassTransitParkingTax => "mass_transit_parking_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::ParkingTax => "parking_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::ResortTax => "resort_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::StateCommunicationsTax => "state_communications_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::StateRetailDeliveryFee => "state_retail_delivery_fee",
            CreateTaxRegistrationParamsCountryOptionsUsType::StateSalesTax => "state_sales_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::TourismTax => "tourism_tax",
            CreateTaxRegistrationParamsCountryOptionsUsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsUsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsUsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "admissions_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::AdmissionsTax,
            "attendance_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::AttendanceTax,
            "entertainment_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::EntertainmentTax,
            "gross_receipts_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::GrossReceiptsTax,
            "hospitality_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::HospitalityTax,
            "local_amusement_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::LocalAmusementTax,
            "local_lease_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::LocalLeaseTax,
            "luxury_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::LuxuryTax,
            "mass_transit_parking_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::MassTransitParkingTax,
            "parking_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::ParkingTax,
            "resort_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::ResortTax,
            "state_communications_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::StateCommunicationsTax,
            "state_retail_delivery_fee" => CreateTaxRegistrationParamsCountryOptionsUsType::StateRetailDeliveryFee,
            "state_sales_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::StateSalesTax,
            "tourism_tax" => CreateTaxRegistrationParamsCountryOptionsUsType::TourismTax,
            other => CreateTaxRegistrationParamsCountryOptionsUsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxAssociationObject {
    TaxAssociation,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxAssociationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxAssociationObject::TaxAssociation => "tax.association",
            TaxAssociationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxAssociationObject {
    fn default() -> Self {
        TaxAssociationObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxAssociationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxAssociationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.association" => TaxAssociationObject::TaxAssociation,
            other => TaxAssociationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    InboundGoods,
    SmallSeller,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::InboundGoods => "inbound_goods",
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::SmallSeller => "small_seller",
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Standard => "standard",
            TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound_goods" => TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::InboundGoods,
            "small_seller" => TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::SmallSeller,
            "standard" => TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Standard,
            other => {
                TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    Simplified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Simplified => "simplified",
            TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsSimplifiedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "simplified" => TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Simplified,
            other => TaxProductRegistrationsResourceCountryOptionsSimplifiedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsAdmissionsTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=admissions_tax#registration-types) representing th..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsGrossReceiptsTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=gross_receipts_tax#registration-types) representin..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax {
    #[doc = "A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsTourismTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=tourism_tax#registration-types) representing the l..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceJurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceJurisdictionLevel {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceJurisdictionLevel::City => "city",
            TaxProductResourceJurisdictionLevel::Country => "country",
            TaxProductResourceJurisdictionLevel::County => "county",
            TaxProductResourceJurisdictionLevel::District => "district",
            TaxProductResourceJurisdictionLevel::State => "state",
            TaxProductResourceJurisdictionLevel::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceJurisdictionLevel {
    fn default() -> Self {
        TaxProductResourceJurisdictionLevel::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceJurisdictionLevel {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceJurisdictionLevel::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "city" => TaxProductResourceJurisdictionLevel::City,
            "country" => TaxProductResourceJurisdictionLevel::Country,
            "county" => TaxProductResourceJurisdictionLevel::County,
            "district" => TaxProductResourceJurisdictionLevel::District,
            "state" => TaxProductResourceJurisdictionLevel::State,
            other => TaxProductResourceJurisdictionLevel::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourcePostalAddress {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default)]
    pub country: String,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix,..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourceActive {}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxTransactionLineItemObject {
    TaxTransactionLineItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxTransactionLineItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxTransactionLineItemObject::TaxTransactionLineItem => "tax.transaction_line_item",
            TaxTransactionLineItemObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxTransactionLineItemObject {
    fn default() -> Self {
        TaxTransactionLineItemObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxTransactionLineItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxTransactionLineItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.transaction_line_item" => TaxTransactionLineItemObject::TaxTransactionLineItem,
            other => TaxTransactionLineItemObject::Unknown(other.to_string()),
        })
    }
}
