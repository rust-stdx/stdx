#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAeStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsAlStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAlStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAlStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsAuStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAuStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAuStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsBbStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBbStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBbStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsBhStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBhStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBhStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBj {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBjType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBj {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsChStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsChStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsChStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCm {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsCmType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCv {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsCvType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCv {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCyStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsCyStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCyStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsDkStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsDkStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsDkStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsFiStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsFiStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsFiStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsGe {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsGeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsGnStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsGnStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGnStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsHrStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsHrStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsHrStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsItStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsItStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsItStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsKe {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsKeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsKe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsKr {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsKrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsKr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLk {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsLkType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLvStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsLvStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLvStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsMeStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsMeStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMx {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsMxType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsNoStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsNoStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNoStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsNp {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsNpType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsPlStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsPlStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsPlStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsRsStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsRsStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsRsStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsRu {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsRuType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsRu {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSeStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsSeStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsTh {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsThType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsTh {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsTw {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsTwType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsTw {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsUg {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUgType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUg {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsUyStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUyStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUyStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsZm {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsZmType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsZm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxSettingParams {
    #[doc = "Default configuration to be used on Stripe Tax calculations."]
    pub defaults: Option<stripe_types_05::tax::CreateTaxSettingParamsDefaults>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The place where your business is located."]
    pub head_office: Option<stripe_types_06::tax::CreateTaxSettingParamsHeadOffice>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxSettingParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("defaults", self.defaults.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("head_office", self.head_office.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultInboundGoods {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub standard: Option<stripe_types_06::tax::TaxProductRegistrationsResourceCountryOptionsDefaultStandard>,
    #[doc = "Type of registration in `country`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::tax::TaxProductRegistrationsResourceCountryOptionsDefaultType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsSpain {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub standard: Option<stripe_types_05::tax::TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    #[doc = "Type of registration in ES."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsEuropeType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxAssociationTransactionAttempts {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub committed: Option<stripe_types_02::tax::TaxProductResourceTaxAssociationTransactionAttemptsResourceCommitted>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub errored: Option<stripe_types_05::tax::TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored>,
    #[doc = "The source of the tax transaction attempt."]
    #[json(default)]
    pub source: String,
    #[doc = "The status of the transaction attempt."]
    #[json(default)]
    pub status: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxBreakdown {
    #[doc = "The amount of tax, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Specifies whether the tax amount is included in the line item amount."]
    #[json(default)]
    pub inclusive: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rate_details: Option<stripe_types_06::tax::TaxProductResourceTaxRateDetails>,
    #[doc = "The reasoning behind this tax, for example, if the product is tax exempt."]
    #[json(default)]
    pub taxability_reason: stripe_types_01::tax::TaxProductResourceLineItemTaxBreakdownTaxabilityReason,
    #[doc = "The amount on which tax is calculated, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units)."]
    #[json(default)]
    pub taxable_amount: i64,
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxSettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<stripe_types_05::tax::TaxProductResourceTaxSettingsDefaults>,
    #[doc = "The place where your business is located."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub head_office: Option<stripe_types_05::tax::TaxProductResourceTaxSettingsHeadOffice>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::tax::TaxSettingsObject,
    #[doc = "Whether these settings have the information Stripe Tax needs to calculate tax."]
    #[json(default)]
    pub status: stripe_types_04::tax::TaxSettingsStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_05::tax::TaxProductResourceTaxSettingsStatusDetails>,
}
