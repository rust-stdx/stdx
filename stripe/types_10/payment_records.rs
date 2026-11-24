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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetails {
    #[doc = "The authorization code of the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[doc = "Card brand."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsBrand>,
    #[doc = "When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<i64>,
    #[doc = "Check results by Card networks on Card address and CVC at time of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<stripe_types_06::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks>,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "A high-level description of the type of cards issued in this range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Two-digit number representing the card's expiration month."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[doc = "Four-digit number representing the card's expiration year."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[doc = "Uniquely identifies this particular card number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Card funding type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_types_01::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsFunding>,
    #[doc = "Issuer identification number of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[doc = "Installment details for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<stripe_types_08::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceInstallments>,
    #[doc = "The name of the card's issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The last four digits of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "True if this payment was marked as MOTO and out of scope for SCA."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    #[doc = "Identifies which network this charge was processed on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_types_02::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsBrand>,
    #[doc = "Advice code from the card network for the failed payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_advice_code: Option<String>,
    #[doc = "Decline code from the card network for the failed payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_decline_code: Option<String>,
    #[doc = "If this card has network token credentials, this contains the details of the network token credentials."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_token: Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceNetworkToken>,
    #[doc = "This is used by the financial networks to identify a transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_transaction_id: Option<String>,
    #[doc = "Populated if this transaction used 3D Secure authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<stripe_types_06::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceThreeDSecure>,
    #[doc = "If this Card is part of a card wallet, this contains the details of the card wallet."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceWallet>,
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentAttemptInformationalParams {
    #[doc = "Customer information for this payment."]
    pub customer_details:
        Option<stripe_types_06::payment_records::ReportPaymentAttemptInformationalParamsCustomerDetails>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Shipping information for this payment."]
    pub shipping_details:
        Option<stripe_types_09::payment_records::ReportPaymentAttemptInformationalParamsShippingDetails>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentAttemptInformationalParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer_details", self.customer_details.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("shipping_details", self.shipping_details.as_ref());
    }
}
