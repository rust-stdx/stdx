#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsItemsItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<stripe_types_01::subscriptions::CreateSubscriptionParamsItemsItemBillingThresholds>,
    #[doc = "The trial offer to apply to this subscription item."]
    pub current_trial: Option<stripe_types_02::subscriptions::CreateSubscriptionParamsItemsItemCurrentTrial>,
    #[doc = "The coupons to redeem into discounts for the subscription item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Plan ID for this item, as a string."]
    pub plan: Option<String>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_11::subscriptions::CreateSubscriptionParamsItemsItemPriceData>,
    #[doc = "Quantity for this item."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("current_trial", self.current_trial.as_ref());
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

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptions {
    #[doc = "This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’..."]
    pub acss_debit:
        Option<stripe_types_07::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,
    #[doc = "This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent."]
    pub bancontact:
        Option<stripe_types_05::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontact>,
    #[doc = "This sub-hash contains details about the Billie payment method options to pass to the invoice’s PaymentIntent."]
    pub billie:
        Option<stripe_types_09::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillie>,
    #[doc = "This sub-hash contains details about the Blik payment method options to pass to the invoice’s PaymentIntent."]
    pub blik: Option<stripe_types_05::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlik>,
    #[doc = "This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent."]
    pub card: Option<stripe_types_09::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCard>,
    #[doc = "This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent."]
    pub customer_balance: Option<
        stripe_types_09::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalance,
    >,
    #[doc = "This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent."]
    pub konbini:
        Option<stripe_types_01::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini>,
    #[doc = "This sub-hash contains details about the PayTo payment method options to pass to the invoice’s PaymentIntent."]
    pub payto: Option<stripe_types_07::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPayto>,
    #[doc = "This sub-hash contains details about the Pix payment method options to pass to the invoice’s PaymentIntent."]
    pub pix: Option<stripe_types_09::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPix>,
    #[doc = "This sub-hash contains details about the SEPA Direct Debit payment method options to pass to the invoice’s PaymentInt..."]
    pub sepa_debit:
        Option<stripe_types_06::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsSepaDebit>,
    #[doc = "This sub-hash contains details about the UPI payment method options to pass to the invoice’s PaymentIntent."]
    pub upi: Option<stripe_types_10::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpi>,
    #[doc = "This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent."]
    pub us_bank_account: Option<
        stripe_types_11::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccount,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptions {
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

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct MigrateSubscriptionParams {
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    pub billing_mode: stripe_types_11::subscriptions::MigrateSubscriptionParamsBillingMode,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for MigrateSubscriptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("billing_mode", &self.billing_mode);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
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
pub struct SubscriptionsResourceBillingSchedulesAppliesTo {
    #[doc = "The billing schedule will apply to the subscription item with the given price ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<crate::Expandable<stripe_types_11::prices::Price>>,
    #[doc = "Controls which subscription items the billing schedule applies to."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::subscriptions::SubscriptionsResourceBillingSchedulesAppliesToType,
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
pub struct SubscriptionsResourcePaymentMethodOptions {
    #[doc = "This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices cre..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_08::invoice_payments::InvoicePaymentMethodOptionsAcssDebit>,
    #[doc = "This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscri..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_06::invoice_payments::InvoicePaymentMethodOptionsBancontact>,
    #[doc = "This sub-hash contains details about the Billie payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_09::subscriptions::SubscriptionPaymentMethodOptionsBillie>,
    #[doc = "This sub-hash contains details about the Blik payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_06::subscriptions::SubscriptionPaymentMethodOptionsBlik>,
    #[doc = "This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_07::subscriptions::SubscriptionPaymentMethodOptionsCard>,
    #[doc = "This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subs..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_10::invoice_payments::InvoicePaymentMethodOptionsCustomerBalance>,
    #[doc = "This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_03::invoice_payments::InvoicePaymentMethodOptionsKonbini>,
    #[doc = "This sub-hash contains details about the PayTo payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_09::invoice_payments::InvoicePaymentMethodOptionsPayto>,
    #[doc = "This sub-hash contains details about the Pix payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_08::subscriptions::SubscriptionPaymentMethodOptionsPix>,
    #[doc = "This sub-hash contains details about the SEPA Direct Debit payment method options to pass to invoices created by the ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_02::invoice_payments::InvoicePaymentMethodOptionsSepaDebit>,
    #[doc = "This sub-hash contains details about the UPI payment method options to pass to invoices created by the subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_07::invoice_payments::InvoicePaymentMethodOptionsUpi>,
    #[doc = "This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the s..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_11::invoice_payments::InvoicePaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsAddInvoiceItemsItem {
    #[doc = "Controls whether discounts apply to this invoice item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons to redeem into discounts for the item."]
    pub discounts:
        Option<Vec<stripe_types_05::subscriptions::UpdateSubscriptionParamsAddInvoiceItemsItemDiscountsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_10::subscriptions::UpdateSubscriptionParamsAddInvoiceItemsItemPeriod>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_09::subscriptions::UpdateSubscriptionParamsAddInvoiceItemsItemPriceData>,
    #[doc = "Quantity for this item."]
    pub quantity: Option<i64>,
    #[doc = "The tax rates which apply to the item."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsAddInvoiceItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("discountable", self.discountable.as_ref());
        if let Some(values) = self.discounts.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("discounts", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("period", self.period.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsItemsItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<stripe_types_05::subscriptions::UpdateSubscriptionParamsItemsItemBillingThresholds>,
    #[doc = "Delete all usage for a given subscription item."]
    pub clear_usage: Option<bool>,
    #[doc = "The trial offer to apply to this subscription item."]
    pub current_trial: Option<stripe_types_05::subscriptions::UpdateSubscriptionParamsItemsItemCurrentTrial>,
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
    pub price_data: Option<stripe_types_10::subscriptions::UpdateSubscriptionParamsItemsItemPriceData>,
    #[doc = "Quantity for this item."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsItemsItem {
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

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_11::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<stripe_types_05::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}
