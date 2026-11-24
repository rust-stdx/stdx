#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateReversalParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A flat amount to reverse across the entire transaction, in the smallest currency unit..."]
    pub flat_amount: Option<i64>,
    #[doc = "The line item amounts to reverse."]
    pub line_items: Option<Vec<stripe_types_03::tax::CreateReversalParamsLineItemsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If `partial`, the provided line item or shipping cost amounts are reversed."]
    pub mode: stripe_types_03::tax::CreateReversalParamsMode,
    #[doc = "The ID of the Transaction to partially or fully reverse."]
    pub original_transaction: String,
    #[doc = "A custom identifier for this reversal, such as `myOrder_123-refund_1`, which must be unique across all transactions."]
    pub reference: String,
    #[doc = "The shipping cost to reverse."]
    pub shipping_cost: Option<stripe_types_03::tax::CreateReversalParamsShippingCost>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateReversalParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("flat_amount", self.flat_amount.as_ref());
        if let Some(values) = self.line_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("line_items", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("mode", &self.mode);
        form.field("original_transaction", &self.original_transaction);
        form.field("reference", &self.reference);
        form.field_opt("shipping_cost", self.shipping_cost.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxCalculationParamsLineItemsItem {
    #[doc = "A positive integer representing the line item's total price in the smallest currency unit..."]
    pub amount: i64,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "If provided, the product's `tax_code` will be used as the line item's `tax_code`."]
    pub product: Option<String>,
    #[doc = "The number of units of the item being purchased."]
    pub quantity: Option<i64>,
    #[doc = "A custom identifier for this line item, which must be unique across the line items in the calculation."]
    pub reference: Option<String>,
    #[doc = "Specifies whether the `amount` includes taxes."]
    pub tax_behavior: Option<stripe_types_02::tax::CreateTaxCalculationParamsLineItemsItemTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID to use for this line item."]
    pub tax_code: Option<String>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("performance_location", self.performance_location.as_ref());
        form.field_opt("product", self.product.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxLocationParamsType = stripe_types_04::tax::ListTaxLocationsParamsType;

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsAeStandard {
    #[doc = "Place of supply scheme used in an standard registration."]
    pub place_of_supply_scheme:
        Option<stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAeStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("place_of_supply_scheme", self.place_of_supply_scheme.as_ref());
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAlStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAlType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsAtStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    pub place_of_supply_scheme:
        stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAtStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("place_of_supply_scheme", &self.place_of_supply_scheme);
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAuStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAuType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsAzType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBaStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBbType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBdStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBeType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBfStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBgType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBhStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsBjType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsByType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCdStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsChType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCmType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCrType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCyStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsCyType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsDeStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsDeType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEcType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEeStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEgType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEtStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsEtType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsFrStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsFrType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGeType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGnStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsGrType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsHrStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsHuType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsIeStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsIeType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsIsType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsItStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsJpType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsKgType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsKrType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLaType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLtStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLtType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLvStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsLvType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMdType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMkStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMkType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMtStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMtType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsMyType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNlType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNoStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsNpType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsOmStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsOmType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsPhType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsPtStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsPtType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsRsStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsRsType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSaType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSgStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSgType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSkStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAtStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSkType =
    stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsAtType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsSrType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsTjType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsTwType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUaType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsAttendanceTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsGrossReceiptsTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsLuxuryTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
#[derive(Debug, Clone)]
pub struct CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItem {
    #[doc = "A [FIPS code](https://www.census.gov/library/reference/code-lists/ansi.html) representing the local jurisdiction."]
    pub jurisdiction: Option<String>,
    #[doc = "The type of the election for the state sales tax registration."]
    pub r#type: stripe_types_01::tax::CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItemType,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUsStateSalesTaxElectionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("jurisdiction", self.jurisdiction.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUsTourismTax =
    stripe_types_04::tax::CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUyStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsUyType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAeType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsVnType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsZmType =
    stripe_types_03::tax::CreateTaxRegistrationParamsCountryOptionsAmType;

#[cfg(feature = "tax")]
pub type CreateTaxRegistrationParamsCountryOptionsZwStandardPlaceOfSupplyScheme =
    stripe_types_00::tax::CreateTaxRegistrationParamsCountryOptionsAeStandardPlaceOfSupplyScheme;

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxSettingParamsDefaults {
    #[doc = "Specifies the default tax behavior..."]
    pub tax_behavior: Option<stripe_types_01::tax::CreateTaxSettingParamsDefaultsTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxSettingParamsDefaults {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
    }
}

#[cfg(feature = "tax")]
pub type GetTaxRegistrationParams = stripe_types_01::tax::GetTaxCalculationParams;

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct ListTaxRegistrationsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::tax::ListTaxRegistrationsParamsStatus>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for ListTaxRegistrationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsEuStandard {
    #[doc = "Place of supply scheme used in an EU standard registration."]
    #[json(default)]
    pub place_of_supply_scheme:
        stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsEuStandardPlaceOfSupplyScheme,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsSimplified {
    #[doc = "Type of registration in `country`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::tax::TaxProductRegistrationsResourceCountryOptionsSimplifiedType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceCustomerDetailsResourceTaxId {
    #[doc = "The type of the tax ID, one of `ad_nrt`, `ar_cuit`, `eu_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `cn_tin`, `co_nit`, `cr_..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::tax::TaxProductResourceCustomerDetailsResourceTaxIdType,
    #[doc = "The value of the tax ID."]
    #[json(default)]
    pub value: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceJurisdiction {
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default)]
    pub country: String,
    #[doc = "A human-readable name for the jurisdiction imposing the tax."]
    #[json(default)]
    pub display_name: String,
    #[doc = "Indicates the level of the jurisdiction imposing the tax."]
    #[json(default)]
    pub level: stripe_types_00::tax::TaxProductResourceJurisdictionLevel,
    #[doc = "[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceLineItemTaxRateDetails {
    #[doc = "A localized display name for tax type, intended to be human-readable."]
    #[json(default)]
    pub display_name: String,
    #[doc = "The tax rate percentage as a string."]
    #[json(default)]
    pub percentage_decimal: String,
    #[doc = "The tax type, such as `vat` or `sales_tax`."]
    #[json(default)]
    pub tax_type: stripe_types_01::tax::TaxProductResourceLineItemTaxRateDetailsTaxType,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxAssociationTransactionAttemptsResourceErrored {
    #[doc = "Details on why we couldn't commit the tax transaction."]
    #[json(default)]
    pub reason: stripe_types_03::tax::TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason,
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxSettingsDefaults {
    #[doc = "The tax calculation provider this account uses."]
    #[json(default)]
    pub provider: stripe_types_04::tax::TaxProductResourceTaxSettingsDefaultsProvider,
    #[doc = "Default [tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#tax-behavior) used to ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_types_01::tax::TaxProductResourceTaxSettingsDefaultsTaxBehavior>,
    #[doc = "Default [tax code](https://stripe.com/docs/tax/tax-categories) used to classify your products and prices."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxSettingsHeadOffice {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxSettingsStatusDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<stripe_types_00::tax::TaxProductResourceTaxSettingsStatusDetailsResourceActive>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<stripe_types_01::tax::TaxProductResourceTaxSettingsStatusDetailsResourcePending>,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxTransactionLineItem {
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
    pub object: stripe_types_00::tax::TaxTransactionLineItemObject,
    #[doc = "The ID of an existing [Product](https://docs.stripe.com/api/products/object)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The number of units of the item being purchased."]
    #[json(default)]
    pub quantity: i64,
    #[doc = "A custom identifier for this line item in the transaction."]
    #[json(default)]
    pub reference: String,
    #[doc = "If `type=reversal`, contains information about what was reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reversal: Option<stripe_types_01::tax::TaxProductResourceTaxTransactionLineItemResourceReversal>,
    #[doc = "Specifies whether the `amount` includes taxes."]
    #[json(default)]
    pub tax_behavior: stripe_types_01::tax::TaxCalculationLineItemTaxBehavior,
    #[doc = "The [tax code](https://docs.stripe.com/tax/tax-categories) ID used for this resource."]
    #[json(default)]
    pub tax_code: String,
    #[doc = "If `reversal`, this line item reverses an earlier transaction."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::tax::TaxTransactionType,
}

#[cfg(feature = "tax")]
impl crate::PagedItem for TaxTransactionLineItem {
    fn id(&self) -> &str {
        &self.id
    }
}
