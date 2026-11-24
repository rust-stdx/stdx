#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParams {
    #[doc = "Settings for price localization with [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing)."]
    pub adaptive_pricing: Option<stripe_types_00::checkout::CreateCheckoutSessionParamsAdaptivePricing>,
    #[doc = "Configure actions after a Checkout Session has expired."]
    pub after_expiration: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsAfterExpiration>,
    #[doc = "Enables user redeemable promotion codes."]
    pub allow_promotion_codes: Option<bool>,
    #[doc = "A list of the types of payment methods (e.g., `card`) this Checkout Session can accept."]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_02::checkout::CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem>>,
    #[doc = "Settings for automatic tax lookup for this session and resulting payments, invoices, and subscriptions."]
    pub automatic_tax: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsAutomaticTax>,
    #[doc = "Specify whether Checkout should collect the customer's billing address."]
    pub billing_address_collection:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsBillingAddressCollection>,
    #[doc = "The branding settings for the Checkout Session."]
    pub branding_settings: Option<stripe_types_11::checkout::CreateCheckoutSessionParamsBrandingSettings>,
    #[doc = "If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment a..."]
    pub cancel_url: Option<String>,
    #[doc = "A unique string to reference the Checkout Session."]
    pub client_reference_id: Option<String>,
    #[doc = "Configure fields for the Checkout Session to gather active consent from customers."]
    pub consent_collection: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsConsentCollection>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Collect additional information from your customer using custom fields."]
    pub custom_fields: Option<Vec<stripe_types_07::checkout::CreateCheckoutSessionParamsCustomFieldsItem>>,
    #[doc = "Display additional text for your customers using custom text."]
    pub custom_text: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsCustomText>,
    #[doc = "ID of an existing Customer, if one exists."]
    pub customer: Option<String>,
    #[doc = "ID of an existing Account, if one exists."]
    pub customer_account: Option<String>,
    #[doc = "Configure whether a Checkout Session creates a [Customer](https://docs.stripe.com/api/customers) during Session confi..."]
    pub customer_creation: Option<stripe_types_02::checkout::CreateCheckoutSessionParamsCustomerCreation>,
    #[doc = "If provided, this value will be used when the Customer object is created."]
    pub customer_email: Option<String>,
    #[doc = "Controls what fields on Customer can be updated by the Checkout Session."]
    pub customer_update: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsCustomerUpdate>,
    #[doc = "The coupon or promotion code to apply to this Session."]
    pub discounts: Option<Vec<stripe_types_01::checkout::CreateCheckoutSessionParamsDiscountsItem>>,
    #[doc = "A list of the types of payment methods (e.g., `card`) that should be excluded from this Checkout Session."]
    pub excluded_payment_method_types:
        Option<Vec<stripe_types_04::checkout::CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The Epoch time in seconds at which the Checkout Session will expire."]
    pub expires_at: Option<i64>,
    #[doc = "The integration identifier for this Checkout Session."]
    pub integration_identifier: Option<String>,
    #[doc = "Generate a post-purchase Invoice for one-time payments."]
    pub invoice_creation: Option<stripe_types_12::checkout::CreateCheckoutSessionParamsInvoiceCreation>,
    #[doc = "A list of items the customer is purchasing."]
    pub line_items: Option<Vec<stripe_types_10::checkout::CreateCheckoutSessionParamsLineItemsItem>>,
    #[doc = "The IETF language tag of the locale Checkout is displayed in."]
    pub locale: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsLocale>,
    #[doc = "Settings for Managed Payments for this Checkout Session and resulting [PaymentIntents](/api/payment_intents/object), ..."]
    pub managed_payments: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsManagedPayments>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The mode of the Checkout Session."]
    pub mode: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsMode>,
    #[doc = "Controls name collection settings for the session."]
    pub name_collection: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsNameCollection>,
    #[doc = "A list of optional items the customer can add to their order at checkout."]
    pub optional_items: Option<Vec<stripe_types_08::checkout::CreateCheckoutSessionParamsOptionalItemsItem>>,
    #[doc = "Where the user is coming from."]
    pub origin_context: Option<stripe_types_02::checkout::CreateCheckoutSessionParamsOriginContext>,
    #[doc = "A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode."]
    pub payment_intent_data: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentIntentData>,
    #[doc = "Specify whether Checkout should collect a payment method."]
    pub payment_method_collection:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodCollection>,
    #[doc = "The ID of the payment method configuration to use with this Checkout session."]
    pub payment_method_configuration: Option<String>,
    #[doc = "This parameter allows you to set some attributes on the payment method created during a Checkout session."]
    pub payment_method_data: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodData>,
    #[doc = "Payment-method-specific configuration."]
    pub payment_method_options: Option<stripe_types_12::checkout::CreateCheckoutSessionParamsPaymentMethodOptions>,
    #[doc = "This property is used to set up permissions for various actions (for example, update) on the CheckoutSession object."]
    pub permissions: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPermissions>,
    #[doc = "Controls phone number collection settings for the session."]
    pub phone_number_collection: Option<stripe_types_01::checkout::CreateCheckoutSessionParamsPhoneNumberCollection>,
    #[doc = "This parameter applies to `ui_mode: embedded_page`."]
    pub redirect_on_completion: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsRedirectOnCompletion>,
    #[doc = "The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app..."]
    pub return_url: Option<String>,
    #[doc = "Controls saved payment method settings for the session."]
    pub saved_payment_method_options:
        Option<stripe_types_09::checkout::CreateCheckoutSessionParamsSavedPaymentMethodOptions>,
    #[doc = "A subset of parameters to be passed to SetupIntent creation for Checkout Sessions in `setup` mode."]
    pub setup_intent_data: Option<stripe_types_01::checkout::CreateCheckoutSessionParamsSetupIntentData>,
    #[doc = "When set, provides configuration for Checkout to collect a shipping address from a customer."]
    pub shipping_address_collection:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsShippingAddressCollection>,
    #[doc = "The shipping rate options to apply to this Session."]
    pub shipping_options: Option<Vec<crate::checkout::CreateCheckoutSessionParamsShippingOptionsItem>>,
    #[doc = "Describes the type of transaction being performed by Checkout in order to customize relevant text on the page, such a..."]
    pub submit_type: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsSubmitType>,
    #[doc = "A subset of parameters to be passed to subscription creation for Checkout Sessions in `subscription` mode."]
    pub subscription_data: Option<stripe_types_12::checkout::CreateCheckoutSessionParamsSubscriptionData>,
    #[doc = "The URL to which Stripe should send customers when payment or setup is complete."]
    pub success_url: Option<String>,
    #[doc = "Controls tax ID collection during checkout."]
    pub tax_id_collection: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsTaxIdCollection>,
    #[doc = "The UI mode of the Session."]
    pub ui_mode: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsUiMode>,
    #[doc = "Wallet-specific configuration."]
    pub wallet_options: Option<stripe_types_11::checkout::CreateCheckoutSessionParamsWalletOptions>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adaptive_pricing", self.adaptive_pricing.as_ref());
        form.field_opt("after_expiration", self.after_expiration.as_ref());
        form.field_opt("allow_promotion_codes", self.allow_promotion_codes.as_ref());
        if let Some(values) = self.allowed_payment_method_types.as_ref() {
            form.field_array("allowed_payment_method_types", values.as_slice());
        }
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_address_collection", self.billing_address_collection.as_ref());
        form.field_opt("branding_settings", self.branding_settings.as_ref());
        form.field_opt("cancel_url", self.cancel_url.as_ref());
        form.field_opt("client_reference_id", self.client_reference_id.as_ref());
        form.field_opt("consent_collection", self.consent_collection.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.custom_fields.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("custom_fields", index, item);
            }
        }
        form.field_opt("custom_text", self.custom_text.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("customer_creation", self.customer_creation.as_ref());
        form.field_opt("customer_email", self.customer_email.as_ref());
        form.field_opt("customer_update", self.customer_update.as_ref());
        if let Some(values) = self.discounts.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("discounts", index, item);
            }
        }
        if let Some(values) = self.excluded_payment_method_types.as_ref() {
            form.field_array("excluded_payment_method_types", values.as_slice());
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field_opt("integration_identifier", self.integration_identifier.as_ref());
        form.field_opt("invoice_creation", self.invoice_creation.as_ref());
        if let Some(values) = self.line_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("line_items", index, item);
            }
        }
        form.field_opt("locale", self.locale.as_ref());
        form.field_opt("managed_payments", self.managed_payments.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("mode", self.mode.as_ref());
        form.field_opt("name_collection", self.name_collection.as_ref());
        if let Some(values) = self.optional_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("optional_items", index, item);
            }
        }
        form.field_opt("origin_context", self.origin_context.as_ref());
        form.field_opt("payment_intent_data", self.payment_intent_data.as_ref());
        form.field_opt("payment_method_collection", self.payment_method_collection.as_ref());
        form.field_opt("payment_method_configuration", self.payment_method_configuration.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("permissions", self.permissions.as_ref());
        form.field_opt("phone_number_collection", self.phone_number_collection.as_ref());
        form.field_opt("redirect_on_completion", self.redirect_on_completion.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("saved_payment_method_options", self.saved_payment_method_options.as_ref());
        form.field_opt("setup_intent_data", self.setup_intent_data.as_ref());
        form.field_opt("shipping_address_collection", self.shipping_address_collection.as_ref());
        if let Some(values) = self.shipping_options.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("shipping_options", index, item);
            }
        }
        form.field_opt("submit_type", self.submit_type.as_ref());
        form.field_opt("subscription_data", self.subscription_data.as_ref());
        form.field_opt("success_url", self.success_url.as_ref());
        form.field_opt("tax_id_collection", self.tax_id_collection.as_ref());
        form.field_opt("ui_mode", self.ui_mode.as_ref());
        form.field_opt("wallet_options", self.wallet_options.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsShippingOptionsItem {
    #[doc = "The ID of the Shipping Rate to use for this shipping option."]
    pub shipping_rate: Option<String>,
    #[doc = "Parameters to be passed to Shipping Rate creation for this shipping option."]
    pub shipping_rate_data:
        Option<stripe_types_12::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateData>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingOptionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("shipping_rate", self.shipping_rate.as_ref());
        form.field_opt("shipping_rate_data", self.shipping_rate_data.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckoutSessionParams {
    #[doc = "Information about the customer collected within the Checkout Session."]
    pub collected_information: Option<stripe_types_08::checkout::UpdateCheckoutSessionParamsCollectedInformation>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A list of items the customer is purchasing."]
    pub line_items: Option<Vec<stripe_types_12::checkout::UpdateCheckoutSessionParamsLineItemsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The shipping rate options to apply to this Session."]
    pub shipping_options: Option<json::OwnedValue>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collected_information", self.collected_information.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.line_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("line_items", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
