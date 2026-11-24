#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataIdeal {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataIdealBank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataKlarna {
    #[doc = "Customer's date of birth"]
    pub dob: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataKlarnaDob>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dob", self.dob.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: i64,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType,
    #[doc = "Currency in which future payments will be charged."]
    pub currency: String,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
    #[doc = "Specifies payment frequency."]
    pub interval: stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval,
    #[doc = "The number of intervals between payments."]
    pub interval_count: Option<i64>,
    #[doc = "Unique identifier for the mandate or subscription."]
    pub reference: String,
    #[doc = "Start date of the mandate or subscription."]
    pub start_date: i64,
    #[doc = "Specifies the type of mandates supported."]
    pub supported_types: Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptions {
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
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("supported_types", index, item);
            }
        }
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires {
    #[doc = "The cryptogram calculation algorithm used by the card Issuer's ACS to calculate the Authentication cryptogram."]
    pub cb_avalgo: stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    #[doc = "The exemption indicator returned from Cartes Bancaires in the ARes."]
    pub cb_exemption: Option<String>,
    #[doc = "The risk score returned from Cartes Bancaires in the ARes."]
    pub cb_score: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("cb_avalgo", &self.cb_avalgo);
        form.field_opt("cb_exemption", self.cb_exemption.as_ref());
        form.field_opt("cb_score", self.cb_score.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type: Option<
        stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType,
    >,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<
        stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule,
    >,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
    #[doc = "Date, in YYYY-MM-DD format, from which payments will be collected."]
    pub start_date: Option<json::OwnedValue>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    #[doc = "Triggers validations to run across the selected networks"]
    pub requested: Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.requested.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("requested", index, item);
            }
        }
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type:
        Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountHolderType>,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type:
        Option<stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataUsBankAccount {
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
pub struct CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    #[doc = "Cartes Bancaires-specific 3DS fields."]
    pub cartes_bancaires: Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
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
pub struct SetupIntentPaymentMethodOptionsCard {
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_06::setup_intents::SetupIntentPaymentMethodOptionsCardMandateOptions>,
    #[doc = "Selected network to process this SetupIntent on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_types_04::setup_intents::SetupIntentPaymentMethodOptionsCardNetwork>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<stripe_types_02::setup_intents::SetupIntentPaymentMethodOptionsCardRequestThreeDSecure>,
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
pub struct SetupIntentPaymentMethodOptionsPayto {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_06::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsPayto>,
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct UpdateSetupIntentParamsPaymentMethodDataFpx {
    #[doc = "Account holder type for FPX transaction"]
    pub account_holder_type:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataFpxAccountHolderType>,
    #[doc = "The customer's bank."]
    pub bank: stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataFpxBank,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field("bank", &self.bank);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: i64,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType,
    #[doc = "Currency in which future payments will be charged."]
    pub currency: String,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
    #[doc = "Specifies payment frequency."]
    pub interval: stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval,
    #[doc = "The number of intervals between payments."]
    pub interval_count: Option<i64>,
    #[doc = "Unique identifier for the mandate or subscription."]
    pub reference: String,
    #[doc = "Start date of the mandate or subscription."]
    pub start_date: i64,
    #[doc = "Specifies the type of mandates supported."]
    pub supported_types: Option<Vec<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptions {
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
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("supported_types", index, item);
            }
        }
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires {
    #[doc = "The cryptogram calculation algorithm used by the card Issuer's ACS to calculate the Authentication cryptogram."]
    pub cb_avalgo: stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    #[doc = "The exemption indicator returned from Cartes Bancaires in the ARes."]
    pub cb_exemption: Option<String>,
    #[doc = "The risk score returned from Cartes Bancaires in the ARes."]
    pub cb_score: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("cb_avalgo", &self.cb_avalgo);
        form.field_opt("cb_exemption", self.cb_exemption.as_ref());
        form.field_opt("cb_score", self.cb_score.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<
        stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof,
    >,
    #[doc = "Type of amount."]
    pub amount_type:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<
        stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule,
    >,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptions {
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
pub struct UpdateSetupIntentParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsSepaDebitMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}
