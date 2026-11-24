#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParams {
    #[doc = "The customer's address."]
    pub address: Option<stripe_types_00::customers::CreateCustomerParamsAddress>,
    #[doc = "An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the cus..."]
    pub balance: Option<i64>,
    #[doc = "The customer's business name."]
    pub business_name: Option<json::OwnedValue>,
    #[doc = "Balance information and default balance settings for this customer."]
    pub cash_balance: Option<stripe_types_08::customers::CreateCustomerParamsCashBalance>,
    #[doc = "An arbitrary string that you can attach to a customer object."]
    pub description: Option<String>,
    #[doc = "Customer's email address."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The customer's full name."]
    pub individual_name: Option<json::OwnedValue>,
    #[doc = "The prefix for the customer used to generate unique invoice numbers."]
    pub invoice_prefix: Option<String>,
    #[doc = "Default invoice settings for this customer."]
    pub invoice_settings: Option<stripe_types_08::customers::CreateCustomerParamsInvoiceSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The customer's full name or business name."]
    pub name: Option<String>,
    #[doc = "The sequence to be used on the customer's next invoice."]
    pub next_invoice_sequence: Option<i64>,
    pub payment_method: Option<String>,
    #[doc = "The customer's phone number."]
    pub phone: Option<String>,
    #[doc = "Customer's preferred languages, ordered by preference."]
    pub preferred_locales: Option<Vec<String>>,
    #[doc = "The customer's shipping information."]
    pub shipping: Option<stripe_types_07::customers::CreateCustomerParamsShipping>,
    pub source: Option<String>,
    #[doc = "Tax details about the customer."]
    pub tax: Option<stripe_types_05::customers::CreateCustomerParamsTax>,
    #[doc = "The customer's tax exemption."]
    pub tax_exempt: Option<stripe_types_01::customers::CreateCustomerParamsTaxExempt>,
    #[doc = "The customer's tax IDs."]
    pub tax_id_data: Option<Vec<stripe_types_05::customers::CreateCustomerParamsTaxIdDataItem>>,
    #[doc = "ID of the test clock to attach to the customer."]
    pub test_clock: Option<String>,
    pub validate: Option<bool>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("balance", self.balance.as_ref());
        form.field_opt("cash_balance", self.cash_balance.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice_prefix", self.invoice_prefix.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("next_invoice_sequence", self.next_invoice_sequence.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        if let Some(values) = self.preferred_locales.as_ref() {
            form.field_array("preferred_locales", values.as_slice());
        }
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("source", self.source.as_ref());
        form.field_opt("tax", self.tax.as_ref());
        form.field_opt("tax_exempt", self.tax_exempt.as_ref());
        if let Some(values) = self.tax_id_data.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("tax_id_data", index, item);
            }
        }
        form.field_opt("test_clock", self.test_clock.as_ref());
        form.field_opt("validate", self.validate.as_ref());
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<stripe_types_08::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer>,
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerCashBalanceParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A hash of settings for this cash balance."]
    pub settings: Option<stripe_types_07::customers::UpdateCustomerCashBalanceParamsSettings>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerCashBalanceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("settings", self.settings.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParamsCashBalance {
    #[doc = "Settings controlling the behavior of the customer's cash balance, such as reconciliation of funds received."]
    pub settings: Option<stripe_types_09::customers::UpdateCustomerParamsCashBalanceSettings>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParamsCashBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("settings", self.settings.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParamsInvoiceSettings {
    #[doc = "The list of up to 4 default custom fields to be displayed on invoices for this customer."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subsc..."]
    pub default_payment_method: Option<String>,
    #[doc = "Default footer to be displayed on invoices for this customer."]
    pub footer: Option<String>,
    #[doc = "Default options for invoice PDF rendering for this customer."]
    pub rendering_options: Option<stripe_types_07::customers::UpdateCustomerParamsInvoiceSettingsRenderingOptions>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParamsInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("rendering_options", self.rendering_options.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerSourceParams {
    #[doc = "The name of the person or business that owns the bank account."]
    pub account_holder_name: Option<String>,
    #[doc = "The type of entity that holds the account."]
    pub account_holder_type: Option<stripe_types_01::customers::UpdateCustomerSourceParamsAccountHolderType>,
    #[doc = "City/District/Suburb/Town/Village."]
    pub address_city: Option<String>,
    #[doc = "Billing address country, if provided when creating card."]
    pub address_country: Option<String>,
    #[doc = "Address line 1 (Street address/PO Box/Company name)."]
    pub address_line1: Option<String>,
    #[doc = "Address line 2 (Apartment/Suite/Unit/Building)."]
    pub address_line2: Option<String>,
    #[doc = "State/County/Province/Region."]
    pub address_state: Option<String>,
    #[doc = "ZIP or postal code."]
    pub address_zip: Option<String>,
    #[doc = "Two digit number representing the card’s expiration month."]
    pub exp_month: Option<String>,
    #[doc = "Four digit number representing the card’s expiration year."]
    pub exp_year: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Cardholder name."]
    pub name: Option<String>,
    pub owner: Option<stripe_types_09::customers::UpdateCustomerSourceParamsOwner>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerSourceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("address_city", self.address_city.as_ref());
        form.field_opt("address_country", self.address_country.as_ref());
        form.field_opt("address_line1", self.address_line1.as_ref());
        form.field_opt("address_line2", self.address_line2.as_ref());
        form.field_opt("address_state", self.address_state.as_ref());
        form.field_opt("address_zip", self.address_zip.as_ref());
        form.field_opt("exp_month", self.exp_month.as_ref());
        form.field_opt("exp_year", self.exp_year.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("owner", self.owner.as_ref());
    }
}
