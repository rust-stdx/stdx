#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutCustomerBalancePaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<stripe_types_09::checkout::CheckoutCustomerBalanceBankTransferPaymentMethodOptions>,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<stripe_types_03::checkout::CheckoutCustomerBalancePaymentMethodOptionsFundingType>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsBrandingSettings {
    #[doc = "A hex color value starting with `#` representing the background color for the Checkout Session."]
    pub background_color: Option<json::OwnedValue>,
    #[doc = "The border style for the Checkout Session."]
    pub border_style: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsBrandingSettingsBorderStyle>,
    #[doc = "A hex color value starting with `#` representing the button color for the Checkout Session."]
    pub button_color: Option<json::OwnedValue>,
    #[doc = "A string to override the business name shown on the Checkout Session."]
    pub display_name: Option<String>,
    #[doc = "The font family for the Checkout Session corresponding to one of the supported font families..."]
    pub font_family: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsBrandingSettingsFontFamily>,
    #[doc = "The icon for the Checkout Session."]
    pub icon: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsBrandingSettingsIcon>,
    #[doc = "The logo for the Checkout Session."]
    pub logo: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsBrandingSettingsLogo>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBrandingSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("border_style", self.border_style.as_ref());
        form.field_opt("display_name", self.display_name.as_ref());
        form.field_opt("font_family", self.font_family.as_ref());
        form.field_opt("icon", self.icon.as_ref());
        form.field_opt("logo", self.logo.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof:
        Option<stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
    #[doc = "Additional fields for mandate creation."]
    pub mandate_options:
        Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimate {
    #[doc = "The upper bound of the estimated range."]
    pub maximum: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximum>,
    #[doc = "The lower bound of the estimated range."]
    pub minimum: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMinimum>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("maximum", self.maximum.as_ref());
        form.field_opt("minimum", self.minimum.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSubscriptionDataInvoiceSettings {
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsSubscriptionDataInvoiceSettingsIssuer>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("issuer", self.issuer.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsWalletOptions {
    #[doc = "contains details about the Link wallet options (Link is also known as Onelink in the UK)."]
    pub link: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsWalletOptionsLink>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsWalletOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("link", self.link.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckoutSessionParamsLineItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: Option<String>,
    #[doc = "Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline."]
    pub product_data: Option<stripe_types_07::checkout::UpdateCheckoutSessionParamsLineItemsItemPriceDataProductData>,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: Option<stripe_types_07::checkout::UpdateCheckoutSessionParamsLineItemsItemPriceDataRecurring>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_06::checkout::UpdateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior>,
    #[doc = "A non-negative integer in cents (or local equivalent) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsLineItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field_opt("product", self.product.as_ref());
        form.field_opt("product_data", self.product_data.as_ref());
        form.field_opt("recurring", self.recurring.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}
