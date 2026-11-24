#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParams {
    #[doc = "The amount to capture from the authorization."]
    pub capture_amount: Option<i64>,
    #[doc = "Whether to close the authorization after capture."]
    pub close_authorization: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Additional purchase information that is optionally provided by the merchant."]
    pub purchase_details:
        Option<stripe_types_12::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetails>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_amount", self.capture_amount.as_ref());
        form.field_opt("close_authorization", self.close_authorization.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("purchase_details", self.purchase_details.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParams {
    #[doc = "The total amount to attempt to capture."]
    pub amount: i64,
    #[doc = "Card associated with this transaction."]
    pub card: String,
    #[doc = "The currency of the capture."]
    pub currency: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened."]
    pub merchant_data: Option<stripe_types_09::test_helpers::CreateForceCaptureParamsMerchantData>,
    #[doc = "Additional purchase information that is optionally provided by the merchant."]
    pub purchase_details: Option<stripe_types_12::test_helpers::CreateForceCaptureParamsPurchaseDetails>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("card", &self.card);
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("merchant_data", self.merchant_data.as_ref());
        form.field_opt("purchase_details", self.purchase_details.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "ID of an existing PaymentMethod."]
    pub payment_method: Option<String>,
    #[doc = "If provided, this hash will be used to create a PaymentMethod."]
    pub payment_method_data:
        Option<stripe_types_10::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodData>,
    #[doc = "Payment-method-specific configuration for this ConfirmationToken."]
    pub payment_method_options:
        Option<stripe_types_12::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodOptions>,
    #[doc = "Return URL used to confirm the Intent."]
    pub return_url: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this ConfirmationToken's payment method."]
    pub setup_future_usage:
        Option<stripe_types_04::test_helpers::CreateTestHelperConfirmationTokenParamsSetupFutureUsage>,
    #[doc = "Shipping information for this ConfirmationToken."]
    pub shipping: Option<stripe_types_07::test_helpers::CreateTestHelperConfirmationTokenParamsShipping>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParams {
    #[doc = "The total amount to attempt to refund."]
    pub amount: i64,
    #[doc = "Card associated with this unlinked refund transaction."]
    pub card: String,
    #[doc = "The currency of the unlinked refund."]
    pub currency: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened."]
    pub merchant_data: Option<stripe_types_08::test_helpers::CreateUnlinkedRefundParamsMerchantData>,
    #[doc = "Additional purchase information that is optionally provided by the merchant."]
    pub purchase_details: Option<stripe_types_12::test_helpers::CreateUnlinkedRefundParamsPurchaseDetails>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("card", &self.card);
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("merchant_data", self.merchant_data.as_ref());
        form.field_opt("purchase_details", self.purchase_details.as_ref());
    }
}
