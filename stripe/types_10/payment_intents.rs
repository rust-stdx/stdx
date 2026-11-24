#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CapturePaymentIntentParamsHooks {
    #[doc = "Arguments passed in automations"]
    pub inputs: Option<stripe_types_07::payment_intents::CapturePaymentIntentParamsHooksInputs>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CapturePaymentIntentParamsHooks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inputs", self.inputs.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataUpiMandateOptions>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsBillie {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCaptureMethod>,
    #[doc = "Registration details about the buyer's organization."]
    pub company_details:
        Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails>,
    #[doc = "An identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("company_details", self.company_details.as_ref());
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallments {
    #[doc = "Setting to true enables installments for this PaymentIntent."]
    pub enabled: Option<bool>,
    #[doc = "The selected installment plan to use for this payment attempt."]
    pub plan:
        Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("plan", self.plan.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresent {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod,
    >,
    #[doc = "Request ability to capture this payment beyond the standard authorization validity window..."]
    pub request_extended_authorization: Option<bool>,
    #[doc = "Request ability to [increment](https://docs.stripe.com/terminal/features/incremental-authorizations) this PaymentInte..."]
    pub request_incremental_authorization_support: Option<bool>,
    #[doc = "Network routing priority on co-branded EMV cards supporting domestic debit and international card schemes."]
    pub routing:
        Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresentRouting>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresent {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("request_extended_authorization", self.request_extended_authorization.as_ref());
        form.field_opt(
            "request_incremental_authorization_support",
            self.request_incremental_authorization_support.as_ref(),
        );
        form.field_opt("routing", self.routing.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer: Option<
        stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer,
    >,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsKlarna {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaCaptureMethod>,
    #[doc = "On-demand details if setting up or charging an on-demand payment."]
    pub on_demand:
        Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand>,
    #[doc = "Preferred language of the Klarna authorization page that the customer is redirected to"]
    pub preferred_locale:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage>,
    #[doc = "Subscription details if setting up or charging a subscription."]
    pub subscriptions: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("on_demand", self.on_demand.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
    #[doc = "The timestamp at which the Pix expires (between 10 and 1209600 seconds in the future)."]
    pub expires_at: Option<i64>,
    #[doc = "Additional fields for mandate creation."]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBillie {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillieCaptureMethod>,
    #[doc = "Registration details about the buyer's organization."]
    pub company_details:
        Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails>,
    #[doc = "An identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("company_details", self.company_details.as_ref());
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecure {
    #[doc = "The `transStatus` returned from the card Issuer’s ACS in the ARes."]
    pub ares_trans_status: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    #[doc = "The cryptogram, also known as the 'authentication value' (AAV, CAVV or AEVV)."]
    pub cryptogram: String,
    #[doc = "The Electronic Commerce Indicator (ECI) is returned by your 3D Secure provider and indicates what degree of authentic..."]
    pub electronic_commerce_indicator: Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    #[doc = "The exemption requested via 3DS and accepted by the issuer at authentication time."]
    pub exemption_indicator: Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator>,
    #[doc = "Network specific 3DS fields."]
    pub network_options: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions>,
    #[doc = "The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the AReq sent to the card Issuer's ACS."]
    pub requestor_challenge_indicator: Option<String>,
    #[doc = "For 3D Secure 1, the XID."]
    pub transaction_id: String,
    #[doc = "The version of 3D Secure that was performed."]
    pub version: stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ares_trans_status", self.ares_trans_status.as_ref());
        form.field("cryptogram", &self.cryptogram);
        form.field_opt("electronic_commerce_indicator", self.electronic_commerce_indicator.as_ref());
        form.field_opt("exemption_indicator", self.exemption_indicator.as_ref());
        form.field_opt("network_options", self.network_options.as_ref());
        form.field_opt("requestor_challenge_indicator", self.requestor_challenge_indicator.as_ref());
        form.field("transaction_id", &self.transaction_id);
        form.field("version", &self.version);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
    #[doc = "The timestamp at which the Pix expires (between 10 and 1209600 seconds in the future)."]
    pub expires_at: Option<i64>,
    #[doc = "Additional fields for mandate creation."]
    pub mandate_options:
        Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
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
pub struct PaymentIntentAmountDetailsLineItem {
    #[doc = "The discount applied on this line item represented in the smallest currency unit..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::payment_intents::PaymentIntentAmountDetailsLineItemObject,
    #[doc = "Payment method-specific information for line items."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<stripe_types_09::payment_intents::PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions>,
    #[doc = "The product code of the line item, such as an SKU."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[doc = "The product name of the line item."]
    #[json(default)]
    pub product_name: String,
    #[doc = "The quantity of items."]
    #[json(default)]
    pub quantity: i64,
    #[doc = "Contains information about the tax on the item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_types_03::payment_intents::PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourceTax>,
    #[doc = "The unit cost of the line item represented in the smallest currency unit..."]
    #[json(default)]
    pub unit_cost: i64,
    #[doc = "A unit of measure for the line item, such as gallons, feet, meters, etc."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
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
impl crate::PagedItem for PaymentIntentAmountDetailsLineItem {
    fn id(&self) -> &str {
        &self.id
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
pub struct PaymentIntentPaymentMethodOptionsCard {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<
        stripe_types_02::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod,
    >,
    #[doc = "Installment details for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<stripe_types_07::payment_methods::PaymentMethodOptionsCardInstallments>,
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsCardMandateOptions>,
    #[doc = "Selected network to process this payment intent on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_types_00::payment_intents::PaymentIntentPaymentMethodOptionsCardNetwork>,
    #[doc = "Request ability to capture beyond the standard authorization validity window..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "Request ability to [increment the authorization](https://docs.stripe.com/payments/incremental-authorization) for this..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "Request ability to make [multiple captures](https://docs.stripe.com/payments/multicapture) for this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_multicapture:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "Request ability to [overcapture](https://docs.stripe.com/payments/overcapture) for this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_overcapture:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<stripe_types_02::payment_intents::PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "When enabled, using a card that is attached to a customer will require the CVC to be provided again (i.e."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub require_cvc_recollection: Option<bool>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<stripe_types_03::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
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
pub struct PaymentIntentPaymentMethodOptionsUsBankAccount {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<stripe_types_08::financial_connections::LinkedAccountOptionsCommon>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<stripe_types_03::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[doc = "The purpose of the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_purpose:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_types_00::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsHooks {
    #[doc = "Arguments passed in automations"]
    pub inputs: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsHooksInputs>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsHooks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inputs", self.inputs.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataUpiMandateOptions>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    #[doc = "Cartes Bancaires-specific 3DS fields."]
    pub cartes_bancaires: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsKlarna {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaCaptureMethod>,
    #[doc = "On-demand details if setting up or charging an on-demand payment."]
    pub on_demand:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand>,
    #[doc = "Preferred language of the Klarna authorization page that the customer is redirected to"]
    pub preferred_locale:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage>,
    #[doc = "Subscription details if setting up or charging a subscription."]
    pub subscriptions: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("on_demand", self.on_demand.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptions>,
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUpiSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}
