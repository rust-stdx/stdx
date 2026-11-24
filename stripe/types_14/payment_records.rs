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
pub struct PaymentRecord {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_authorized: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_canceled: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_failed: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_guaranteed: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_requested: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[doc = "ID of the Connect application that created the PaymentRecord."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Customer information for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_details:
        Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourceCustomerDetails>,
    #[doc = "Indicates whether the customer was present in your checkout flow during this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_presence: Option<stripe_types_04::payment_records::PaymentRecordCustomerPresence>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "ID of the latest Payment Attempt Record attached to this Payment Record."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub latest_payment_attempt_record: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::payment_records::PaymentRecordObject,
    #[doc = "Information about the Payment Method debited for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details:
        Option<crate::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub processor_details:
        Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourceProcessorDetails>,
    #[doc = "Indicates who reported the payment."]
    #[json(default)]
    pub reported_by: stripe_types_02::payment_records::PaymentRecordReportedBy,
    #[doc = "Shipping information for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_details:
        Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourceShippingDetails>,
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
impl crate::PagedItem for PaymentRecord {
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
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_types_00::payment_methods::PaymentMethodDetailsAchCreditTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_types_06::payment_methods::PaymentMethodDetailsAchDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_02::payment_methods::PaymentMethodDetailsPaymentRecordAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordAffirm>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordAfterpayClearpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordAlipay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_06::payment_methods::PaymentMethodDetailsPaymentRecordAlma>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_09::payment_methods::PaymentMethodDetailsPaymentRecordAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::payment_methods::PaymentMethodDetailsPaymentRecordBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordBillie>,
    #[doc = "The billing details associated with the method of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details:
        Option<stripe_types_06::payment_records::PaymentsPrimitivesPaymentRecordsResourceBillingDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordBizum>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card:
        Option<stripe_types_10::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_08::payment_methods::PaymentMethodDetailsCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordCashapp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_05::payment_methods::PaymentMethodDetailsCrypto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom:
        Option<stripe_types_00::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_01::payment_methods::PaymentMethodDetailsCustomerBalance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_06::payment_methods::PaymentMethodDetailsPaymentRecordEps>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_06::payment_methods::PaymentMethodDetailsFpx>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordGiropay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_03::payment_methods::PaymentMethodDetailsGrabpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::payment_methods::PaymentMethodDetailsPaymentRecordIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types_09::payment_methods::PaymentMethodDetailsInteracPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordKakaoPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_07::payment_methods::PaymentMethodDetailsPaymentRecordKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_09::payment_methods::PaymentMethodDetailsPaymentRecordKonbini>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_06::payment_methods::PaymentMethodDetailsKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordMbWay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_05::payment_methods::PaymentMethodDetailsPaymentRecordMobilepay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub momo: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordMomo>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordMultibanco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsPaymentRecordNaverPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_02::payment_methods::PaymentMethodDetailsPaymentRecordNzBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordOxxo>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_05::payment_methods::PaymentMethodDetailsPaymentRecordP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordPayByBank>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordPayco>,
    #[doc = "ID of the Stripe PaymentMethod used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordPaynow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_07::payment_methods::PaymentMethodDetailsPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordPaypay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordPromptpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_08::payment_methods::PaymentMethodDetailsPaymentRecordRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsPaymentRecordSamsungPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordSatispay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordScalapay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_types_01::payment_methods::PaymentMethodDetailsSepaCreditTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_04::payment_methods::PaymentMethodDetailsPaymentRecordSequra>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::payment_methods::PaymentMethodDetailsPaymentRecordSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_account: Option<stripe_types_00::payment_methods::PaymentMethodDetailsStripeAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordSunbit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordSwish>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordTwint>,
    #[doc = "The type of transaction-specific details of the payment method used in the payment."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::payment_methods::PaymentMethodDetailsPaymentRecordUsBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_types_03::payment_methods::PaymentMethodDetailsWechat>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsPaymentRecordWechatPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPaymentRecordZip>,
}
