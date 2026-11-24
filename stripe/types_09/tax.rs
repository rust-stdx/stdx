#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAt {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAtStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsAwStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsAwStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAwStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsBdStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBdStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBdStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBeStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBeStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsBsStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBsStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBsStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBy {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsByType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBy {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCo {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsCoType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCzStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsCzStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCzStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEc {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsEcType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEc {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEeStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsEeStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsEtStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsEtStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEtStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsFrStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsFrStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsFrStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsHuStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsHuStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsHuStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsId {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsIdType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsId {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsIsStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsIsStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsIsStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsKg {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsKgType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsKg {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsKz {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsKzType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsKz {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLtStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsLtStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLtStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMa {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsMaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsMkStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsMkStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMkStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMtStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsMtStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMtStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMy {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsMyType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMy {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsNzStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsNzStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNzStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsPe {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsPeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsPe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsPtStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsPtStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsPtStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSa {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsSaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsSgStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsSgStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSgStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSiStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsSiStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSiStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsTj {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsTjType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsTj {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsTz {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsTzType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsTz {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsUz {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUzType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUz {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsZaStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsZaStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsZaStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsEurope {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub standard: Option<stripe_types_05::tax::TaxProductRegistrationsResourceCountryOptionsEuStandard>,
    #[doc = "Type of registration in an EU country."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsEuropeType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax {
    #[doc = "Elections for the state sales tax registration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub elections:
        Option<Vec<stripe_types_06::tax::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElection>>,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceLineItemTaxBreakdown {
    #[doc = "The amount of tax, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units)."]
    #[json(default)]
    pub amount: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<stripe_types_05::tax::TaxProductResourceJurisdiction>,
    #[doc = "Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's addre..."]
    #[json(default)]
    pub sourcing: stripe_types_02::tax::TaxProductResourceLineItemTaxBreakdownSourcing,
    #[doc = "Details regarding the rate for this tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rate_details: Option<stripe_types_05::tax::TaxProductResourceLineItemTaxRateDetails>,
    #[doc = "The reasoning behind this tax, for example, if the product is tax exempt."]
    #[json(default)]
    pub taxability_reason: stripe_types_01::tax::TaxProductResourceLineItemTaxBreakdownTaxabilityReason,
    #[doc = "The amount on which tax is calculated, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units)."]
    #[json(default)]
    pub taxable_amount: i64,
}
