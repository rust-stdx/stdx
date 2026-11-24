#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataEps {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataEpsBank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataFpx {
    #[doc = "Account holder type for FPX transaction"]
    pub account_holder_type:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataFpxAccountHolderType>,
    #[doc = "The customer's bank."]
    pub bank: stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataFpxBank,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field("bank", &self.bank);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataSofort {
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    pub country: stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSofortCountry,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "A URL for custom mandate text to render during confirmation step."]
    pub custom_mandate_url: Option<json::OwnedValue>,
    #[doc = "List of Stripe products where this mandate can be selected automatically."]
    pub default_for: Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem>>,
    #[doc = "Description of the mandate interval."]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    pub payment_schedule: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.default_for.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("default_for", index, item);
            }
        }
        form.field_opt("interval_description", self.interval_description.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsBlik {
    #[doc = "The 6-digit BLIK code that a customer has generated using their banking application."]
    pub code: Option<String>,
    #[doc = "Details of the BLIK mandate"]
    pub mandate_options:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsBlikMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    #[doc = "The method used to collect offline mandate customer acceptance."]
    pub collection_method: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct CreateSetupIntentParamsMandateData {
    #[doc = "This hash contains details about the customer acceptance of the Mandate."]
    pub customer_acceptance: stripe_types_05::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptance,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsMandateData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("customer_acceptance", &self.customer_acceptance);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Bank account verification method."]
    pub verification_method:
        Option<stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
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
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    #[doc = "Currency supported by the bank account"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types_03::setup_intents::SetupIntentPaymentMethodOptionsAcssDebitCurrency>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_types_01::setup_intents::SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
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
pub struct SetupIntentPaymentMethodOptionsUpi {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsMandateOptionsUpi>,
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    #[doc = "Your average amount value."]
    pub average_amount: Option<i64>,
    #[doc = "The maximum value you may charge a customer per purchase."]
    pub maximum_amount: Option<i64>,
    #[doc = "The lowest or minimum value you may charge a customer per purchase."]
    pub minimum_amount: Option<i64>,
    #[doc = "Interval at which the customer is making purchases"]
    pub purchase_interval: Option<
        stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval,
    >,
    #[doc = "The number of `purchase_interval` between charges"]
    pub purchase_interval_count: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("average_amount", self.average_amount.as_ref());
        form.field_opt("maximum_amount", self.maximum_amount.as_ref());
        form.field_opt("minimum_amount", self.minimum_amount.as_ref());
        form.field_opt("purchase_interval", self.purchase_interval.as_ref());
        form.field_opt("purchase_interval_count", self.purchase_interval_count.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type: Option<
        stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType,
    >,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<
        stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule,
    >,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
    #[doc = "Date, in YYYY-MM-DD format, from which payments will be collected."]
    pub start_date: Option<json::OwnedValue>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}
