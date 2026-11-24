#[cfg(feature = "invoices")]
pub type AddLinesParamsLinesItemPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails {
    #[doc = "The address the company or entity is registered with."]
    pub registered_address: Option<stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    pub registration_type: Option<stripe_types_04::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT ID number."]
    pub vat: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails {
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
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    #[doc = "Configuration for eu_bank_transfer funding type."]
    pub eu_bank_transfer: Option<stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    #[doc = "The bank transfer type that can be used for funding."]
    pub r#type: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("eu_bank_transfer", self.eu_bank_transfer.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "The maximum amount that can be collected in a single invoice."]
    pub amount: Option<i64>,
    #[doc = "The purpose for which payments are made."]
    pub purpose: Option<
        stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<
        stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType,
    >,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsRenderingPdf {
    #[doc = "Page size for invoice PDF."]
    pub page_size: Option<stripe_types_01::invoices::CreateInvoiceParamsRenderingPdfPageSize>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsRenderingPdf {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("page_size", self.page_size.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_02::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsAutomaticTaxLiabilityType =
    stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsCustomerDetailsAddress = stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsCustomerDetailsTaxIdsItem {
    #[doc = "Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`,..."]
    pub r#type: stripe_types_01::invoices::CreatePreviewParamsCustomerDetailsTaxIdsItemType,
    #[doc = "Value of the tax ID."]
    pub value: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsCustomerDetailsTaxIdsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsInvoiceItemsItemTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsIssuerType = stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEnd {
    #[doc = "A precise Unix timestamp for the end of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the end of the invoice item period."]
    pub r#type: stripe_types_03::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEnd {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsPhasesItemCollectionMethod =
    stripe_types_04::invoices::ListInvoicesParamsCollectionMethod;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsPhasesItemInvoiceSettingsIssuerType =
    stripe_types_01::invoices::CreateInvoiceParamsAutomaticTaxLiabilityType;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsScheduleDetailsPhasesItemItemsItemPriceDataRecurringInterval =
    stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsSubscriptionDetailsBillingCycleAnchor {
    #[doc = "A timestamp to use as the subscription's billing cycle anchor."]
    pub timestamp: Option<i64>,
    #[doc = "Determines how the subscription's billing cycle anchor behaves for the invoice preview."]
    pub r#type: stripe_types_01::invoices::CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsBillingCycleAnchor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsSubscriptionDetailsBillingModeFlexibleProrationDiscounts =
    stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts;

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsSubscriptionDetailsBillingModeType =
    stripe_types_01::invoices::CreatePreviewParamsScheduleDetailsBillingModeType;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThrough {
    #[doc = "When to bill metered usage in the current period."]
    pub r#type:
        stripe_types_01::invoices::CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThrough {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
pub type CreatePreviewParamsSubscriptionDetailsProrationBehavior =
    stripe_types_04::invoices::CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior;

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedInvoice {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::invoices::InvoiceObject,
}

#[cfg(feature = "invoices")]
impl crate::PagedItem for DeletedInvoice {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceItemProrationCreditedItems {
    #[doc = "When `type` is `invoice_item`, the invoice item id for the debited invoice item corresponding to this credit proration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_line_item_details: Option<stripe_types_04::invoices::CreditedItemsInvoiceLineItems>,
    #[doc = "Whether the credit references a pending invoice item or one or more invoice line items on an invoice."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::invoices::InvoiceItemProrationCreditedItemsType,
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
pub struct InvoiceThresholdReason {
    #[doc = "The total invoice amount threshold boundary if it triggered the threshold invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    #[doc = "Indicates which line items triggered a threshold invoice."]
    #[json(default)]
    pub item_reasons: Vec<stripe_types_00::invoices::InvoiceItemThresholdReason>,
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicingRule {
    #[doc = "The type of invoicing rule."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::invoices::InvoicingRuleType,
}

#[cfg(feature = "invoices")]
pub type UpdateInvoiceLineParamsPeriod = stripe_types_02::invoices::CreatePreviewParamsInvoiceItemsItemPeriod;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceLineParamsPriceDataProductDataTaxDetails =
    stripe_types_02::invoices::AddLinesParamsLinesItemPriceDataProductDataTaxDetails;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType = stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress = stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType = stripe_types_04::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval =
    stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType =
    stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsKonbini =
    stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose =
    stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof =
    stripe_types_03::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsSepaDebit =
    stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_types_00::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsRenderingAmountTaxDisplay =
    stripe_types_04::invoices::CreateInvoiceParamsRenderingAmountTaxDisplay;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit =
    stripe_types_02::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsShippingCostShippingRateDataTaxBehavior = stripe_types_03::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior;

#[cfg(feature = "invoices")]
pub type UpdateInvoiceParamsShippingDetailsAddress = stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress;

#[cfg(feature = "invoices")]
pub type UpdateLinesParamsLinesItemPeriod = stripe_types_02::invoices::CreatePreviewParamsInvoiceItemsItemPeriod;

#[cfg(feature = "invoices")]
pub type UpdateLinesParamsLinesItemPriceDataProductDataTaxDetails =
    stripe_types_02::invoices::AddLinesParamsLinesItemPriceDataProductDataTaxDetails;

#[cfg(feature = "invoices")]
pub type VoidInvoiceParams = stripe_types_02::invoices::GetInvoiceParams;
