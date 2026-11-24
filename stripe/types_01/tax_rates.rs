#[cfg(feature = "tax_rates")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRateParamsTaxType {
    AmusementTax,
    CommunicationsTax,
    DigitalExciseTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    MassTransitParkingTax,
    ParkingTax,
    Pst,
    Qst,
    RetailDeliveryFee,
    Rst,
    SalesTax,
    ServiceTax,
    UtilityUsersTax,
    Vat,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax_rates")]
impl CreateTaxRateParamsTaxType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRateParamsTaxType::AmusementTax => "amusement_tax",
            CreateTaxRateParamsTaxType::CommunicationsTax => "communications_tax",
            CreateTaxRateParamsTaxType::DigitalExciseTax => "digital_excise_tax",
            CreateTaxRateParamsTaxType::Gst => "gst",
            CreateTaxRateParamsTaxType::Hst => "hst",
            CreateTaxRateParamsTaxType::Igst => "igst",
            CreateTaxRateParamsTaxType::Jct => "jct",
            CreateTaxRateParamsTaxType::LeaseTax => "lease_tax",
            CreateTaxRateParamsTaxType::MassTransitParkingTax => "mass_transit_parking_tax",
            CreateTaxRateParamsTaxType::ParkingTax => "parking_tax",
            CreateTaxRateParamsTaxType::Pst => "pst",
            CreateTaxRateParamsTaxType::Qst => "qst",
            CreateTaxRateParamsTaxType::RetailDeliveryFee => "retail_delivery_fee",
            CreateTaxRateParamsTaxType::Rst => "rst",
            CreateTaxRateParamsTaxType::SalesTax => "sales_tax",
            CreateTaxRateParamsTaxType::ServiceTax => "service_tax",
            CreateTaxRateParamsTaxType::UtilityUsersTax => "utility_users_tax",
            CreateTaxRateParamsTaxType::Vat => "vat",
            CreateTaxRateParamsTaxType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax_rates")]
impl<'json> json::Decode<'json> for CreateTaxRateParamsTaxType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRateParamsTaxType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amusement_tax" => CreateTaxRateParamsTaxType::AmusementTax,
            "communications_tax" => CreateTaxRateParamsTaxType::CommunicationsTax,
            "digital_excise_tax" => CreateTaxRateParamsTaxType::DigitalExciseTax,
            "gst" => CreateTaxRateParamsTaxType::Gst,
            "hst" => CreateTaxRateParamsTaxType::Hst,
            "igst" => CreateTaxRateParamsTaxType::Igst,
            "jct" => CreateTaxRateParamsTaxType::Jct,
            "lease_tax" => CreateTaxRateParamsTaxType::LeaseTax,
            "mass_transit_parking_tax" => CreateTaxRateParamsTaxType::MassTransitParkingTax,
            "parking_tax" => CreateTaxRateParamsTaxType::ParkingTax,
            "pst" => CreateTaxRateParamsTaxType::Pst,
            "qst" => CreateTaxRateParamsTaxType::Qst,
            "retail_delivery_fee" => CreateTaxRateParamsTaxType::RetailDeliveryFee,
            "rst" => CreateTaxRateParamsTaxType::Rst,
            "sales_tax" => CreateTaxRateParamsTaxType::SalesTax,
            "service_tax" => CreateTaxRateParamsTaxType::ServiceTax,
            "utility_users_tax" => CreateTaxRateParamsTaxType::UtilityUsersTax,
            "vat" => CreateTaxRateParamsTaxType::Vat,
            other => CreateTaxRateParamsTaxType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax_rates")]
impl crate::params::FormEncode for CreateTaxRateParamsTaxType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax_rates")]
#[derive(Debug, Clone, Default)]
pub struct ListTaxRatesParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "tax_rates")]
impl crate::params::FormEncode for ListTaxRatesParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
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
    feature = "tax_rates",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxRateJurisdictionLevel {
    City,
    Country,
    County,
    District,
    Multiple,
    State,
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
    feature = "tax_rates",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl TaxRateJurisdictionLevel {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxRateJurisdictionLevel::City => "city",
            TaxRateJurisdictionLevel::Country => "country",
            TaxRateJurisdictionLevel::County => "county",
            TaxRateJurisdictionLevel::District => "district",
            TaxRateJurisdictionLevel::Multiple => "multiple",
            TaxRateJurisdictionLevel::State => "state",
            TaxRateJurisdictionLevel::Unknown(value) => value,
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
    feature = "tax_rates",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for TaxRateJurisdictionLevel {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxRateJurisdictionLevel::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "city" => TaxRateJurisdictionLevel::City,
            "country" => TaxRateJurisdictionLevel::Country,
            "county" => TaxRateJurisdictionLevel::County,
            "district" => TaxRateJurisdictionLevel::District,
            "multiple" => TaxRateJurisdictionLevel::Multiple,
            "state" => TaxRateJurisdictionLevel::State,
            other => TaxRateJurisdictionLevel::Unknown(other.to_string()),
        })
    }
}
