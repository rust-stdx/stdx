#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationToken {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Time at which this ConfirmationToken expires and can no longer be used to confirm a PaymentIntent or SetupIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Data used for generating a Mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_data: Option<stripe_types_08::confirmation_tokens::ConfirmationTokensResourceMandateData>,
    #[doc = "Set of key-value pairs that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::confirmation_tokens::ConfirmationTokenObject,
    #[doc = "ID of the PaymentIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not ye..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,
    #[doc = "Payment-method-specific configuration for this ConfirmationToken."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options:
        Option<stripe_types_12::confirmation_tokens::ConfirmationTokensResourcePaymentMethodOptions>,
    #[doc = "Payment details collected by the Payment Element, used to create a PaymentMethod when a PaymentIntent or SetupIntent ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_preview: Option<crate::confirmation_tokens::ConfirmationTokensResourcePaymentMethodPreview>,
    #[doc = "Return URL used to confirm the Intent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this ConfirmationToken's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::confirmation_tokens::ConfirmationTokenSetupFutureUsage>,
    #[doc = "ID of the SetupIntent that this ConfirmationToken was used to confirm, or null if this ConfirmationToken has not yet ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<String>,
    #[doc = "Shipping information collected on this ConfirmationToken."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_05::confirmation_tokens::ConfirmationTokensResourceShipping>,
    #[doc = "Indicates whether the Stripe SDK is used to handle confirmation flow."]
    #[json(default)]
    pub use_stripe_sdk: bool,
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
impl crate::PagedItem for ConfirmationToken {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourcePaymentMethodPreview {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_03::payment_methods::PaymentMethodAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_00::payment_methods::PaymentMethodAffirm>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_00::payment_methods::PaymentMethodAfterpayClearpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_03::payment_intents::PaymentFlowsPrivatePaymentMethodsAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redisplay:
        Option<stripe_types_04::confirmation_tokens::ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_02::payment_methods::PaymentMethodAlma>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_04::payment_methods::PaymentMethodAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_03::payment_methods::PaymentMethodAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_02::payment_methods::PaymentMethodBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_00::payment_methods::PaymentMethodBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_01::payment_methods::PaymentMethodBillie>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_05::billing::BillingDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_00::payment_methods::PaymentMethodBizum>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_02::payment_methods::PaymentMethodBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_02::payment_methods::PaymentMethodBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_14::payment_methods::PaymentMethodCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_08::payment_methods::PaymentMethodCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_04::payment_methods::PaymentMethodCashapp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_02::payment_methods::PaymentMethodCrypto>,
    #[doc = "The ID of the Customer to which this PaymentMethod is saved."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::customers::Customer>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_01::payment_methods::PaymentMethodCustomerBalance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_06::payment_methods::PaymentMethodEps>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_05::payment_methods::PaymentMethodFpx>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_01::payment_methods::PaymentMethodGiropay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_02::payment_methods::PaymentMethodGrabpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types_06::payment_methods::PaymentMethodIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types_06::payment_methods::PaymentMethodInteracPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_04::payment_methods::PaymentMethodKakaoPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_06::payment_methods::PaymentMethodKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_03::payment_methods::PaymentMethodKonbini>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_05::payment_methods::PaymentMethodKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_04::payment_methods::PaymentMethodLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_03::payment_methods::PaymentMethodMbWay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_01::payment_methods::PaymentMethodMobilepay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_04::payment_methods::PaymentMethodMultibanco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_05::payment_methods::PaymentMethodNaverPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_02::payment_methods::PaymentMethodNzBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_01::payment_methods::PaymentMethodOxxo>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_05::payment_methods::PaymentMethodP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<stripe_types_02::payment_methods::PaymentMethodPayByBank>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_04::payment_methods::PaymentMethodPayco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_03::payment_methods::PaymentMethodPaynow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_01::payment_methods::PaymentMethodPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay: Option<stripe_types_04::payment_methods::PaymentMethodPaypay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_01::payment_methods::PaymentMethodPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_03::payment_methods::PaymentMethodPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types_00::payment_methods::PaymentMethodPromptpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_01::payment_methods::PaymentMethodRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_02::payment_methods::PaymentMethodSamsungPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_01::payment_methods::PaymentMethodSatispay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_02::payment_methods::PaymentMethodScalapay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_14::payment_methods::PaymentMethodSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_00::payment_methods::PaymentMethodSequra>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_03::payment_methods::PaymentMethodSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_01::payment_methods::PaymentMethodSunbit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_03::payment_methods::PaymentMethodSwish>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_04::payment_methods::PaymentMethodTwint>,
    #[doc = "The type of the PaymentMethod."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::confirmation_tokens::ConfirmationTokensResourcePaymentMethodPreviewType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_03::payment_methods::PaymentMethodUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_11::payment_methods::PaymentMethodUsBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_03::payment_methods::PaymentMethodWechatPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_04::payment_methods::PaymentMethodZip>,
}
