#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLink {
    #[doc = "Whether the payment link's `url` is active."]
    #[json(default)]
    pub active: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<stripe_types_06::payment_links::PaymentLinksResourceAfterCompletion>,
    #[doc = "Whether user redeemable promotion codes are enabled."]
    #[json(default)]
    pub allow_promotion_codes: bool,
    #[doc = "The ID of the Connect application that created the Payment Link."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_07::payment_links::PaymentLinkApplicationExpanded>>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[doc = "This represents the percentage of the subscription invoice total that will be transferred to the application owner's ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::payment_links::PaymentLinksResourceAutomaticTax>,
    #[doc = "Configuration for collecting the customer's billing address."]
    #[json(default)]
    pub billing_address_collection: stripe_types_02::payment_links::PaymentLinkBillingAddressCollection,
    #[doc = "When set, provides configuration to gather active consent from customers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<stripe_types_08::payment_links::PaymentLinksResourceConsentCollection>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Collect additional information from your customer using custom fields."]
    #[json(default)]
    pub custom_fields: Vec<stripe_types_09::payment_links::PaymentLinksResourceCustomFields>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<stripe_types_05::payment_links::PaymentLinksResourceCustomText>,
    #[doc = "Configuration for Customer creation during checkout."]
    #[json(default)]
    pub customer_creation: stripe_types_04::payment_links::PaymentLinkCustomerCreation,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The custom message to be displayed to a customer when a payment link is no longer active."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inactive_message: Option<String>,
    #[doc = "Configuration for creating invoice for payment mode payment links."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<crate::payment_links::PaymentLinksResourceInvoiceCreation>,
    #[doc = "The line items representing what is being sold."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<crate::List<crate::common::Item>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Settings for Managed Payments for this Payment Link and resulting [CheckoutSessions](/api/checkout/sessions/object), ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub managed_payments: Option<stripe_types_00::checkout::PaymentPagesCheckoutSessionManagedPayments>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name_collection: Option<stripe_types_06::payment_links::PaymentLinksResourceNameCollection>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::payment_links::PaymentLinkObject,
    #[doc = "The account on behalf of which to charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<stripe_types_14::account::Account>>,
    #[doc = "The optional items presented to the customer at checkout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub optional_items: Option<Vec<stripe_types_06::payment_links::PaymentLinksResourceOptionalItem>>,
    #[doc = "Indicates the parameters to be passed to PaymentIntent creation during checkout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<stripe_types_06::payment_links::PaymentLinksResourcePaymentIntentData>,
    #[doc = "Configuration for collecting a payment method during checkout."]
    #[json(default)]
    pub payment_method_collection: stripe_types_04::payment_links::PaymentLinkCustomerCreation,
    #[doc = "Payment-method-specific configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<stripe_types_11::payment_links::PaymentLinksResourcePaymentMethodOptions>,
    #[doc = "The list of payment method types that customers can use."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<stripe_types_03::payment_links::PaymentLinkPaymentMethodTypesItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<stripe_types_02::payment_links::PaymentLinksResourcePhoneNumberCollection>,
    #[doc = "Settings that restrict the usage of a payment link."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<stripe_types_06::payment_links::PaymentLinksResourceRestrictions>,
    #[doc = "Configuration for collecting the customer's shipping address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection:
        Option<stripe_types_05::payment_links::PaymentLinksResourceShippingAddressCollection>,
    #[doc = "The shipping rate options applied to the session."]
    #[json(default)]
    pub shipping_options: Vec<stripe_types_12::payment_links::PaymentLinksResourceShippingOption>,
    #[doc = "Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button."]
    #[json(default)]
    pub submit_type: stripe_types_04::payment_links::PaymentLinkSubmitType,
    #[doc = "When creating a subscription, the specified configuration data will be used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<crate::payment_links::PaymentLinksResourceSubscriptionData>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<stripe_types_05::payment_links::PaymentLinksResourceTaxIdCollection>,
    #[doc = "The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<crate::payment_links::PaymentLinksResourceTransferData>,
    #[doc = "The public URL that can be shared with customers."]
    #[json(default)]
    pub url: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl crate::PagedItem for PaymentLink {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceAutomaticTax {
    #[doc = "If `true`, tax will be calculated automatically using the customer's location."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub liability: Option<stripe_types_14::accounts::ConnectAccountReference>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceInvoiceCreation {
    #[doc = "Enable creating an invoice on successful payment."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Configuration for the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<crate::payment_links::PaymentLinksResourceInvoiceSettings>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceInvoiceSettings {
    #[doc = "The account tax IDs associated with the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<
        Vec<crate::Expandable<crate::payment_links::PaymentLinksResourceInvoiceSettingsAccountTaxIdsItemExpanded>>,
    >,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<stripe_types_00::invoices::InvoiceSettingCustomField>>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Footer to be displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<stripe_types_14::accounts::ConnectAccountReference>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Options for invoice PDF rendering."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<stripe_types_00::invoices::InvoiceSettingCheckoutRenderingOptions>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone)]
pub enum PaymentLinksResourceInvoiceSettingsAccountTaxIdsItemExpanded {
    /// Expanded `tax_id`.
    TaxId(Box<stripe_types_14::tax_ids::TaxId>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourceInvoiceSettingsAccountTaxIdsItemExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "tax_id" => PaymentLinksResourceInvoiceSettingsAccountTaxIdsItemExpanded::TaxId(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => PaymentLinksResourceInvoiceSettingsAccountTaxIdsItemExpanded::Other(value),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceSubscriptionData {
    #[doc = "The subscription's description, meant to be displayable to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<crate::payment_links::PaymentLinksResourceSubscriptionDataInvoiceSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on Subscriptions..."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Integer representing the number of trial period days before the customer is charged for the first time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    #[doc = "Settings related to subscription trials."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<stripe_types_07::subscriptions::SubscriptionsTrialsResourceTrialSettings>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<stripe_types_14::accounts::ConnectAccountReference>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceTransferData {
    #[doc = "The amount in cents (or local equivalent) that will be transferred to the destination account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "The connected account receiving the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::Expandable<stripe_types_14::account::Account>>,
}
