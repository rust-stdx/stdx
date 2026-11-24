#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParams {
    #[doc = "Behavior after the purchase is complete."]
    pub after_completion: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsAfterCompletion>,
    #[doc = "Enables user redeemable promotion codes."]
    pub allow_promotion_codes: Option<bool>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "Configuration for automatic tax collection."]
    pub automatic_tax: Option<stripe_types_07::payment_links::CreatePaymentLinkParamsAutomaticTax>,
    #[doc = "Configuration for collecting the customer's billing address."]
    pub billing_address_collection:
        Option<stripe_types_00::payment_links::CreatePaymentLinkParamsBillingAddressCollection>,
    #[doc = "Configure fields to gather active consent from customers."]
    pub consent_collection: Option<stripe_types_08::payment_links::CreatePaymentLinkParamsConsentCollection>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Collect additional information from your customer using custom fields."]
    pub custom_fields: Option<Vec<stripe_types_08::payment_links::CreatePaymentLinkParamsCustomFieldsItem>>,
    #[doc = "Display additional text for your customers using custom text."]
    pub custom_text: Option<stripe_types_07::payment_links::CreatePaymentLinkParamsCustomText>,
    #[doc = "Configures whether [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link cr..."]
    pub customer_creation: Option<stripe_types_02::payment_links::CreatePaymentLinkParamsCustomerCreation>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The custom message to be displayed to a customer when a payment link is no longer active."]
    pub inactive_message: Option<String>,
    #[doc = "Generate a post-purchase Invoice for one-time payments."]
    pub invoice_creation: Option<stripe_types_12::payment_links::CreatePaymentLinkParamsInvoiceCreation>,
    #[doc = "The line items representing what is being sold."]
    pub line_items: Vec<stripe_types_11::payment_links::CreatePaymentLinkParamsLineItemsItem>,
    #[doc = "Settings for Managed Payments for this Payment Link and resulting [CheckoutSessions](/api/checkout/sessions/object), ..."]
    pub managed_payments: Option<stripe_types_01::payment_links::CreatePaymentLinkParamsManagedPayments>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Controls settings applied for collecting the customer's name."]
    pub name_collection: Option<stripe_types_07::payment_links::CreatePaymentLinkParamsNameCollection>,
    #[doc = "The account on behalf of which to charge."]
    pub on_behalf_of: Option<String>,
    #[doc = "A list of optional items the customer can add to their order at checkout."]
    pub optional_items: Option<Vec<stripe_types_09::payment_links::CreatePaymentLinkParamsOptionalItemsItem>>,
    #[doc = "A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode."]
    pub payment_intent_data: Option<stripe_types_05::payment_links::CreatePaymentLinkParamsPaymentIntentData>,
    #[doc = "Specify whether Checkout should collect a payment method."]
    pub payment_method_collection:
        Option<stripe_types_05::payment_links::CreatePaymentLinkParamsPaymentMethodCollection>,
    #[doc = "Payment-method-specific configuration."]
    pub payment_method_options: Option<stripe_types_11::payment_links::CreatePaymentLinkParamsPaymentMethodOptions>,
    #[doc = "The list of payment method types that customers can use."]
    pub payment_method_types:
        Option<Vec<stripe_types_04::payment_links::CreatePaymentLinkParamsPaymentMethodTypesItem>>,
    #[doc = "Controls phone number collection settings during checkout."]
    pub phone_number_collection: Option<stripe_types_04::payment_links::CreatePaymentLinkParamsPhoneNumberCollection>,
    #[doc = "Settings that restrict the usage of a payment link."]
    pub restrictions: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsRestrictions>,
    #[doc = "Configuration for collecting the customer's shipping address."]
    pub shipping_address_collection:
        Option<stripe_types_05::payment_links::CreatePaymentLinkParamsShippingAddressCollection>,
    #[doc = "The shipping rate options to apply to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by t..."]
    pub shipping_options: Option<Vec<stripe_types_01::payment_links::CreatePaymentLinkParamsShippingOptionsItem>>,
    #[doc = "Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit..."]
    pub submit_type: Option<stripe_types_03::payment_links::CreatePaymentLinkParamsSubmitType>,
    #[doc = "When creating a subscription, the specified configuration data will be used."]
    pub subscription_data: Option<stripe_types_12::payment_links::CreatePaymentLinkParamsSubscriptionData>,
    #[doc = "Controls tax ID collection during checkout."]
    pub tax_id_collection: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsTaxIdCollection>,
    #[doc = "The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be ..."]
    pub transfer_data: Option<stripe_types_00::payment_links::CreatePaymentLinkParamsTransferData>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("after_completion", self.after_completion.as_ref());
        form.field_opt("allow_promotion_codes", self.allow_promotion_codes.as_ref());
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        form.field_opt("application_fee_percent", self.application_fee_percent.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_address_collection", self.billing_address_collection.as_ref());
        form.field_opt("consent_collection", self.consent_collection.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.custom_fields.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("custom_fields", index, item);
            }
        }
        form.field_opt("custom_text", self.custom_text.as_ref());
        form.field_opt("customer_creation", self.customer_creation.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("inactive_message", self.inactive_message.as_ref());
        form.field_opt("invoice_creation", self.invoice_creation.as_ref());
        for (index, item) in self.line_items.iter().enumerate() {
            form.field_indexed("line_items", index, item);
        }
        form.field_opt("managed_payments", self.managed_payments.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name_collection", self.name_collection.as_ref());
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        if let Some(values) = self.optional_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("optional_items", index, item);
            }
        }
        form.field_opt("payment_intent_data", self.payment_intent_data.as_ref());
        form.field_opt("payment_method_collection", self.payment_method_collection.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        if let Some(values) = self.payment_method_types.as_ref() {
            form.field_array("payment_method_types", values.as_slice());
        }
        form.field_opt("phone_number_collection", self.phone_number_collection.as_ref());
        form.field_opt("restrictions", self.restrictions.as_ref());
        form.field_opt("shipping_address_collection", self.shipping_address_collection.as_ref());
        if let Some(values) = self.shipping_options.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("shipping_options", index, item);
            }
        }
        form.field_opt("submit_type", self.submit_type.as_ref());
        form.field_opt("subscription_data", self.subscription_data.as_ref());
        form.field_opt("tax_id_collection", self.tax_id_collection.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParams {
    #[doc = "Whether the payment link's `url` is active."]
    pub active: Option<bool>,
    #[doc = "Behavior after the purchase is complete."]
    pub after_completion: Option<stripe_types_07::payment_links::UpdatePaymentLinkParamsAfterCompletion>,
    #[doc = "Enables user redeemable promotion codes."]
    pub allow_promotion_codes: Option<bool>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<json::OwnedValue>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<json::OwnedValue>,
    #[doc = "Configuration for automatic tax collection."]
    pub automatic_tax: Option<stripe_types_10::payment_links::UpdatePaymentLinkParamsAutomaticTax>,
    #[doc = "Configuration for collecting the customer's billing address."]
    pub billing_address_collection:
        Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsBillingAddressCollection>,
    #[doc = "Configure fields to gather active consent from customers."]
    pub consent_collection: Option<stripe_types_10::payment_links::UpdatePaymentLinkParamsConsentCollection>,
    #[doc = "Collect additional information from your customer using custom fields."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "Display additional text for your customers using custom text."]
    pub custom_text: Option<stripe_types_07::payment_links::UpdatePaymentLinkParamsCustomText>,
    #[doc = "Configures whether [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by this payment link cr..."]
    pub customer_creation: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsCustomerCreation>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The custom message to be displayed to a customer when a payment link is no longer active."]
    pub inactive_message: Option<json::OwnedValue>,
    #[doc = "Generate a post-purchase Invoice for one-time payments."]
    pub invoice_creation: Option<stripe_types_12::payment_links::UpdatePaymentLinkParamsInvoiceCreation>,
    #[doc = "The line items representing what is being sold."]
    pub line_items: Option<Vec<stripe_types_07::payment_links::UpdatePaymentLinkParamsLineItemsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Controls settings applied for collecting the customer's name."]
    pub name_collection: Option<stripe_types_08::payment_links::UpdatePaymentLinkParamsNameCollection>,
    #[doc = "The account on behalf of which to charge."]
    pub on_behalf_of: Option<json::OwnedValue>,
    #[doc = "A list of optional items the customer can add to their order at checkout."]
    pub optional_items: Option<json::OwnedValue>,
    #[doc = "A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode."]
    pub payment_intent_data: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsPaymentIntentData>,
    #[doc = "Specify whether Checkout should collect a payment method."]
    pub payment_method_collection:
        Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsPaymentMethodCollection>,
    #[doc = "Payment-method-specific configuration."]
    pub payment_method_options: Option<stripe_types_08::payment_links::UpdatePaymentLinkParamsPaymentMethodOptions>,
    #[doc = "The list of payment method types that customers can use."]
    pub payment_method_types: Option<json::OwnedValue>,
    #[doc = "Controls phone number collection settings during checkout."]
    pub phone_number_collection: Option<stripe_types_05::payment_links::UpdatePaymentLinkParamsPhoneNumberCollection>,
    #[doc = "Settings that restrict the usage of a payment link."]
    pub restrictions: Option<stripe_types_09::payment_links::UpdatePaymentLinkParamsRestrictions>,
    #[doc = "Configuration for collecting the customer's shipping address."]
    pub shipping_address_collection:
        Option<stripe_types_09::payment_links::UpdatePaymentLinkParamsShippingAddressCollection>,
    #[doc = "The shipping rate options to apply to [checkout sessions](https://docs.stripe.com/api/checkout/sessions) created by t..."]
    pub shipping_options: Option<json::OwnedValue>,
    #[doc = "Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit..."]
    pub submit_type: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsSubmitType>,
    #[doc = "When creating a subscription, the specified configuration data will be used."]
    pub subscription_data: Option<stripe_types_12::payment_links::UpdatePaymentLinkParamsSubscriptionData>,
    #[doc = "Controls tax ID collection during checkout."]
    pub tax_id_collection: Option<stripe_types_09::payment_links::UpdatePaymentLinkParamsTaxIdCollection>,
    #[doc = "The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be ..."]
    pub transfer_data: Option<stripe_types_00::payment_links::UpdatePaymentLinkParamsTransferData>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("after_completion", self.after_completion.as_ref());
        form.field_opt("allow_promotion_codes", self.allow_promotion_codes.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_address_collection", self.billing_address_collection.as_ref());
        form.field_opt("consent_collection", self.consent_collection.as_ref());
        form.field_opt("custom_text", self.custom_text.as_ref());
        form.field_opt("customer_creation", self.customer_creation.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice_creation", self.invoice_creation.as_ref());
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
        form.field_opt("name_collection", self.name_collection.as_ref());
        form.field_opt("payment_intent_data", self.payment_intent_data.as_ref());
        form.field_opt("payment_method_collection", self.payment_method_collection.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("phone_number_collection", self.phone_number_collection.as_ref());
        form.field_opt("restrictions", self.restrictions.as_ref());
        form.field_opt("shipping_address_collection", self.shipping_address_collection.as_ref());
        form.field_opt("submit_type", self.submit_type.as_ref());
        form.field_opt("subscription_data", self.subscription_data.as_ref());
        form.field_opt("tax_id_collection", self.tax_id_collection.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}
