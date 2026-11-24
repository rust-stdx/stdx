#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AddLinesParamsLinesItem {
    #[doc = "The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice."]
    pub amount: Option<i64>,
    #[doc = "An arbitrary string which you can attach to the invoice item."]
    pub description: Option<String>,
    #[doc = "Controls whether discounts apply to this line item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons, promotion codes & existing discounts which apply to the line item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "ID of an unassigned invoice item to assign to this invoice."]
    pub invoice_item: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_06::invoices::AddLinesParamsLinesItemPeriod>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_08::invoices::AddLinesParamsLinesItemPriceData>,
    #[doc = "The pricing information for the invoice item."]
    pub pricing: Option<stripe_types_02::invoices::AddLinesParamsLinesItemPricing>,
    #[doc = "Non-negative integer."]
    pub quantity: Option<i64>,
    #[doc = "Non-negative decimal with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "A list of up to 20 tax amounts for this line item."]
    pub tax_amounts: Option<json::OwnedValue>,
    #[doc = "The tax rates which apply to the line item."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AddLinesParamsLinesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("discountable", self.discountable.as_ref());
        form.field_opt("invoice_item", self.invoice_item.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("period", self.period.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("pricing", self.pricing.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("quantity_decimal", self.quantity_decimal.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_08::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimate {
    #[doc = "The upper bound of the estimated range."]
    pub maximum:
        Option<stripe_types_05::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximum>,
    #[doc = "The lower bound of the estimated range."]
    pub minimum:
        Option<stripe_types_09::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMinimum>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("maximum", self.maximum.as_ref());
        form.field_opt("minimum", self.minimum.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsCustomerDetails {
    #[doc = "The customer's address."]
    pub address: Option<stripe_types_05::invoices::CreatePreviewParamsCustomerDetailsAddress>,
    #[doc = "The customer's shipping information."]
    pub shipping: Option<stripe_types_09::invoices::CreatePreviewParamsCustomerDetailsShipping>,
    #[doc = "Tax details about the customer."]
    pub tax: Option<stripe_types_00::invoices::CreatePreviewParamsCustomerDetailsTax>,
    #[doc = "The customer's tax exemption."]
    pub tax_exempt: Option<stripe_types_04::invoices::CreatePreviewParamsCustomerDetailsTaxExempt>,
    #[doc = "The customer's tax IDs."]
    pub tax_ids: Option<Vec<stripe_types_05::invoices::CreatePreviewParamsCustomerDetailsTaxIdsItem>>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsCustomerDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("tax", self.tax.as_ref());
        form.field_opt("tax_exempt", self.tax_exempt.as_ref());
        if let Some(values) = self.tax_ids.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tax_ids", index, item);
            }
        }
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsInvoiceItemsItem {
    #[doc = "The integer amount in cents (or local equivalent) of previewed invoice item."]
    pub amount: Option<i64>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "An arbitrary string which you can attach to the invoice item."]
    pub description: Option<String>,
    #[doc = "Explicitly controls whether discounts apply to this invoice item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons to redeem into discounts for the invoice item in the preview."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "The ID of the invoice item to update in preview."]
    pub invoiceitem: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_02::invoices::CreatePreviewParamsInvoiceItemsItemPeriod>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_07::invoices::CreatePreviewParamsInvoiceItemsItemPriceData>,
    #[doc = "Non-negative integer."]
    pub quantity: Option<i64>,
    #[doc = "Non-negative decimal with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::invoices::CreatePreviewParamsInvoiceItemsItemTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
    #[doc = "The tax rates that apply to the item."]
    pub tax_rates: Option<json::OwnedValue>,
    #[doc = "The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsInvoiceItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("discountable", self.discountable.as_ref());
        form.field_opt("invoiceitem", self.invoiceitem.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("period", self.period.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("quantity_decimal", self.quantity_decimal.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsSubscriptionDetailsBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_09::invoices::CreatePreviewParamsSubscriptionDetailsBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_05::invoices::CreatePreviewParamsSubscriptionDetailsBillingModeType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsSubscriptionDetailsPause {
    #[doc = "Controls what to bill for when pausing the subscription."]
    pub bill_for: Option<stripe_types_07::invoices::CreatePreviewParamsSubscriptionDetailsPauseBillFor>,
    #[doc = "Determines how to handle debits and credits when pausing."]
    pub invoicing_behavior:
        Option<stripe_types_02::invoices::CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior>,
    #[doc = "The type of pause to apply."]
    pub r#type: Option<stripe_types_04::invoices::CreatePreviewParamsSubscriptionDetailsPauseType>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPause {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bill_for", self.bill_for.as_ref());
        form.field_opt("invoicing_behavior", self.invoicing_behavior.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceLineParamsPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: Option<String>,
    #[doc = "Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline."]
    pub product_data: Option<stripe_types_08::invoices::UpdateInvoiceLineParamsPriceDataProductData>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_06::invoices::UpdateInvoiceLineParamsPriceDataTaxBehavior>,
    #[doc = "A non-negative integer in cents (or local equivalent) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceLineParamsPriceData {
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
pub struct UpdateInvoiceParamsAutomaticTax {
    #[doc = "Whether Stripe automatically computes tax on this invoice."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_08::invoices::UpdateInvoiceParamsAutomaticTaxLiability>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillie {
    #[doc = "Registration details about the buyer's organization."]
    pub company_details:
        Option<stripe_types_08::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails>,
    #[doc = "An identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("company_details", self.company_details.as_ref());
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments {
    #[doc = "Setting to true enables installments for this invoice."]
    pub enabled: Option<bool>,
    #[doc = "The selected installment plan to use for this invoice."]
    pub plan:
        Option<stripe_types_09::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("plan", self.plan.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer: Option<
        stripe_types_09::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer,
    >,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_09::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsRendering {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display: Option<stripe_types_05::invoices::UpdateInvoiceParamsRenderingAmountTaxDisplay>,
    #[doc = "Invoice pdf rendering options"]
    pub pdf: Option<stripe_types_07::invoices::UpdateInvoiceParamsRenderingPdf>,
    #[doc = "ID of the invoice rendering template to use for this invoice."]
    pub template: Option<String>,
    #[doc = "The specific version of invoice rendering template to use for this invoice."]
    pub template_version: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsRendering {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("pdf", self.pdf.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsShippingCostShippingRateDataFixedAmount {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Shipping rates defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_07::invoices::UpdateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValue,
        >,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingCostShippingRateDataFixedAmount {
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
