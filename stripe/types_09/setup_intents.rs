#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataUsBankAccountAccountHolderType>,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    #[doc = "Your average amount value."]
    pub average_amount: Option<i64>,
    #[doc = "The maximum value you may charge a customer per purchase."]
    pub maximum_amount: Option<i64>,
    #[doc = "The lowest or minimum value you may charge a customer per purchase."]
    pub minimum_amount: Option<i64>,
    #[doc = "Interval at which the customer is making purchases"]
    pub purchase_interval: Option<
        stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval,
    >,
    #[doc = "The number of `purchase_interval` between charges"]
    pub purchase_interval_count: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand {
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
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<
        stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof,
    >,
    #[doc = "Type of amount."]
    pub amount_type:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<
        stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule,
    >,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsPixMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("start_date", self.start_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodDataUpiMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: i64,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType,
    #[doc = "Currency in which future payments will be charged."]
    pub currency: String,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
    #[doc = "Specifies payment frequency."]
    pub interval: stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval,
    #[doc = "The number of intervals between payments."]
    pub interval_count: Option<i64>,
    #[doc = "Unique identifier for the mandate or subscription."]
    pub reference: String,
    #[doc = "Start date of the mandate or subscription."]
    pub start_date: i64,
    #[doc = "Specifies the type of mandates supported."]
    pub supported_types: Option<Vec<stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("amount_type", &self.amount_type);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
        form.field("reference", &self.reference);
        form.field("start_date", &self.start_date);
        if let Some(values) = self.supported_types.as_ref() {
            form.field_array("supported_types", values.as_slice());
        }
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsKlarna {
    #[doc = "The currency of the SetupIntent."]
    pub currency: Option<String>,
    #[doc = "On-demand details if setting up a payment method for on-demand payments."]
    pub on_demand: Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand>,
    #[doc = "Preferred language of the Klarna authorization page that the customer is redirected to"]
    pub preferred_locale:
        Option<stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale>,
    #[doc = "Subscription details if setting up or charging a subscription"]
    pub subscriptions: Option<json::OwnedValue>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("on_demand", self.on_demand.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options:
        Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<
        stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof,
    >,
    #[doc = "Type of amount."]
    pub amount_type:
        Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<
        stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule,
    >,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("start_date", self.start_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
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
pub struct SetupIntentNextAction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik_authorize: Option<stripe_types_02::payment_intents::PaymentIntentNextActionBlikAuthorize>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_types_05::payment_intents::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix_display_qr_code: Option<stripe_types_01::setup_intents::SetupIntentNextActionPixDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<stripe_types_00::setup_intents::SetupIntentNextActionRedirectToUrl>,
    #[doc = "Type of the next action to perform."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi_handle_redirect_or_display_qr_code:
        Option<stripe_types_06::payment_intents::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>,
    #[doc = "When confirming a SetupIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authenti..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<std::collections::HashMap<String, json::OwnedValue>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits: Option<stripe_types_06::setup_intents::SetupIntentNextActionVerifyWithMicrodeposits>,
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
pub struct SetupIntentPaymentMethodOptionsBlik {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsBlik>,
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
pub struct SetupIntentPaymentMethodOptionsPix {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsMandateOptionsPix>,
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataEps {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataEpsBank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataIdeal {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataIdealBank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataKlarna {
    #[doc = "Customer's date of birth"]
    pub dob: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataKlarnaDob>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dob", self.dob.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct UpdateSetupIntentParamsPaymentMethodDataSofort {
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    pub country: stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSofortCountry,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataUsBankAccountAccountHolderType>,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "A URL for custom mandate text to render during confirmation step."]
    pub custom_mandate_url: Option<json::OwnedValue>,
    #[doc = "List of Stripe products where this mandate can be selected automatically."]
    pub default_for: Option<Vec<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem>>,
    #[doc = "Description of the mandate interval."]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    pub payment_schedule: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
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
pub struct UpdateSetupIntentParamsPaymentMethodOptionsBlik {
    #[doc = "The 6-digit BLIK code that a customer has generated using their banking application."]
    pub code: Option<String>,
    #[doc = "Details of the BLIK mandate"]
    pub mandate_options:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsBlikMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    #[doc = "The method used to collect offline mandate customer acceptance."]
    pub collection_method: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    #[doc = "Triggers validations to run across the selected networks"]
    pub requested: Option<Vec<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.requested.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("requested", index, item);
            }
        }
    }
}
