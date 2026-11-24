#[cfg(feature = "invoices")]
pub type AddLinesParamsLinesItemPeriod = stripe_types_02::invoices::CreatePreviewParamsInvoiceItemsItemPeriod;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AddLinesParamsLinesItemPriceDataProductData {
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
    pub tax_details: Option<stripe_types_02::invoices::AddLinesParamsLinesItemPriceDataProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AddLinesParamsLinesItemPriceDataProductData {
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
pub type CreateInvoiceParamsCollectionMethod = stripe_types_04::invoices::ListInvoicesParamsCollectionMethod;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsFromInvoice {
    #[doc = "The relation between the new invoice and the original invoice."]
    pub action: stripe_types_04::invoices::CreateInvoiceParamsFromInvoiceAction,
    #[doc = "The `id` of the invoice that will be cloned."]
    pub invoice: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsFromInvoice {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("action", &self.action);
        form.field("invoice", &self.invoice);
    }
}

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsIssuerType = stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact {
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    pub preferred_language: Option<
        stripe_types_04::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub count: Option<i64>,
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub interval: Option<
        stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,
    >,
    #[doc = "Type of installment plan, one of `fixed_count`, `bonus`, or `revolving`."]
    pub r#type:
        stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("count", self.count.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsKonbini =
    stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof:
        Option<stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
    }
}

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsSepaDebit =
    stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_02::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            form.field_array("account_subcategories", values.as_slice());
        }
    }
}

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit =
    stripe_types_02::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValue {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
    }
}

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsShippingCostShippingRateDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type CreateInvoiceParamsShippingDetailsAddress = stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsCustomerDetailsShippingAddress = stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsInvoiceItemsItemPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    pub proration_discounts:
        Option<stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsBillingModeFlexible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("proration_discounts", self.proration_discounts.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStart {
    #[doc = "A precise Unix timestamp for the start of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the start of the invoice item period."]
    pub r#type:
        stripe_types_00::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStart {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsPhasesItemAutomaticTaxLiabilityType =
    stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemDuration {
    #[doc = "Specifies phase duration."]
    pub interval: stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval,
    #[doc = "The multiplier applied to the interval."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemDuration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsPhasesItemItemsItemPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsProrationBehavior =
    stripe_types_04::invoices::CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsSubscriptionDetailsItemsItemBillingThresholds =
    stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsPhasesItemItemsItemBillingThresholds;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsSubscriptionDetailsItemsItemPriceDataRecurringInterval =
    stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsSubscriptionDetailsItemsItemPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFrom {
    #[doc = "When to credit for unused time."]
    pub r#type: stripe_types_01::invoices::CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFrom {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
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
pub struct InvoiceMandateOptionsCard {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_types_00::invoices::InvoiceMandateOptionsCardAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
pub struct InvoiceMandateOptionsPayto {
    #[doc = "The maximum amount that can be collected in a single invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "Only `maximum` is supported."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type: Option<stripe_types_00::invoices::InvoiceMandateOptionsCardAmountType>,
    #[doc = "The purpose for which payments are made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_types_01::invoices::InvoiceMandateOptionsPaytoPurpose>,
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
pub struct InvoiceRenderingPdf {
    #[doc = "Page size of invoice pdf."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<stripe_types_04::invoices::InvoiceRenderingPdfPageSize>,
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
pub struct InvoicesResourceInvoiceTaxId {
    #[doc = "The type of the tax ID, one of `ad_nrt`, `ar_cuit`, `eu_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `cn_tin`, `co_nit`, `cr_..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::invoices::InvoicesResourceInvoiceTaxIdType,
    #[doc = "The value of the tax ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
pub struct InvoicesResourceUncollectibleStatusDetails {
    #[doc = "The reason why the invoice is uncollectible."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_00::invoices::InvoicesResourceUncollectibleStatusDetailsReason>,
}

#[cfg(feature = "invoices")]
pub type ListInvoicesParamsDueDate = stripe_types_00::invoices::ListInvoicesParamsCreated;

#[cfg(feature = "invoices")]
pub type MarkUncollectibleParams = stripe_types_02::invoices::GetInvoiceParams;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct RemoveLinesParamsLinesItem {
    #[doc = "Either `delete` or `unassign`."]
    pub behavior: stripe_types_02::invoices::RemoveLinesParamsLinesItemBehavior,
    #[doc = "ID of an existing line item to remove from this invoice."]
    pub id: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for RemoveLinesParamsLinesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("behavior", &self.behavior);
        form.field("id", &self.id);
    }
}

#[cfg(feature = "invoices")]
pub type SendInvoiceParams = stripe_types_02::invoices::GetInvoiceParams;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceLineParamsPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceLineParamsPricing = stripe_types_02::invoices::AddLinesParamsLinesItemPricing;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsAutomaticTaxLiabilityType =
    stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsCollectionMethod = stripe_types_04::invoices::ListInvoicesParamsCollectionMethod;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsIssuerType = stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage =
    stripe_types_04::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik =
    stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer = stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem = stripe_types_02::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem = stripe_types_02::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem = stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsRenderingPdfPageSize = stripe_types_01::invoices::CreateInvoiceParamsRenderingPdfPageSize;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit =
    stripe_types_02::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsShippingCostShippingRateDataType =
    stripe_types_00::invoices::CreateInvoiceParamsShippingCostShippingRateDataType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsTransferData = stripe_types_02::invoices::CreateInvoiceParamsTransferData;

#[cfg(feature = "invoices")]
pub type UpdateLinesParamsLinesItemPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type UpdateLinesParamsLinesItemPricing = stripe_types_02::invoices::AddLinesParamsLinesItemPricing;
