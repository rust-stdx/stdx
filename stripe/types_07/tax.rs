#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxCalculationParamsCustomerDetails {
    #[doc = "The customer's postal address (for example, home or business location)."]
    pub address: Option<stripe_types_04::tax::CreateTaxCalculationParamsCustomerDetailsAddress>,
    #[doc = "The type of customer address provided."]
    pub address_source: Option<stripe_types_00::tax::CreateTaxCalculationParamsCustomerDetailsAddressSource>,
    #[doc = "The customer's IP address (IPv4 or IPv6)."]
    pub ip_address: Option<String>,
    #[doc = "The customer's tax IDs."]
    pub tax_ids: Option<Vec<stripe_types_06::tax::CreateTaxCalculationParamsCustomerDetailsTaxIdsItem>>,
    #[doc = "Overrides the tax calculation result to allow you to not collect tax from your customer."]
    pub taxability_override: Option<stripe_types_03::tax::CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsCustomerDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_source", self.address_source.as_ref());
        form.field_opt("ip_address", self.ip_address.as_ref());
        if let Some(values) = self.tax_ids.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tax_ids", index, item);
            }
        }
        form.field_opt("taxability_override", self.taxability_override.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxCalculationParamsShipFromDetails {
    #[doc = "The address from which the goods are being shipped from."]
    pub address: stripe_types_06::tax::CreateTaxCalculationParamsShipFromDetailsAddress,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsShipFromDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxCalculationParamsShippingCost {
    #[doc = "A positive integer in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units) representing the s..."]
    pub amount: Option<i64>,
    #[doc = "If provided, the [shipping rate](https://docs.stripe.com/api/shipping_rates/object)'s `amount`, `tax_code` and `tax_b..."]
    pub shipping_rate: Option<String>,
    #[doc = "Specifies whether the `amount` includes taxes."]
    pub tax_behavior: Option<stripe_types_06::tax::CreateTaxCalculationParamsShippingCostTaxBehavior>,
    #[doc = "The [tax code](https://docs.stripe.com/tax/tax-categories) used to calculate tax on shipping."]
    pub tax_code: Option<String>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsShippingCost {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("shipping_rate", self.shipping_rate.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxLocationParams {
    #[doc = "The physical address of the tax location."]
    pub address: stripe_types_06::tax::CreateTaxLocationParamsAddress,
    #[doc = "Details to identify the tax location by its venue, types of events held, or available services, such as 'A spacious a..."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The type of tax location."]
    pub r#type: stripe_types_05::tax::CreateTaxLocationParamsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxLocationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsAoStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsAoStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAoStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAz {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAzType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAz {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsBaStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBaStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBaStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsBfStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBfStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBfStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBgStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBgStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBgStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsCdStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsCdStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCdStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCl {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsClType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCl {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCr {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsCrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsDeStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsDeStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsDeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEg {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsEgType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEg {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEsStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsEsStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEsStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsGbStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsGbStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGbStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsGrStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsGrStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGrStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsIeStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsIeStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsIeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsIn {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsInType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsIn {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsJpStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsJpStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsJpStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsKh {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsKhType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsKh {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLa {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsLaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLuStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsLuStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLuStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMd {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsMdType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsMrStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsMrStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMrStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsNg {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsNgType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNg {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsNlStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsNlStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNlStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsOmStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsOmStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsOmStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsPh {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsPhType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsPh {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsRoStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsRoStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsRoStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSkStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsSkStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSkStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSn {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsSnType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSn {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsSrStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsSrStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSrStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsTr {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsTrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsTr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsUa {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsUsStateSalesTax {
    #[doc = "Elections for the state sales tax registration."]
    pub elections: Vec<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItem>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUsStateSalesTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        for (index, item) in self.elections.iter().enumerate() {
            form.field_indexed("elections", index, item);
        }
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsVn {
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsVnType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsVn {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsZwStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsZwStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsZwStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceCustomerDetails {
    #[doc = "The customer's postal address (for example, home or business location)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_00::tax::TaxProductResourcePostalAddress>,
    #[doc = "The type of customer address provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_source: Option<stripe_types_01::tax::TaxProductResourceCustomerDetailsAddressSource>,
    #[doc = "The customer's IP address (IPv4 or IPv6)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The customer's tax IDs (for example, EU VAT numbers)."]
    #[json(default)]
    pub tax_ids: Vec<stripe_types_05::tax::TaxProductResourceCustomerDetailsResourceTaxId>,
    #[doc = "The taxability override used for taxation."]
    #[json(default)]
    pub taxability_override: stripe_types_01::tax::TaxProductResourceCustomerDetailsTaxabilityOverride,
}
