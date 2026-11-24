#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AddLinesParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub invoice_metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The line items to add."]
    pub lines: Vec<stripe_types_11::invoices::AddLinesParamsLinesItem>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AddLinesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.invoice_metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("invoice_metadata", key, value);
            }
        }
        for (index, item) in self.lines.iter().enumerate() {
            form.field_indexed("lines", index, item);
        }
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptions {
    #[doc = "If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method opti..."]
    pub acss_debit: Option<stripe_types_09::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,
    #[doc = "If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the ..."]
    pub bancontact: Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact>,
    #[doc = "If paying by `billie`, this sub-hash contains details about the Billie payment method options to pass to the invoice’..."]
    pub billie: Option<stripe_types_08::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillie>,
    #[doc = "If paying by `blik`, this sub-hash contains details about the Blik payment method options to pass to the invoice’s Pa..."]
    pub blik: Option<stripe_types_01::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik>,
    #[doc = "If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s Pa..."]
    pub card: Option<stripe_types_10::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCard>,
    #[doc = "If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pas..."]
    pub customer_balance:
        Option<stripe_types_07::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance>,
    #[doc = "If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoic..."]
    pub konbini: Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsKonbini>,
    #[doc = "If paying by `payto`, this sub-hash contains details about the PayTo payment method options to pass to the invoice’s ..."]
    pub payto: Option<stripe_types_07::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPayto>,
    #[doc = "If paying by `pix`, this sub-hash contains details about the Pix payment method options to pass to the invoice’s Paym..."]
    pub pix: Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPix>,
    #[doc = "If paying by `sepa_debit`, this sub-hash contains details about the SEPA Direct Debit payment method options to pass ..."]
    pub sepa_debit: Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsSepaDebit>,
    #[doc = "If paying by `upi`, this sub-hash contains details about the UPI payment method options to pass to the invoice’s Paym..."]
    pub upi: Option<stripe_types_09::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpi>,
    #[doc = "If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to p..."]
    pub us_bank_account:
        Option<stripe_types_11::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsShippingCostShippingRateData {
    #[doc = "The estimated range for how long shipping will take, meant to be displayable to the customer."]
    pub delivery_estimate:
        Option<stripe_types_11::invoices::CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimate>,
    #[doc = "The name of the shipping rate, meant to be displayable to the customer."]
    pub display_name: String,
    #[doc = "Describes a fixed amount to charge for shipping."]
    pub fixed_amount: Option<stripe_types_08::invoices::CreateInvoiceParamsShippingCostShippingRateDataFixedAmount>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_06::invoices::CreateInvoiceParamsShippingCostShippingRateDataTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "The type of calculation to use on the shipping rate."]
    pub r#type: Option<stripe_types_00::invoices::CreateInvoiceParamsShippingCostShippingRateDataType>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("delivery_estimate", self.delivery_estimate.as_ref());
        form.field("display_name", &self.display_name);
        form.field_opt("fixed_amount", self.fixed_amount.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemItemsItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds:
        Option<stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsPhasesItemItemsItemBillingThresholds>,
    #[doc = "The coupons to redeem into discounts for the subscription item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a configuration item."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The plan ID to subscribe to."]
    pub plan: Option<String>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_10::invoices::CreatePreviewParamsScheduleDetailsPhasesItemItemsItemPriceData>,
    #[doc = "Quantity for the given price."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("plan", self.plan.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsSubscriptionDetailsItemsItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds:
        Option<stripe_types_06::invoices::CreatePreviewParamsSubscriptionDetailsItemsItemBillingThresholds>,
    #[doc = "Delete all usage for a given subscription item."]
    pub clear_usage: Option<bool>,
    #[doc = "The trial offer to apply to this subscription item."]
    pub current_trial: Option<stripe_types_03::invoices::CreatePreviewParamsSubscriptionDetailsItemsItemCurrentTrial>,
    #[doc = "A flag that, if set to `true`, will delete the specified item."]
    pub deleted: Option<bool>,
    #[doc = "The coupons to redeem into discounts for the subscription item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Subscription item to update."]
    pub id: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Plan ID for this item, as a string."]
    pub plan: Option<String>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_10::invoices::CreatePreviewParamsSubscriptionDetailsItemsItemPriceData>,
    #[doc = "Quantity for this item."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("clear_usage", self.clear_usage.as_ref());
        form.field_opt("current_trial", self.current_trial.as_ref());
        form.field_opt("deleted", self.deleted.as_ref());
        form.field_opt("id", self.id.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("plan", self.plan.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
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
pub struct InvoicesPaymentMethodOptions {
    #[doc = "If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method opti..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_08::invoice_payments::InvoicePaymentMethodOptionsAcssDebit>,
    #[doc = "If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_06::invoice_payments::InvoicePaymentMethodOptionsBancontact>,
    #[doc = "If paying by `billie`, this sub-hash contains details about the Billie payment method options to pass to the invoice’..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_09::invoice_payments::InvoicePaymentMethodOptionsBillie>,
    #[doc = "If paying by `blik`, this sub-hash contains details about the Blik payment method options to pass to the invoice’s Pa..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_02::invoice_payments::InvoicePaymentMethodOptionsBlik>,
    #[doc = "If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s Pa..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_05::invoice_payments::InvoicePaymentMethodOptionsCard>,
    #[doc = "If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pas..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_10::invoice_payments::InvoicePaymentMethodOptionsCustomerBalance>,
    #[doc = "If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoic..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_03::invoice_payments::InvoicePaymentMethodOptionsKonbini>,
    #[doc = "If paying by `payto`, this sub-hash contains details about the PayTo payment method options to pass to the invoice’s ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_09::invoice_payments::InvoicePaymentMethodOptionsPayto>,
    #[doc = "If paying by `pix`, this sub-hash contains details about the Pix payment method options to pass to the invoice’s Paym..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_06::invoice_payments::InvoicePaymentMethodOptionsPix>,
    #[doc = "If paying by `sepa_debit`, this sub-hash contains details about the SEPA Direct Debit payment method options to pass ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_02::invoice_payments::InvoicePaymentMethodOptionsSepaDebit>,
    #[doc = "If paying by `upi`, this sub-hash contains details about the UPI payment method options to pass to the invoice’s Paym..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_07::invoice_payments::InvoicePaymentMethodOptionsUpi>,
    #[doc = "If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to p..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_11::invoice_payments::InvoicePaymentMethodOptionsUsBankAccount>,
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
pub struct InvoicesResourceShippingCost {
    #[doc = "Total shipping cost before any taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total tax amount applied due to shipping costs."]
    #[json(default)]
    pub amount_tax: i64,
    #[doc = "Total shipping cost after taxes are applied."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "The ID of the ShippingRate for this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<crate::Expandable<stripe_types_11::shipping_rates::ShippingRate>>,
    #[doc = "The taxes applied to the shipping rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_types_07::common::LineItemsTaxAmount>>,
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceLineParams {
    #[doc = "The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice."]
    pub amount: Option<i64>,
    #[doc = "An arbitrary string which you can attach to the invoice item."]
    pub description: Option<String>,
    #[doc = "Controls whether discounts apply to this line item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons, promotion codes & existing discounts which apply to the line item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_05::invoices::UpdateInvoiceLineParamsPeriod>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_11::invoices::UpdateInvoiceLineParamsPriceData>,
    #[doc = "The pricing information for the invoice item."]
    pub pricing: Option<stripe_types_06::invoices::UpdateInvoiceLineParamsPricing>,
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
impl crate::params::FormEncode for UpdateInvoiceLineParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("discountable", self.discountable.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
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
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCard {
    #[doc = "Installment configuration for payments attempted on this invoice."]
    pub installments:
        Option<stripe_types_11::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure: Option<
        stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure,
    >,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("installments", self.installments.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_10::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<stripe_types_06::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsShippingCostShippingRateData {
    #[doc = "The estimated range for how long shipping will take, meant to be displayable to the customer."]
    pub delivery_estimate:
        Option<stripe_types_10::invoices::UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimate>,
    #[doc = "The name of the shipping rate, meant to be displayable to the customer."]
    pub display_name: String,
    #[doc = "Describes a fixed amount to charge for shipping."]
    pub fixed_amount: Option<stripe_types_11::invoices::UpdateInvoiceParamsShippingCostShippingRateDataFixedAmount>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_05::invoices::UpdateInvoiceParamsShippingCostShippingRateDataTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "The type of calculation to use on the shipping rate."]
    pub r#type: Option<stripe_types_06::invoices::UpdateInvoiceParamsShippingCostShippingRateDataType>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingCostShippingRateData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("delivery_estimate", self.delivery_estimate.as_ref());
        form.field("display_name", &self.display_name);
        form.field_opt("fixed_amount", self.fixed_amount.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateLinesParamsLinesItem {
    #[doc = "The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice."]
    pub amount: Option<i64>,
    #[doc = "An arbitrary string which you can attach to the invoice item."]
    pub description: Option<String>,
    #[doc = "Controls whether discounts apply to this line item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons, promotion codes & existing discounts which apply to the line item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "ID of an existing line item on the invoice."]
    pub id: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_05::invoices::UpdateLinesParamsLinesItemPeriod>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_10::invoices::UpdateLinesParamsLinesItemPriceData>,
    #[doc = "The pricing information for the invoice item."]
    pub pricing: Option<stripe_types_06::invoices::UpdateLinesParamsLinesItemPricing>,
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
impl crate::params::FormEncode for UpdateLinesParamsLinesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("discountable", self.discountable.as_ref());
        form.field("id", &self.id);
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
