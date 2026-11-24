#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAl {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsAlStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAlType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAl {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAu {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsAuStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsAuType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAu {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBa {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsBaStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBaType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBa {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBd {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsBdStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBdType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsBeStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsBeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsBh {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsBhStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsBhType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsBh {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCd {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsCdStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsCdType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsCy {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsCyStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsCyType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCy {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsDe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsDeStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsDeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsDe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsEeStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsEeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsEt {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsEtStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsEtType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsEt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsFr {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsFrStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsFrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsFr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsGn {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsGnStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsGnType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsGn {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsHr {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsHrStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsHrType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsHr {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsIe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsIeStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsIeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsIe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsJp {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsJpStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsJpType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsJp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLt {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsLtStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsLtType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsLv {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsLvStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsLvType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsLv {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsMk {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsMkStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsMkType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsMk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsNl {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsNlStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsNlType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNl {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsNo {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsNoStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsNoType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsNo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsOm {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsOmStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsOmType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsOm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsPt {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsPtStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsPtType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsPt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSe {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsSeStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsSeType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSe {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSg {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_09::tax::CreateTaxRegistrationParamsCountryOptionsSgStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsSgType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSg {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsSk {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsSkStandard>,
    #[doc = "Type of registration to be created in an EU country."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsSkType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsSk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsUs {
    #[doc = "Options for the admission tax registration."]
    pub admissions_tax: Option<stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax>,
    #[doc = "Options for the attendance tax registration."]
    pub attendance_tax: Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUsAttendanceTax>,
    #[doc = "Options for the entertainment tax registration."]
    pub entertainment_tax: Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsEntertainmentTax>,
    #[doc = "Options for the gross receipts tax registration."]
    pub gross_receipts_tax: Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUsGrossReceiptsTax>,
    #[doc = "Options for the hospitality tax registration."]
    pub hospitality_tax: Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsHospitalityTax>,
    #[doc = "Options for the local amusement tax registration."]
    pub local_amusement_tax: Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsLocalAmusementTax>,
    #[doc = "Options for the local lease tax registration."]
    pub local_lease_tax: Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsLocalLeaseTax>,
    #[doc = "Options for the luxury tax registration."]
    pub luxury_tax: Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUsLuxuryTax>,
    #[doc = "Options for the mass transit parking tax registration."]
    pub mass_transit_parking_tax:
        Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsMassTransitParkingTax>,
    #[doc = "Options for the parking tax registration."]
    pub parking_tax: Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsParkingTax>,
    #[doc = "Options for the resort tax registration."]
    pub resort_tax: Option<stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsUsResortTax>,
    #[doc = "Two-letter US state code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: String,
    #[doc = "Options for the state sales tax registration."]
    pub state_sales_tax: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsUsStateSalesTax>,
    #[doc = "Options for the tourism tax registration."]
    pub tourism_tax: Option<stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUsTourismTax>,
    #[doc = "Type of registration to be created in the US."]
    pub r#type: stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsUsType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("admissions_tax", self.admissions_tax.as_ref());
        form.field_opt("attendance_tax", self.attendance_tax.as_ref());
        form.field_opt("entertainment_tax", self.entertainment_tax.as_ref());
        form.field_opt("gross_receipts_tax", self.gross_receipts_tax.as_ref());
        form.field_opt("hospitality_tax", self.hospitality_tax.as_ref());
        form.field_opt("local_amusement_tax", self.local_amusement_tax.as_ref());
        form.field_opt("local_lease_tax", self.local_lease_tax.as_ref());
        form.field_opt("luxury_tax", self.luxury_tax.as_ref());
        form.field_opt("mass_transit_parking_tax", self.mass_transit_parking_tax.as_ref());
        form.field_opt("parking_tax", self.parking_tax.as_ref());
        form.field_opt("resort_tax", self.resort_tax.as_ref());
        form.field("state", &self.state);
        form.field_opt("state_sales_tax", self.state_sales_tax.as_ref());
        form.field_opt("tourism_tax", self.tourism_tax.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsUy {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_08::tax::CreateTaxRegistrationParamsCountryOptionsUyStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_05::tax::CreateTaxRegistrationParamsCountryOptionsUyType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUy {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsZw {
    #[doc = "Options for the standard registration."]
    pub standard: Option<stripe_types_07::tax::CreateTaxRegistrationParamsCountryOptionsZwStandard>,
    #[doc = "Type of registration to be created in `country`."]
    pub r#type: stripe_types_06::tax::CreateTaxRegistrationParamsCountryOptionsZwType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsZw {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("standard", self.standard.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxAssociation {
    #[doc = "The [Tax Calculation](https://docs.stripe.com/api/tax/calculations/object) that was included in PaymentIntent."]
    #[json(default)]
    pub calculation: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::tax::TaxAssociationObject,
    #[doc = "The [PaymentIntent](https://docs.stripe.com/api/payment_intents/object) that this Tax Association is tracking."]
    #[json(default)]
    pub payment_intent: String,
    #[doc = "Information about the tax transactions linked to this payment intent"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_transaction_attempts:
        Option<Vec<stripe_types_08::tax::TaxProductResourceTaxAssociationTransactionAttempts>>,
}

#[cfg(feature = "tax")]
impl crate::PagedItem for TaxAssociation {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxCalculationLineItem {
    #[doc = "The line item amount in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The amount of tax calculated for this line item, in the smallest currency unit..."]
    #[json(default)]
    pub amount_tax: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::tax::TaxCalculationLineItemObject,
    #[doc = "Indicates the line item represents a performance where the venue location might determine the tax, not the customer a..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub performance_location: Option<String>,
    #[doc = "The ID of an existing [Product](https://docs.stripe.com/api/products/object)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The number of units of the item being purchased."]
    #[json(default)]
    pub quantity: i64,
    #[doc = "A custom identifier for this line item."]
    #[json(default)]
    pub reference: String,
    #[doc = "Specifies whether the `amount` includes taxes."]
    #[json(default)]
    pub tax_behavior: stripe_types_01::tax::TaxCalculationLineItemTaxBehavior,
    #[doc = "Detailed account of taxes relevant to this line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_breakdown: Option<Vec<stripe_types_09::tax::TaxProductResourceLineItemTaxBreakdown>>,
    #[doc = "The [tax code](https://docs.stripe.com/tax/tax-categories) ID used for this resource."]
    #[json(default)]
    pub tax_code: String,
}

#[cfg(feature = "tax")]
impl crate::PagedItem for TaxCalculationLineItem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxTransactionShippingCost {
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
