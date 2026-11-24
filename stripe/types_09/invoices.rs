#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options: Option<
        stripe_types_05::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions,
    >,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<
        stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_05::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsRendering {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display: Option<stripe_types_04::invoices::CreateInvoiceParamsRenderingAmountTaxDisplay>,
    #[doc = "Invoice pdf rendering options"]
    pub pdf: Option<stripe_types_05::invoices::CreateInvoiceParamsRenderingPdf>,
    #[doc = "ID of the invoice rendering template to use for this invoice."]
    pub template: Option<String>,
    #[doc = "The specific version of invoice rendering template to use for this invoice."]
    pub template_version: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsRendering {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("pdf", self.pdf.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_06::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsShippingDetails {
    #[doc = "Shipping address"]
    pub address: stripe_types_06::invoices::CreateInvoiceParamsShippingDetailsAddress,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)"]
    pub phone: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsCustomerDetailsShipping {
    #[doc = "Customer shipping address."]
    pub address: stripe_types_06::invoices::CreatePreviewParamsCustomerDetailsShippingAddress,
    #[doc = "Customer name."]
    pub name: String,
    #[doc = "Customer phone (including extension)."]
    pub phone: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsCustomerDetailsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::invoices::CreatePreviewParamsIssuerType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::invoices::CreatePreviewParamsScheduleDetailsPhasesItemInvoiceSettingsIssuerType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsSubscriptionDetailsBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    pub proration_discounts:
        Option<stripe_types_05::invoices::CreatePreviewParamsSubscriptionDetailsBillingModeFlexibleProrationDiscounts>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsBillingModeFlexible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("proration_discounts", self.proration_discounts.as_ref());
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicesResourceInvoiceRendering {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<String>,
    #[doc = "Invoice pdf rendering options"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pdf: Option<stripe_types_06::invoices::InvoiceRenderingPdf>,
    #[doc = "ID of the rendering template that the invoice is formatted by."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "Version of the rendering template that the invoice is using."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub template_version: Option<i64>,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicesResourceStatusDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub uncollectible: Option<stripe_types_06::invoices::InvoicesResourceUncollectibleStatusDetails>,
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoicesParams {
    pub collection_method: Option<stripe_types_04::invoices::ListInvoicesParamsCollectionMethod>,
    pub created: Option<stripe_types_00::invoices::ListInvoicesParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub due_date: Option<stripe_types_06::invoices::ListInvoicesParamsDueDate>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_01::invoices::ListInvoicesParamsStatus>,
    pub subscription: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for ListInvoicesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("due_date", self.due_date.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("subscription", self.subscription.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub count: Option<i64>,
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub interval: Option<
        stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
    >,
    #[doc = "Type of installment plan, one of `fixed_count`, `bonus`, or `revolving`."]
    pub r#type:
        stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("count", self.count.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    #[doc = "Configuration for eu_bank_transfer funding type."]
    pub eu_bank_transfer: Option<stripe_types_06::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    #[doc = "The bank transfer type that can be used for funding."]
    pub r#type: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("eu_bank_transfer", self.eu_bank_transfer.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof:
        Option<stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<
        stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType,
    >,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_06::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_05::invoices::UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct UpdateInvoiceParamsShippingDetails {
    #[doc = "Shipping address"]
    pub address: stripe_types_05::invoices::UpdateInvoiceParamsShippingDetailsAddress,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)"]
    pub phone: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateLinesParamsLinesItemPriceDataProductData {
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
    pub tax_details: Option<stripe_types_05::invoices::UpdateLinesParamsLinesItemPriceDataProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateLinesParamsLinesItemPriceDataProductData {
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
