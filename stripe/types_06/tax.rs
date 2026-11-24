#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxCalculationParamsCustomerDetailsTaxIdsItem {
    #[doc = "Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`,..."]
    pub r#type: stripe_types_03::tax::CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType,
    #[doc = "Value of the tax ID."]
    pub value: String,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsCustomerDetailsTaxIdsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxCalculationParamsShipFromDetailsAddress =
    stripe_types_04::tax::CreateTaxCalculationParamsCustomerDetailsAddress;

#[cfg(feature = "tax")]
pub type CreateTaxCalculationParamsShippingCostTaxBehavior =
    stripe_types_02::tax::CreateTaxCalculationParamsLineItemsItemTaxBehavior;

#[cfg(feature = "tax")]
pub type CreateTaxLocationParamsAddress = stripe_types_04::tax::CreateTaxCalculationParamsCustomerDetailsAddress;

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAm {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAoStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAoType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAwStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAwType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBaType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBbStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBdType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBeStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBfType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBgStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBhType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBsStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBsType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCa {
    #[doc = "Options for the provincial tax registration."]
    pub province_standard: Option<stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsCaProvinceStandard>,
    #[doc = "Type of registration to be created in Canada."]
    pub r#type: stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsCaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("province_standard", self.province_standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCdType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsChStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsClType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCoType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCvType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCzStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCzType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsDkStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsDkType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEeType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEsStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEsType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsFiStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsFiType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGbStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGbType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGnType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGrStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsHrType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsHuStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsIdType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsInType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsIsStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsItType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsJpStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsKeType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsKhType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsKzType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLkType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLuStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLuType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMaType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMeStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMeType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMrStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMrType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMxType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNgType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNlStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNoType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNzStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNzType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsPeType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsPlStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsPlType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsRoStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsRoType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsRuType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSeStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSeType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSiStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSiType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSnType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSrStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsThType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsTrType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsTzType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUgType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsEntertainmentTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsHospitalityTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsLocalAmusementTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsLocalLeaseTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsMassTransitParkingTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsParkingTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsResortTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUzType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsZaStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsZaType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsZwType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxSettingParamsHeadOffice {
    #[doc = "The location of the business for tax purposes."]
    pub address: stripe_types_01::tax::CreateTaxSettingParamsHeadOfficeAddress,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxSettingParamsHeadOffice {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
    }
}

#[cfg(feature = "tax")]
pub type GetTaxLocationParams = stripe_types_01::tax::GetTaxCalculationParams;

#[cfg(feature = "tax")]
pub type GetTaxSettingParams = stripe_types_01::tax::GetTaxCalculationParams;

#[cfg(feature = "tax")]
pub type GetTaxTransactionParams = stripe_types_01::tax::GetTaxCalculationParams;

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct ListTaxLocationsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub r#type: stripe_types_04::tax::ListTaxLocationsParamsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for ListTaxLocationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
pub type ListTaxTransactionLineItemsParams = stripe_types_02::tax::ListTaxCalculationLineItemsParams;

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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxDeductedAtSource {
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::tax::TaxDeductedAtSourceObject,
    #[doc = "The end of the invoicing period."]
    #[json(default)]
    pub period_end: i64,
    #[doc = "The start of the invoicing period."]
    #[json(default)]
    pub period_start: i64,
    #[doc = "The TAN that was supplied to Stripe when TDS was assessed"]
    #[json(default)]
    pub tax_deduction_account_number: String,
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
impl crate::PagedItem for TaxDeductedAtSource {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxLocation {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "A descriptive text providing additional context about the tax location."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::tax::TaxLocationObject,
    #[doc = "The type of tax location to be defined."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::tax::TaxLocationType,
}

#[cfg(feature = "tax")]
impl crate::PagedItem for TaxLocation {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsCanada {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub province_standard:
        Option<stripe_types_01::tax::TaxProductRegistrationsResourceCountryOptionsCaProvinceStandard>,
    #[doc = "Type of registration in Canada."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsCanadaType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefault {
    #[doc = "Type of registration in `country`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::tax::TaxProductRegistrationsResourceCountryOptionsDefaultType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultStandard {
    #[doc = "Place of supply scheme used in an Default standard registration."]
    #[json(default)]
    pub place_of_supply_scheme:
        stripe_types_04::tax::TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsThailand {
    #[doc = "Type of registration in `country`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsSimplifiedType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection {
    #[doc = "A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    #[doc = "The type of the election for the state sales tax registration."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::tax::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceShipFromDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_00::tax::TaxProductResourcePostalAddress>,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxRateDetails {
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The amount of the tax rate when the `rate_type` is `flat_amount`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<stripe_types_00::tax_rates::TaxRateFlatAmount>,
    #[doc = "The tax rate percentage as a string."]
    #[json(default)]
    pub percentage_decimal: String,
    #[doc = "Indicates the type of tax rate applied to the taxable amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rate_type: Option<stripe_types_02::tax::TaxProductResourceTaxRateDetailsRateType>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The tax type, such as `vat` or `sales_tax`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<stripe_types_01::tax::TaxProductResourceLineItemTaxRateDetailsTaxType>,
}
