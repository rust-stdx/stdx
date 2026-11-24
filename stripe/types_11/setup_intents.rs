#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodDataUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataUpiMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodDataUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Bank account verification method."]
    pub verification_method:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    #[doc = "Cartes Bancaires-specific 3DS fields."]
    pub cartes_bancaires: Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options:
        Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("permissions", index, item);
            }
        }
        if let Some(values) = self.prefetch.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prefetch", index, item);
            }
        }
        form.field_opt("return_url", self.return_url.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecure {
    #[doc = "The `transStatus` returned from the card Issuer’s ACS in the ARes."]
    pub ares_trans_status: Option<stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    #[doc = "The cryptogram, also known as the 'authentication value' (AAV, CAVV or AEVV)."]
    pub cryptogram: Option<String>,
    #[doc = "The Electronic Commerce Indicator (ECI) is returned by your 3D Secure provider and indicates what degree of authentic..."]
    pub electronic_commerce_indicator: Option<stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    #[doc = "Network specific 3DS fields."]
    pub network_options: Option<stripe_types_08::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions>,
    #[doc = "The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the AReq sent to the card Issuer's ACS."]
    pub requestor_challenge_indicator: Option<String>,
    #[doc = "For 3D Secure 1, the XID."]
    pub transaction_id: Option<String>,
    #[doc = "The version of 3D Secure that was performed."]
    pub version: Option<stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ares_trans_status", self.ares_trans_status.as_ref());
        form.field_opt("cryptogram", self.cryptogram.as_ref());
        form.field_opt("electronic_commerce_indicator", self.electronic_commerce_indicator.as_ref());
        form.field_opt("network_options", self.network_options.as_ref());
        form.field_opt("requestor_challenge_indicator", self.requestor_challenge_indicator.as_ref());
        form.field_opt("transaction_id", self.transaction_id.as_ref());
        form.field_opt("version", self.version.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsPix {
    #[doc = "Additional fields for mandate creation."]
    pub mandate_options:
        Option<stripe_types_09::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_09::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            form.field_array("permissions", values.as_slice());
        }
        if let Some(values) = self.prefetch.as_ref() {
            form.field_array("prefetch", values.as_slice());
        }
        form.field_opt("return_url", self.return_url.as_ref());
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
pub struct SetupIntentPaymentMethodOptionsUsBankAccount {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<stripe_types_08::financial_connections::LinkedAccountOptionsCommon>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_types_01::setup_intents::SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitCurrency>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Bank account verification method."]
    pub verification_method:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsKlarna {
    #[doc = "The currency of the SetupIntent."]
    pub currency: Option<String>,
    #[doc = "On-demand details if setting up a payment method for on-demand payments."]
    pub on_demand: Option<stripe_types_07::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand>,
    #[doc = "Preferred language of the Klarna authorization page that the customer is redirected to"]
    pub preferred_locale:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale>,
    #[doc = "Subscription details if setting up or charging a subscription"]
    pub subscriptions: Option<json::OwnedValue>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("on_demand", self.on_demand.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsPix {
    #[doc = "Additional fields for mandate creation."]
    pub mandate_options:
        Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_07::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUpiMandateOptions>,
    pub setup_future_usage:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("permissions", index, item);
            }
        }
        if let Some(values) = self.prefetch.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prefetch", index, item);
            }
        }
        form.field_opt("return_url", self.return_url.as_ref());
    }
}
