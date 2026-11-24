#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AddLinesParamsLinesItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: Option<String>,
    #[doc = "Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline."]
    pub product_data: Option<stripe_types_06::invoices::AddLinesParamsLinesItemPriceDataProductData>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::invoices::AddLinesParamsLinesItemPriceDataTaxBehavior>,
    #[doc = "A non-negative integer in cents (or local equivalent) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AddLinesParamsLinesItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field_opt("product", self.product.as_ref());
        form.field_opt("product_data", self.product_data.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillie {
    #[doc = "Registration details about the buyer's organization."]
    pub company_details:
        Option<stripe_types_05::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails>,
    #[doc = "An identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("company_details", self.company_details.as_ref());
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_02::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            form.field_array("permissions", values.as_slice());
        }
        if let Some(values) = self.prefetch.as_ref() {
            form.field_array("prefetch", values.as_slice());
        }
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsShippingCostShippingRateDataFixedAmount {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Shipping rates defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_06::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValue,
        >,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataFixedAmount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval:
        stripe_types_05::invoices::CreatePreviewParamsScheduleDetailsPhasesItemItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsSubscriptionDetailsItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_06::invoices::CreatePreviewParamsSubscriptionDetailsItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceLineParamsPriceDataProductData {
    #[doc = "The product's description, meant to be displayable to the customer."]
    pub description: Option<String>,
    #[doc = "A list of up to 8 URLs of images for this product, meant to be displayable to the customer."]
    pub images: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The product's name, meant to be displayable to the customer."]
    pub name: String,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "Tax details for this product, including the [tax code](/tax/tax-codes) and an optional performance location."]
    pub tax_details: Option<stripe_types_05::invoices::UpdateInvoiceLineParamsPriceDataProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceLineParamsPriceDataProductData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.images.as_ref() {
            form.field_array("images", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("tax_details", self.tax_details.as_ref());
        form.field_opt("unit_label", self.unit_label.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct UpdateInvoiceParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::invoices::UpdateInvoiceParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct UpdateInvoiceParamsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::invoices::UpdateInvoiceParamsIssuerType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact {
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    pub preferred_language: Option<
        stripe_types_06::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails {
    #[doc = "The address the company or entity is registered with."]
    pub registered_address: Option<stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    pub registration_type: Option<stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT ID number."]
    pub vat: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("registered_name", self.registered_name.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("registration_type", self.registration_type.as_ref());
        form.field_opt("vat", self.vat.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "The maximum amount that can be collected in a single invoice."]
    pub amount: Option<i64>,
    #[doc = "The purpose for which payments are made."]
    pub purpose: Option<
        stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}
