#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType,
    >,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataUsBankAccountAccountHolderType,
    >,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAlipay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAlipay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails {
    #[doc = "The address the company or entity is registered with."]
    pub registered_address: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    pub registration_type: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT id number"]
    pub vat: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("registered_name", self.registered_name.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("registration_type", self.registration_type.as_ref());
        form.field_opt("vat", self.vat.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsBlik {
    #[doc = "The 6-digit BLIK code that a customer has generated using their banking application."]
    pub code: Option<String>,
    #[doc = "Details of the BLIK mandate"]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBlikMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBlikSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresentRouting {
    #[doc = "Routing requested priority"]
    pub requested_priority: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresentRouting {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("requested_priority", self.requested_priority.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires {
    #[doc = "The cryptogram calculation algorithm used by the card Issuer's ACS to calculate the Authentication cryptogram."]
    pub cb_avalgo: stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    #[doc = "The exemption indicator returned from Cartes Bancaires in the ARes."]
    pub cb_exemption: Option<String>,
    #[doc = "The risk score returned from Cartes Bancaires in the ARes."]
    pub cb_score: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("cb_avalgo", &self.cb_avalgo);
        form.field_opt("cb_exemption", self.cb_exemption.as_ref());
        form.field_opt("cb_score", self.cb_score.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCrypto {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCryptoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCrypto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsIdeal {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsKakaoPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKakaoPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKakaoPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsKakaoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsP24 {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsP24SetupFutureUsage>,
    #[doc = "Confirm that the payer has accepted the P24 terms and conditions."]
    pub tos_shown_and_accepted: Option<bool>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("tos_shown_and_accepted", self.tos_shown_and_accepted.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPaypal {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaypalCaptureMethod>,
    #[doc = "[Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the cu..."]
    pub preferred_locale:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale>,
    #[doc = "A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID."]
    pub reference: Option<String>,
    #[doc = "The risk correlation ID for an on-session payment using a saved PayPal payment method."]
    pub risk_correlation_id: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaypalSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("risk_correlation_id", self.risk_correlation_id.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPromptpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPromptpaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPromptpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsRevolutPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsRevolutPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsRevolutPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsRevolutPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSatispay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSatispayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSatispaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSatispay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSequra {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSequraCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSequraSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSequra {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSwish {
    #[doc = "A reference for this payment to be displayed in the Swish app."]
    pub reference: Option<json::OwnedValue>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSwishSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSwish {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    #[doc = "Triggers validations to run across the selected networks"]
    pub requested: Option<Vec<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.requested.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("requested", index, item);
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct CreatePaymentIntentParamsMandateData {
    #[doc = "This hash contains details about the customer acceptance of the Mandate."]
    pub customer_acceptance: stripe_types_06::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptance,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsMandateData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("customer_acceptance", &self.customer_acceptance);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpayCaptureMethod,
    >,
    #[doc = "An internal identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAuBecsDebit {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBancontact {
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    pub preferred_language: Option<
        stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBancontactSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBlik {
    #[doc = "The 6-digit BLIK code that a customer has generated using their banking application."]
    pub code: Option<String>,
    #[doc = "Details of the BLIK mandate"]
    pub mandate_options:
        Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBlikSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: i64,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
    #[doc = "Specifies payment frequency."]
    pub interval: stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval,
    #[doc = "The number of intervals between payments."]
    pub interval_count: Option<i64>,
    #[doc = "Unique identifier for the mandate or subscription."]
    pub reference: String,
    #[doc = "Start date of the mandate or subscription."]
    pub start_date: i64,
    #[doc = "Specifies the type of mandates supported."]
    pub supported_types: Option<Vec<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("amount_type", &self.amount_type);
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

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCrypto {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCryptoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCrypto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer,
    >,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type: Option<
        stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsEps {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsGrabpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsGrabpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsIdeal {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsKlarna {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKlarnaCaptureMethod>,
    #[doc = "On-demand details if setting up or charging an on-demand payment."]
    pub on_demand:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand>,
    #[doc = "Preferred language of the Klarna authorization page that the customer is redirected to"]
    pub preferred_locale:
        Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage>,
    #[doc = "Subscription details if setting up or charging a subscription."]
    pub subscriptions: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("on_demand", self.on_demand.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsKonbini {
    #[doc = "An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores."]
    pub confirmation_number: Option<json::OwnedValue>,
    #[doc = "The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire."]
    pub expires_after_days: Option<json::OwnedValue>,
    #[doc = "The timestamp at which the Konbini payment instructions will expire."]
    pub expires_at: Option<json::OwnedValue>,
    #[doc = "A product descriptor of up to 22 characters, which will appear to customers at the convenience store."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsMultibanco {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMultibancoSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsMultibanco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsNaverPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsNaverPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsNaverPaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsNzBankAccount {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsNzBankAccountSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsOxxo {
    #[doc = "The number of calendar days before an OXXO voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsOxxo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPaypal {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypalCaptureMethod>,
    #[doc = "[Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the cu..."]
    pub preferred_locale:
        Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale>,
    #[doc = "A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID."]
    pub reference: Option<String>,
    #[doc = "The risk correlation ID for an on-session payment using a saved PayPal payment method."]
    pub risk_correlation_id: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypalSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("risk_correlation_id", self.risk_correlation_id.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSamsungPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSamsungPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSamsungPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSamsungPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSequra {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSequraCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSequraSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSequra {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct IncrementAuthorizationParamsHooksInputs {
    #[doc = "Tax arguments for automations"]
    pub tax: Option<stripe_types_05::payment_intents::IncrementAuthorizationParamsHooksInputsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for IncrementAuthorizationParamsHooksInputs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tax", self.tax.as_ref());
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
pub struct PaymentFlowsAmountDetailsResourceLineItemsListResourceLineItemResourcePaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_03::payment_intents::PaymentFlowsPrivatePaymentMethodsCardPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_04::payment_intents::PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_01::payment_intents::PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_06::payment_intents::PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptions>,
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
pub struct PaymentFlowsPaymentIntentAsyncWorkflows {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<stripe_types_05::payment_intents::PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputs>,
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
pub struct PaymentIntentPaymentMethodOptionsAcssDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<stripe_types_03::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_types_00::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsAmountDetails {
    #[doc = "The total discount applied on the transaction represented in the smallest currency unit..."]
    pub discount_amount: Option<json::OwnedValue>,
    #[doc = "Set to `false` to return arithmetic validation errors in the response without failing the request."]
    pub enforce_arithmetic_validation: Option<bool>,
    #[doc = "A list of line items, each containing information about a product in the PaymentIntent."]
    pub line_items: Option<json::OwnedValue>,
    #[doc = "Contains information about the shipping portion of the amount."]
    pub shipping: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsAmountDetailsShipping>,
    #[doc = "Contains information about the tax portion of the amount."]
    pub tax: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsAmountDetailsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsAmountDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enforce_arithmetic_validation", self.enforce_arithmetic_validation.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataEps {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataEpsBank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataKlarna {
    #[doc = "Customer's date of birth"]
    pub dob: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataKlarnaDob>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dob", self.dob.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataSofort {
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    pub country: stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSofortCountry,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAffirm {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod>,
    #[doc = "Preferred language of the Affirm authorization page that the customer is redirected to."]
    pub preferred_locale: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAmazonPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAmazonPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAmazonPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAmazonPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBacsDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: i64,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
    #[doc = "Specifies payment frequency."]
    pub interval: stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval,
    #[doc = "The number of intervals between payments."]
    pub interval_count: Option<i64>,
    #[doc = "Unique identifier for the mandate or subscription."]
    pub reference: String,
    #[doc = "Start date of the mandate or subscription."]
    pub start_date: i64,
    #[doc = "Specifies the type of mandates supported."]
    pub supported_types: Option<Vec<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("amount_type", &self.amount_type);
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

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCashapp {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCashappCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCashappSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCashapp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCrypto {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCryptoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCrypto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsEps {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsIdeal {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsLink {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsLinkCaptureMethod>,
    #[doc = "Deprecated This is a legacy parameter that no longer has any function."]
    pub persistent_token: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("persistent_token", self.persistent_token.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsNaverPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsNaverPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsNaverPaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsP24 {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsP24SetupFutureUsage>,
    #[doc = "Confirm that the payer has accepted the P24 terms and conditions."]
    pub tos_shown_and_accepted: Option<bool>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("tos_shown_and_accepted", self.tos_shown_and_accepted.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPayco {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaycoCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaycoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPayco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType>,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule>,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPromptpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPromptpaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPromptpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSequra {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSequraCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSequraSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSequra {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType,
    >,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    #[doc = "The method used to collect offline mandate customer acceptance."]
    pub collection_method: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_05::payment_intents::UpdatePaymentIntentParamsShippingAddress,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub carrier: Option<String>,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)."]
    pub phone: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub tracking_number: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
    }
}
