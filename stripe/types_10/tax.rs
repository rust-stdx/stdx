#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxCalculationParams {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of an existing customer to use for this calculation."]
    pub customer: Option<String>,
    #[doc = "Details about the customer, including address and tax IDs."]
    pub customer_details: Option<stripe_types_07::tax::CreateTaxCalculationParamsCustomerDetails>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A list of items the customer is purchasing."]
    pub line_items: Vec<stripe_types_05::tax::CreateTaxCalculationParamsLineItemsItem>,
    #[doc = "Details about the address from which the goods are being shipped."]
    pub ship_from_details: Option<stripe_types_07::tax::CreateTaxCalculationParamsShipFromDetails>,
    #[doc = "Shipping cost details to be used for the calculation."]
    pub shipping_cost: Option<stripe_types_07::tax::CreateTaxCalculationParamsShippingCost>,
    #[doc = "The calculation uses the tax rules and rates that are in effect at this timestamp."]
    pub tax_date: Option<i64>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_details", self.customer_details.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        for (index, item) in self.line_items.iter().enumerate() {
            form.field_indexed("line_items", index, item);
        }
        form.field_opt("ship_from_details", self.ship_from_details.as_ref());
        form.field_opt("shipping_cost", self.shipping_cost.as_ref());
        form.field_opt("tax_date", self.tax_date.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAo {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsAoStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsAoType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAw {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsAwStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsAwType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAw {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBb {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsBbStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBbType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBb {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBf {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsBfStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBfType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBf {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBg {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsBgStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBgType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBg {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBs {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsBsStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCh {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsChStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsChType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCh {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCz {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsCzStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsCzType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCz {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsDk {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsDkStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsDkType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsDk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEs {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsEsStandard>,
    #[doc = "Type of registration to be created in ES."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsEsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsFi {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsFiStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsFiType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsFi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsGb {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsGbStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsGbType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGb {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsGr {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsGrStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsGrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsHu {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsHuStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsHuType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsHu {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsIs {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsIsStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsIsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsIs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsIt {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsItStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsItType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsIt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLu {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsLuStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsLuType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLu {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsMeStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsMeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMr {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsMrStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsMrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMt {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsMtStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsMtType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsNz {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsNzStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsNzType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNz {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsPl {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsPlStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsPlType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsPl {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsRo {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsRoStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsRoType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsRo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsRs {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsRsStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsRsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsRs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSi {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsSiStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsSiType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSr {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsSrStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsSrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsZa {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsZaStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsZaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsZa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUnitedStates {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub admissions_tax: Option<stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsUsAdmissionsTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub attendance_tax: Option<stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsUsAttendanceTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entertainment_tax:
        Option<stripe_types_04::tax::TaxProductRegistrationsResourceCountryOptionsUsEntertainmentTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gross_receipts_tax:
        Option<stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsUsGrossReceiptsTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hospitality_tax: Option<stripe_types_04::tax::TaxProductRegistrationsResourceCountryOptionsUsHospitalityTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub local_amusement_tax:
        Option<stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsUsLocalAmusementTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub local_lease_tax: Option<stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsUsLocalLeaseTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub luxury_tax: Option<stripe_types_01::tax::TaxProductRegistrationsResourceCountryOptionsUsLuxuryTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mass_transit_parking_tax:
        Option<stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsUsMassTransitParkingTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parking_tax: Option<stripe_types_03::tax::TaxProductRegistrationsResourceCountryOptionsUsParkingTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub resort_tax: Option<stripe_types_02::tax::TaxProductRegistrationsResourceCountryOptionsUsResortTax>,
    #[doc = "Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    #[json(default)]
    pub state: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state_sales_tax: Option<stripe_types_09::tax::TaxProductRegistrationsResourceCountryOptionsUsStateSalesTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tourism_tax: Option<stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsUsTourismTax>,
    #[doc = "Type of registration in the US."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::tax::TaxProductRegistrationsResourceCountryOptionsUnitedStatesType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxCalculationShippingCost {
    #[doc = "The shipping amount in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The amount of tax calculated for shipping, in the smallest currency unit..."]
    #[json(default)]
    pub amount_tax: i64,
    #[doc = "The ID of an existing [ShippingRate](https://docs.stripe.com/api/shipping_rates/object)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
    #[doc = "Specifies whether the `amount` includes taxes."]
    #[json(default)]
    pub tax_behavior: stripe_types_01::tax::TaxCalculationLineItemTaxBehavior,
    #[doc = "Detailed account of taxes relevant to shipping cost."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_breakdown: Option<Vec<stripe_types_09::tax::TaxProductResourceLineItemTaxBreakdown>>,
    #[doc = "The [tax code](https://docs.stripe.com/tax/tax-categories) ID used for shipping."]
    #[json(default)]
    pub tax_code: String,
}
