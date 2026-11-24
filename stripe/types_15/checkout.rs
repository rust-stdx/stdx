#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSession {
    #[doc = "Settings for price localization with [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub adaptive_pricing: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionAdaptivePricing>,
    #[doc = "When set, provides configuration for actions to take if this Checkout Session expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionAfterExpiration>,
    #[doc = "Enables user redeemable promotion codes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    #[doc = "A list of the types of payment methods (e.g., `card`) this Checkout Session can accept."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_payment_method_types: Option<Vec<String>>,
    #[doc = "Total of all items before discounts or taxes are applied."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,
    #[doc = "Total of all items after discounts and taxes are applied."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::checkout::PaymentPagesCheckoutSessionAutomaticTax>,
    #[doc = "Describes whether Checkout should collect the customer's billing address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<stripe_types_04::checkout::CheckoutSessionBillingAddressCollection>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub branding_settings: Option<stripe_types_09::checkout::PaymentPagesCheckoutSessionBrandingSettings>,
    #[doc = "If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment a..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[doc = "A unique string to reference the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[doc = "The client secret of your Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Information about the customer collected within the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collected_information: Option<stripe_types_08::checkout::PaymentPagesCheckoutSessionCollectedInformation>,
    #[doc = "Results of `consent_collection` for this session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub consent: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionConsent>,
    #[doc = "When set, provides configuration for the Checkout Session to gather active consent from customers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<stripe_types_07::checkout::PaymentPagesCheckoutSessionConsentCollection>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Currency conversion details for [Adaptive Pricing](https://docs.stripe.com/payments/checkout/adaptive-pricing) sessio..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency_conversion: Option<stripe_types_02::checkout::PaymentPagesCheckoutSessionCurrencyConversion>,
    #[doc = "Collect additional information from your customer using custom fields."]
    #[json(default)]
    pub custom_fields: Vec<stripe_types_07::checkout::PaymentPagesCheckoutSessionCustomFields>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionCustomText>,
    #[doc = "The ID of the customer for this Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::checkout::CheckoutSessionCustomerExpanded>>,
    #[doc = "The ID of the account for this Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Configure whether a Checkout Session creates a Customer when the Checkout Session completes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<stripe_types_01::checkout::CheckoutSessionCustomerCreation>,
    #[doc = "The customer details including the customer's tax exempt status and the customer's tax IDs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<stripe_types_09::checkout::PaymentPagesCheckoutSessionCustomerDetails>,
    #[doc = "If provided, this value will be used when the Customer object is created."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[doc = "List of coupons and promotion codes attached to the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<crate::checkout::PaymentPagesCheckoutSessionDiscount>>,
    #[doc = "A list of the types of payment methods (e.g., `card`) that should be excluded from this Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub excluded_payment_method_types: Option<Vec<String>>,
    #[doc = "The timestamp at which the Checkout Session will expire."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The integration identifier for this Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub integration_identifier: Option<String>,
    #[doc = "ID of the invoice created by the Checkout Session, if it exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<crate::Expandable<stripe_types_14::invoices::Invoice>>,
    #[doc = "Details on the state of invoice creation for the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<crate::checkout::PaymentPagesCheckoutSessionInvoiceCreation>,
    #[doc = "The line items purchased by the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<crate::List<crate::common::Item>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The IETF language tag of the locale Checkout is displayed in."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<stripe_types_04::checkout::CheckoutSessionLocale>,
    #[doc = "Settings for Managed Payments for this Checkout Session and resulting [PaymentIntents](/api/payment_intents/object), ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub managed_payments: Option<stripe_types_00::checkout::PaymentPagesCheckoutSessionManagedPayments>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The mode of the Checkout Session."]
    #[json(default)]
    pub mode: stripe_types_04::checkout::CheckoutSessionMode,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name_collection: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionNameCollection>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::checkout::CheckoutSessionObject,
    #[doc = "The optional items presented to the customer at checkout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub optional_items: Option<Vec<stripe_types_06::checkout::PaymentPagesCheckoutSessionOptionalItem>>,
    #[doc = "Where the user is coming from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub origin_context: Option<stripe_types_02::checkout::CheckoutSessionOriginContext>,
    #[doc = "The ID of the PaymentIntent for Checkout Sessions in `payment` mode."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<stripe_types_14::payment_intents::PaymentIntent>>,
    #[doc = "The ID of the Payment Link that created this Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<crate::Expandable<crate::payment_links::PaymentLink>>,
    #[doc = "Configure whether a Checkout Session should collect a payment method for sessions with mode `payment`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<stripe_types_01::checkout::CheckoutSessionCustomerCreation>,
    #[doc = "Information about the payment method configuration used for this Checkout session if using dynamic payment methods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details:
        Option<stripe_types_03::payment_methods::PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
    #[doc = "Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<stripe_types_12::checkout::CheckoutSessionPaymentMethodOptions>,
    #[doc = "A list of the types of payment methods (e.g."]
    #[json(default)]
    pub payment_method_types: Vec<String>,
    #[doc = "The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`."]
    #[json(default)]
    pub payment_status: stripe_types_00::checkout::CheckoutSessionPaymentStatus,
    #[doc = "This property is used to set up permissions for various actions (e.g., update) on the CheckoutSession object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<stripe_types_06::checkout::PaymentPagesCheckoutSessionPermissions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionPhoneNumberCollection>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub presentment_details: Option<stripe_types_04::payment_intents::PaymentFlowsPaymentIntentPresentmentDetails>,
    #[doc = "The ID of the original expired Checkout Session that triggered the recovery flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub recovered_from: Option<String>,
    #[doc = "This parameter applies to `ui_mode: embedded_page`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<stripe_types_03::checkout::CheckoutSessionRedirectOnCompletion>,
    #[doc = "Applies to Checkout Sessions with `ui_mode: embedded_page` or `ui_mode: elements`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[doc = "Controls saved payment method settings for the session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub saved_payment_method_options:
        Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionSavedPaymentMethodOptions>,
    #[doc = "The ID of the SetupIntent for Checkout Sessions in `setup` mode."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<crate::Expandable<stripe_types_14::setup_intents::SetupIntent>>,
    #[doc = "When set, provides configuration for Checkout to collect a shipping address from a customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection:
        Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionShippingAddressCollection>,
    #[doc = "The details of the customer cost of shipping, including the customer chosen ShippingRate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<stripe_types_12::checkout::PaymentPagesCheckoutSessionShippingCost>,
    #[doc = "The shipping rate options applied to this Session."]
    #[json(default)]
    pub shipping_options: Vec<stripe_types_12::checkout::PaymentPagesCheckoutSessionShippingOption>,
    #[doc = "The status of the Checkout Session, one of `open`, `complete`, or `expired`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_02::checkout::CheckoutSessionStatus>,
    #[doc = "Describes the type of transaction being performed by Checkout in order to customize relevant text on the page, such a..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<stripe_types_01::checkout::CheckoutSessionSubmitType>,
    #[doc = "The ID of the [Subscription](https://docs.stripe.com/api/subscriptions) for Checkout Sessions in `subscription` mode."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<crate::Expandable<stripe_types_14::subscriptions::Subscription>>,
    #[doc = "The URL the customer will be directed to after the payment or subscription creation is successful."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionTaxIdCollection>,
    #[doc = "Tax and discount details for the computed total amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_details: Option<crate::checkout::PaymentPagesCheckoutSessionTotalDetails>,
    #[doc = "The UI mode of the Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<stripe_types_02::checkout::CheckoutSessionUiMode>,
    #[doc = "The URL to the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Wallet-specific configuration for this Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet_options: Option<stripe_types_07::checkout::CheckoutSessionWalletOptions>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl crate::PagedItem for CheckoutSession {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone)]
pub enum CheckoutSessionCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<stripe_types_14::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                CheckoutSessionCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => CheckoutSessionCustomerExpanded::Other(value),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionAutomaticTax {
    #[doc = "Indicates whether automatic tax is enabled for the session"]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub liability: Option<stripe_types_14::accounts::ConnectAccountReference>,
    #[doc = "The tax provider powering automatic tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The status of the most recent automated tax calculation for this session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_04::checkout::PaymentPagesCheckoutSessionAutomaticTaxStatus>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionDiscount {
    #[doc = "Coupon attached to the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<crate::Expandable<stripe_types_06::coupons::Coupon>>,
    #[doc = "Promotion code attached to the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<crate::Expandable<stripe_types_14::promotion_codes::PromotionCode>>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionInvoiceCreation {
    #[doc = "Indicates whether invoice creation is enabled for the Checkout Session."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<crate::checkout::PaymentPagesCheckoutSessionInvoiceSettings>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionInvoiceSettings {
    #[doc = "The account tax IDs associated with the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<
        Vec<crate::Expandable<crate::checkout::PaymentPagesCheckoutSessionInvoiceSettingsAccountTaxIdsItemExpanded>>,
    >,
    #[doc = "Custom fields displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<stripe_types_00::invoices::InvoiceSettingCustomField>>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Footer displayed on the invoice."]
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
    feature = "events"
))]
#[derive(Debug, Clone)]
pub enum PaymentPagesCheckoutSessionInvoiceSettingsAccountTaxIdsItemExpanded {
    /// Expanded `tax_id`.
    TaxId(Box<stripe_types_14::tax_ids::TaxId>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionInvoiceSettingsAccountTaxIdsItemExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "tax_id" => PaymentPagesCheckoutSessionInvoiceSettingsAccountTaxIdsItemExpanded::TaxId(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => PaymentPagesCheckoutSessionInvoiceSettingsAccountTaxIdsItemExpanded::Other(value),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionTotalDetails {
    #[doc = "This is the sum of all the discounts."]
    #[json(default)]
    pub amount_discount: i64,
    #[doc = "This is the sum of all the shipping amounts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<i64>,
    #[doc = "This is the sum of all the tax amounts."]
    #[json(default)]
    pub amount_tax: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<crate::checkout::PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    #[doc = "The aggregated discounts."]
    #[json(default)]
    pub discounts: Vec<crate::common::LineItemsDiscountAmount>,
    #[doc = "The aggregated tax amounts by rate."]
    #[json(default)]
    pub taxes: Vec<stripe_types_07::common::LineItemsTaxAmount>,
}
