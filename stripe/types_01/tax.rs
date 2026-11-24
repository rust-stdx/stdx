#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsAtType {
    Ioss,
    OssNonUnion,
    OssUnion,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsAtType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsAtType::Ioss => "ioss",
            CreateTaxRegistrationParamsCountryOptionsAtType::OssNonUnion => "oss_non_union",
            CreateTaxRegistrationParamsCountryOptionsAtType::OssUnion => "oss_union",
            CreateTaxRegistrationParamsCountryOptionsAtType::Standard => "standard",
            CreateTaxRegistrationParamsCountryOptionsAtType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsAtType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsAtType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ioss" => CreateTaxRegistrationParamsCountryOptionsAtType::Ioss,
            "oss_non_union" => CreateTaxRegistrationParamsCountryOptionsAtType::OssNonUnion,
            "oss_union" => CreateTaxRegistrationParamsCountryOptionsAtType::OssUnion,
            "standard" => CreateTaxRegistrationParamsCountryOptionsAtType::Standard,
            other => CreateTaxRegistrationParamsCountryOptionsAtType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAtType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::LocalUseTax => "local_use_tax",
            CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::SimplifiedSellersUseTax => {
                "simplified_sellers_use_tax"
            }
            CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::SingleLocalUseTax => {
                "single_local_use_tax"
            }
            CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "local_use_tax" => CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::LocalUseTax,
            "simplified_sellers_use_tax" => {
                CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::SimplifiedSellersUseTax
            }
            "single_local_use_tax" => {
                CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::SingleLocalUseTax
            }
            other => {
                CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxSettingParamsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxSettingParamsDefaultsTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxSettingParamsDefaultsTaxBehavior::Exclusive => "exclusive",
            CreateTaxSettingParamsDefaultsTaxBehavior::Inclusive => "inclusive",
            CreateTaxSettingParamsDefaultsTaxBehavior::InferredByCurrency => "inferred_by_currency",
            CreateTaxSettingParamsDefaultsTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxSettingParamsDefaultsTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxSettingParamsDefaultsTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateTaxSettingParamsDefaultsTaxBehavior::Exclusive,
            "inclusive" => CreateTaxSettingParamsDefaultsTaxBehavior::Inclusive,
            "inferred_by_currency" => CreateTaxSettingParamsDefaultsTaxBehavior::InferredByCurrency,
            other => CreateTaxSettingParamsDefaultsTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxSettingParamsDefaultsTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxSettingParamsHeadOfficeAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix,..."]
    pub state: Option<String>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxSettingParamsHeadOfficeAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct GetTaxCalculationParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for GetTaxCalculationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxCalculationLineItemTaxBehavior {
    Exclusive,
    Inclusive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxCalculationLineItemTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxCalculationLineItemTaxBehavior::Exclusive => "exclusive",
            TaxCalculationLineItemTaxBehavior::Inclusive => "inclusive",
            TaxCalculationLineItemTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxCalculationLineItemTaxBehavior {
    fn default() -> Self {
        TaxCalculationLineItemTaxBehavior::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxCalculationLineItemTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxCalculationLineItemTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => TaxCalculationLineItemTaxBehavior::Exclusive,
            "inclusive" => TaxCalculationLineItemTaxBehavior::Inclusive,
            other => TaxCalculationLineItemTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxCalculationObject {
    TaxCalculation,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxCalculationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxCalculationObject::TaxCalculation => "tax.calculation",
            TaxCalculationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxCalculationObject {
    fn default() -> Self {
        TaxCalculationObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxCalculationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxCalculationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.calculation" => TaxCalculationObject::TaxCalculation,
            other => TaxCalculationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxIDsOwnerType {
    Account,
    Application,
    Customer,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl TaxIDsOwnerType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxIDsOwnerType::Account => "account",
            TaxIDsOwnerType::Application => "application",
            TaxIDsOwnerType::Customer => "customer",
            TaxIDsOwnerType::SelfValue => "self",
            TaxIDsOwnerType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl Default for TaxIDsOwnerType {
    fn default() -> Self {
        TaxIDsOwnerType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for TaxIDsOwnerType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxIDsOwnerType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => TaxIDsOwnerType::Account,
            "application" => TaxIDsOwnerType::Application,
            "customer" => TaxIDsOwnerType::Customer,
            "self" => TaxIDsOwnerType::SelfValue,
            other => TaxIDsOwnerType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard {
    #[doc = "Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    #[json(default)]
    pub province: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
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
impl TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::AdmissionsTax => "admissions_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::AttendanceTax => "attendance_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::EntertainmentTax => "entertainment_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::GrossReceiptsTax => "gross_receipts_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::HospitalityTax => "hospitality_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LocalAmusementTax => "local_amusement_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LocalLeaseTax => "local_lease_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LuxuryTax => "luxury_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::MassTransitParkingTax => {
                "mass_transit_parking_tax"
            }
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::ParkingTax => "parking_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::ResortTax => "resort_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateCommunicationsTax => {
                "state_communications_tax"
            }
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateRetailDeliveryFee => {
                "state_retail_delivery_fee"
            }
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateSalesTax => "state_sales_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::TourismTax => "tourism_tax",
            TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsUnitedStatesType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "admissions_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::AdmissionsTax,
            "attendance_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::AttendanceTax,
            "entertainment_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::EntertainmentTax,
            "gross_receipts_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::GrossReceiptsTax,
            "hospitality_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::HospitalityTax,
            "local_amusement_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LocalAmusementTax,
            "local_lease_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LocalLeaseTax,
            "luxury_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::LuxuryTax,
            "mass_transit_parking_tax" => {
                TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::MassTransitParkingTax
            }
            "parking_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::ParkingTax,
            "resort_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::ResortTax,
            "state_communications_tax" => {
                TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateCommunicationsTax
            }
            "state_retail_delivery_fee" => {
                TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateRetailDeliveryFee
            }
            "state_sales_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::StateSalesTax,
            "tourism_tax" => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::TourismTax,
            other => TaxProductRegistrationsResourceCountryOptionsUnitedStatesType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsLuxuryTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=luxury_tax#registration-types) representing the lo..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceCustomerDetailsAddressSource {
    Billing,
    Shipping,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceCustomerDetailsAddressSource {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceCustomerDetailsAddressSource::Billing => "billing",
            TaxProductResourceCustomerDetailsAddressSource::Shipping => "shipping",
            TaxProductResourceCustomerDetailsAddressSource::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceCustomerDetailsAddressSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceCustomerDetailsAddressSource::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing" => TaxProductResourceCustomerDetailsAddressSource::Billing,
            "shipping" => TaxProductResourceCustomerDetailsAddressSource::Shipping,
            other => TaxProductResourceCustomerDetailsAddressSource::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceCustomerDetailsTaxabilityOverride {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceCustomerDetailsTaxabilityOverride::CustomerExempt => "customer_exempt",
            TaxProductResourceCustomerDetailsTaxabilityOverride::None => "none",
            TaxProductResourceCustomerDetailsTaxabilityOverride::ReverseCharge => "reverse_charge",
            TaxProductResourceCustomerDetailsTaxabilityOverride::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn default() -> Self {
        TaxProductResourceCustomerDetailsTaxabilityOverride::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceCustomerDetailsTaxabilityOverride {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceCustomerDetailsTaxabilityOverride::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_exempt" => TaxProductResourceCustomerDetailsTaxabilityOverride::CustomerExempt,
            "none" => TaxProductResourceCustomerDetailsTaxabilityOverride::None,
            "reverse_charge" => TaxProductResourceCustomerDetailsTaxabilityOverride::ReverseCharge,
            other => TaxProductResourceCustomerDetailsTaxabilityOverride::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::CustomerExempt => "customer_exempt",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotCollecting => "not_collecting",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSupported => "not_supported",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExempt => "product_exempt",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProportionallyRated => "proportionally_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReducedRated => "reduced_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReverseCharge => "reverse_charge",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::StandardRated => "standard_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ZeroRated => "zero_rated",
            TaxProductResourceLineItemTaxBreakdownTaxabilityReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn default() -> Self {
        TaxProductResourceLineItemTaxBreakdownTaxabilityReason::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceLineItemTaxBreakdownTaxabilityReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceLineItemTaxBreakdownTaxabilityReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_exempt" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::CustomerExempt,
            "not_collecting" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotCollecting,
            "not_subject_to_tax" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSubjectToTax,
            "not_supported" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::NotSupported,
            "portion_product_exempt" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionProductExempt,
            "portion_reduced_rated" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionReducedRated,
            "portion_standard_rated" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::PortionStandardRated,
            "product_exempt" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExempt,
            "product_exempt_holiday" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProductExemptHoliday,
            "proportionally_rated" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ProportionallyRated,
            "reduced_rated" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReducedRated,
            "reverse_charge" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ReverseCharge,
            "standard_rated" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::StandardRated,
            "taxable_basis_reduced" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::TaxableBasisReduced,
            "zero_rated" => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::ZeroRated,
            other => TaxProductResourceLineItemTaxBreakdownTaxabilityReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceLineItemTaxRateDetailsTaxType {
    AdmissionsTax,
    AmusementTax,
    AttendanceTax,
    CommunicationsTax,
    DigitalExciseTax,
    EntertainmentTax,
    GrossReceiptsTax,
    Gst,
    HospitalityTax,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    LuxuryTax,
    MassTransitParkingTax,
    ParkingTax,
    Pst,
    Qst,
    RecyclingFee,
    ResortTax,
    RetailDeliveryFee,
    Rst,
    SalesTax,
    ServiceTax,
    TourismTax,
    UtilityUsersTax,
    Vat,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceLineItemTaxRateDetailsTaxType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceLineItemTaxRateDetailsTaxType::AdmissionsTax => "admissions_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::AmusementTax => "amusement_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::AttendanceTax => "attendance_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::CommunicationsTax => "communications_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::DigitalExciseTax => "digital_excise_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::EntertainmentTax => "entertainment_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::GrossReceiptsTax => "gross_receipts_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Gst => "gst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::HospitalityTax => "hospitality_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Hst => "hst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Igst => "igst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Jct => "jct",
            TaxProductResourceLineItemTaxRateDetailsTaxType::LeaseTax => "lease_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::LuxuryTax => "luxury_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::MassTransitParkingTax => "mass_transit_parking_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::ParkingTax => "parking_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Pst => "pst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Qst => "qst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::RecyclingFee => "recycling_fee",
            TaxProductResourceLineItemTaxRateDetailsTaxType::ResortTax => "resort_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::RetailDeliveryFee => "retail_delivery_fee",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Rst => "rst",
            TaxProductResourceLineItemTaxRateDetailsTaxType::SalesTax => "sales_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::ServiceTax => "service_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::TourismTax => "tourism_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::UtilityUsersTax => "utility_users_tax",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Vat => "vat",
            TaxProductResourceLineItemTaxRateDetailsTaxType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn default() -> Self {
        TaxProductResourceLineItemTaxRateDetailsTaxType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceLineItemTaxRateDetailsTaxType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceLineItemTaxRateDetailsTaxType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "admissions_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::AdmissionsTax,
            "amusement_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::AmusementTax,
            "attendance_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::AttendanceTax,
            "communications_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::CommunicationsTax,
            "digital_excise_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::DigitalExciseTax,
            "entertainment_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::EntertainmentTax,
            "gross_receipts_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::GrossReceiptsTax,
            "gst" => TaxProductResourceLineItemTaxRateDetailsTaxType::Gst,
            "hospitality_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::HospitalityTax,
            "hst" => TaxProductResourceLineItemTaxRateDetailsTaxType::Hst,
            "igst" => TaxProductResourceLineItemTaxRateDetailsTaxType::Igst,
            "jct" => TaxProductResourceLineItemTaxRateDetailsTaxType::Jct,
            "lease_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::LeaseTax,
            "luxury_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::LuxuryTax,
            "mass_transit_parking_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::MassTransitParkingTax,
            "parking_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::ParkingTax,
            "pst" => TaxProductResourceLineItemTaxRateDetailsTaxType::Pst,
            "qst" => TaxProductResourceLineItemTaxRateDetailsTaxType::Qst,
            "recycling_fee" => TaxProductResourceLineItemTaxRateDetailsTaxType::RecyclingFee,
            "resort_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::ResortTax,
            "retail_delivery_fee" => TaxProductResourceLineItemTaxRateDetailsTaxType::RetailDeliveryFee,
            "rst" => TaxProductResourceLineItemTaxRateDetailsTaxType::Rst,
            "sales_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::SalesTax,
            "service_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::ServiceTax,
            "tourism_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::TourismTax,
            "utility_users_tax" => TaxProductResourceLineItemTaxRateDetailsTaxType::UtilityUsersTax,
            "vat" => TaxProductResourceLineItemTaxRateDetailsTaxType::Vat,
            other => TaxProductResourceLineItemTaxRateDetailsTaxType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    Exclusive,
    Inclusive,
    InferredByCurrency,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "tax"))]
impl TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::Exclusive => "exclusive",
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::Inclusive => "inclusive",
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::InferredByCurrency => "inferred_by_currency",
            TaxProductResourceTaxSettingsDefaultsTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl<'json> json::Decode<'json> for TaxProductResourceTaxSettingsDefaultsTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceTaxSettingsDefaultsTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => TaxProductResourceTaxSettingsDefaultsTaxBehavior::Exclusive,
            "inclusive" => TaxProductResourceTaxSettingsDefaultsTaxBehavior::Inclusive,
            "inferred_by_currency" => TaxProductResourceTaxSettingsDefaultsTaxBehavior::InferredByCurrency,
            other => TaxProductResourceTaxSettingsDefaultsTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxSettingsStatusDetailsResourcePending {
    #[doc = "The list of missing fields that are required to perform calculations."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub missing_fields: Option<Vec<String>>,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxTransactionLineItemResourceReversal {
    #[doc = "The `id` of the line item to reverse in the original transaction."]
    #[json(default)]
    pub original_line_item: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxRegistrationStatus {
    Active,
    Expired,
    Scheduled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxRegistrationStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxRegistrationStatus::Active => "active",
            TaxRegistrationStatus::Expired => "expired",
            TaxRegistrationStatus::Scheduled => "scheduled",
            TaxRegistrationStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxRegistrationStatus {
    fn default() -> Self {
        TaxRegistrationStatus::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxRegistrationStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxRegistrationStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => TaxRegistrationStatus::Active,
            "expired" => TaxRegistrationStatus::Expired,
            "scheduled" => TaxRegistrationStatus::Scheduled,
            other => TaxRegistrationStatus::Unknown(other.to_string()),
        })
    }
}
