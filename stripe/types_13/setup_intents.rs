#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParams {
    #[doc = "The list of payment method types to allow for this SetupIntent."]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_06::setup_intents::ConfirmSetupIntentParamsAllowedPaymentMethodTypesItem>>,
    #[doc = "ID of the ConfirmationToken used to confirm this SetupIntent."]
    pub confirmation_token: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    pub mandate_data: Option<json::OwnedValue>,
    #[doc = "ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent."]
    pub payment_method: Option<String>,
    #[doc = "When included, this hash creates a PaymentMethod that is set as the `payment_method`..."]
    pub payment_method_data: Option<stripe_types_12::setup_intents::ConfirmSetupIntentParamsPaymentMethodData>,
    #[doc = "Payment method-specific configuration for this SetupIntent."]
    pub payment_method_options: Option<crate::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptions>,
    #[doc = "The URL to redirect your customer back to after they authenticate on the payment method's app or site."]
    pub return_url: Option<String>,
    #[doc = "Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next ac..."]
    pub use_stripe_sdk: Option<bool>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_payment_method_types.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allowed_payment_method_types", index, item);
            }
        }
        form.field_opt("confirmation_token", self.confirmation_token.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("use_stripe_sdk", self.use_stripe_sdk.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptions {
    #[doc = "If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_11::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options."]
    pub amazon_pay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options."]
    pub bacs_debit: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "If this is a `bizum` SetupIntent, this sub-hash contains details about the Bizum payment method options."]
    pub bizum: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsBlik>,
    #[doc = "Configuration for any card setup attempted on this SetupIntent."]
    pub card: Option<crate::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCard>,
    #[doc = "If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options."]
    pub card_present: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardPresent>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_10::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsKlarna>,
    #[doc = "If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also..."]
    pub link: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsLink>,
    #[doc = "If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPaypal>,
    #[doc = "If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_11::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPayto>,
    #[doc = "If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_10::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsPix>,
    #[doc = "If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options."]
    pub sepa_debit: Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_10::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUpi>,
    #[doc = "If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method o..."]
    pub us_bank_account:
        Option<stripe_types_12::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsCard {
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options:
        Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "When specified, this parameter signals that a card has been collected as MOTO (Mail Order Telephone Order) and thus o..."]
    pub moto: Option<bool>,
    #[doc = "Selected network to process this SetupIntent on."]
    pub network: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardNetwork>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "If 3D Secure authentication was performed with a third-party provider, the authentication details to use for this setup."]
    pub three_d_secure:
        Option<stripe_types_12::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecure>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("moto", self.moto.as_ref());
        form.field_opt("network", self.network.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
        form.field_opt("three_d_secure", self.three_d_secure.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParams {
    #[doc = "The list of payment method types to allow for this SetupIntent."]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_00::setup_intents::CreateSetupIntentParamsAllowedPaymentMethodTypesItem>>,
    #[doc = "If present, the SetupIntent's payment method will be attached to the in-context Stripe Account."]
    pub attach_to_self: Option<bool>,
    #[doc = "When you enable this parameter, this SetupIntent accepts payment methods that you enable in the Dashboard and that ar..."]
    pub automatic_payment_methods:
        Option<stripe_types_05::setup_intents::CreateSetupIntentParamsAutomaticPaymentMethods>,
    #[doc = "Set to `true` to attempt to confirm this SetupIntent immediately."]
    pub confirm: Option<bool>,
    #[doc = "ID of the ConfirmationToken used to confirm this SetupIntent."]
    pub confirmation_token: Option<String>,
    #[doc = "ID of the Customer this SetupIntent belongs to, if one exists."]
    pub customer: Option<String>,
    #[doc = "ID of the Account this SetupIntent belongs to, if one exists."]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The list of payment method types to exclude from use with this SetupIntent."]
    pub excluded_payment_method_types:
        Option<Vec<stripe_types_03::setup_intents::CreateSetupIntentParamsExcludedPaymentMethodTypesItem>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Indicates the directions of money movement for which this payment method is intended to be used."]
    pub flow_directions: Option<Vec<stripe_types_02::setup_intents::CreateSetupIntentParamsFlowDirectionsItem>>,
    #[doc = "This hash contains details about the mandate to create."]
    pub mandate_data: Option<stripe_types_07::setup_intents::CreateSetupIntentParamsMandateData>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The Stripe account ID created for this SetupIntent."]
    pub on_behalf_of: Option<String>,
    #[doc = "ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent."]
    pub payment_method: Option<String>,
    #[doc = "The ID of the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) to use with t..."]
    pub payment_method_configuration: Option<String>,
    #[doc = "When included, this hash creates a PaymentMethod that is set as the `payment_method`..."]
    pub payment_method_data: Option<stripe_types_10::setup_intents::CreateSetupIntentParamsPaymentMethodData>,
    #[doc = "Payment method-specific configuration for this SetupIntent."]
    pub payment_method_options: Option<crate::setup_intents::CreateSetupIntentParamsPaymentMethodOptions>,
    #[doc = "The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app..."]
    pub return_url: Option<String>,
    #[doc = "If you populate this hash, this SetupIntent generates a `single_use` mandate after successful completion."]
    pub single_use: Option<stripe_types_03::setup_intents::CreateSetupIntentParamsSingleUse>,
    #[doc = "Indicates how the payment method is intended to be used in the future."]
    pub usage: Option<stripe_types_04::setup_intents::CreateSetupIntentParamsUsage>,
    #[doc = "Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next ac..."]
    pub use_stripe_sdk: Option<bool>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_payment_method_types.as_ref() {
            form.field_array("allowed_payment_method_types", values.as_slice());
        }
        form.field_opt("attach_to_self", self.attach_to_self.as_ref());
        form.field_opt("automatic_payment_methods", self.automatic_payment_methods.as_ref());
        form.field_opt("confirm", self.confirm.as_ref());
        form.field_opt("confirmation_token", self.confirmation_token.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.excluded_payment_method_types.as_ref() {
            form.field_array("excluded_payment_method_types", values.as_slice());
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.flow_directions.as_ref() {
            form.field_array("flow_directions", values.as_slice());
        }
        form.field_opt("mandate_data", self.mandate_data.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_configuration", self.payment_method_configuration.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("single_use", self.single_use.as_ref());
        form.field_opt("usage", self.usage.as_ref());
        form.field_opt("use_stripe_sdk", self.use_stripe_sdk.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptions {
    #[doc = "If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_07::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options."]
    pub amazon_pay: Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options."]
    pub bacs_debit: Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "If this is a `bizum` SetupIntent, this sub-hash contains details about the Bizum payment method options."]
    pub bizum: Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBlik>,
    #[doc = "Configuration for any card setup attempted on this SetupIntent."]
    pub card: Option<stripe_types_12::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCard>,
    #[doc = "If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options."]
    pub card_present: Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardPresent>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_09::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsKlarna>,
    #[doc = "If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also..."]
    pub link: Option<stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsLink>,
    #[doc = "If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaypal>,
    #[doc = "If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_09::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPayto>,
    #[doc = "If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_11::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPix>,
    #[doc = "If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options."]
    pub sepa_debit: Option<stripe_types_09::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_10::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUpi>,
    #[doc = "If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method o..."]
    pub us_bank_account:
        Option<stripe_types_12::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParams {
    #[doc = "The list of payment method types to allow for this SetupIntent."]
    pub allowed_payment_method_types: Option<json::OwnedValue>,
    #[doc = "If present, the SetupIntent's payment method will be attached to the in-context Stripe Account."]
    pub attach_to_self: Option<bool>,
    #[doc = "ID of the Customer this SetupIntent belongs to, if one exists."]
    pub customer: Option<String>,
    #[doc = "ID of the Account this SetupIntent belongs to, if one exists."]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The list of payment method types to exclude from use with this SetupIntent."]
    pub excluded_payment_method_types: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Indicates the directions of money movement for which this payment method is intended to be used."]
    pub flow_directions: Option<Vec<stripe_types_05::setup_intents::UpdateSetupIntentParamsFlowDirectionsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "ID of the payment method (a PaymentMethod, Card, or saved Source object) to attach to this SetupIntent."]
    pub payment_method: Option<String>,
    #[doc = "The ID of the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) to use with t..."]
    pub payment_method_configuration: Option<String>,
    #[doc = "When included, this hash creates a PaymentMethod that is set as the `payment_method`..."]
    pub payment_method_data: Option<stripe_types_12::setup_intents::UpdateSetupIntentParamsPaymentMethodData>,
    #[doc = "Payment method-specific configuration for this SetupIntent."]
    pub payment_method_options: Option<crate::setup_intents::UpdateSetupIntentParamsPaymentMethodOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("attach_to_self", self.attach_to_self.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.flow_directions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("flow_directions", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_configuration", self.payment_method_configuration.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptions {
    #[doc = "If this is a `acss_debit` SetupIntent, this sub-hash contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_11::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "If this is a `amazon_pay` SetupIntent, this sub-hash contains details about the AmazonPay payment method options."]
    pub amazon_pay: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "If this is a `bacs_debit` SetupIntent, this sub-hash contains details about the Bacs Debit payment method options."]
    pub bacs_debit: Option<stripe_types_07::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "If this is a `bizum` SetupIntent, this sub-hash contains details about the Bizum payment method options."]
    pub bizum: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsBlik>,
    #[doc = "Configuration for any card setup attempted on this SetupIntent."]
    pub card: Option<crate::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCard>,
    #[doc = "If this is a `card_present` PaymentMethod, this sub-hash contains details about the card-present payment method options."]
    pub card_present: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardPresent>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_11::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsKlarna>,
    #[doc = "If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also..."]
    pub link: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsLink>,
    #[doc = "If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPaypal>,
    #[doc = "If this is a `payto` SetupIntent, this sub-hash contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_10::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPayto>,
    #[doc = "If this is a `pix` SetupIntent, this sub-hash contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_11::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsPix>,
    #[doc = "If this is a `sepa_debit` SetupIntent, this sub-hash contains details about the SEPA Debit payment method options."]
    pub sepa_debit: Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "If this is a `upi` SetupIntent, this sub-hash contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_11::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUpi>,
    #[doc = "If this is a `us_bank_account` SetupIntent, this sub-hash contains details about the US bank account payment method o..."]
    pub us_bank_account:
        Option<stripe_types_12::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsCard {
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options:
        Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "When specified, this parameter signals that a card has been collected as MOTO (Mail Order Telephone Order) and thus o..."]
    pub moto: Option<bool>,
    #[doc = "Selected network to process this SetupIntent on."]
    pub network: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardNetwork>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure:
        Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "If 3D Secure authentication was performed with a third-party provider, the authentication details to use for this setup."]
    pub three_d_secure:
        Option<stripe_types_12::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecure>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("moto", self.moto.as_ref());
        form.field_opt("network", self.network.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
        form.field_opt("three_d_secure", self.three_d_secure.as_ref());
    }
}
