#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutUsBankAccountPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<stripe_types_09::checkout::CheckoutFinancialConnectionsPaymentMethodOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<stripe_types_01::checkout::CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsInvoiceCreationInvoiceData {
    #[doc = "The account tax IDs associated with the invoice."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "Default custom fields to be displayed on invoices for this customer."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Default footer to be displayed on invoices for this customer."]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsInvoiceCreationInvoiceDataIssuer>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Default options for invoice PDF rendering for this customer."]
    pub rendering_options:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptions>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsInvoiceCreationInvoiceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("issuer", self.issuer.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("rendering_options", self.rendering_options.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsLineItemsItem {
    #[doc = "When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout."]
    pub adjustable_quantity:
        Option<stripe_types_04::checkout::CreateCheckoutSessionParamsLineItemsItemAdjustableQuantity>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsLineItemsItemPriceData>,
    #[doc = "The quantity of the line item being purchased."]
    pub quantity: Option<i64>,
    #[doc = "The [tax rates](https://docs.stripe.com/api/tax_rates) which apply to this line item."]
    pub tax_rates: Option<Vec<String>>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adjustable_quantity", self.adjustable_quantity.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        if let Some(values) = self.tax_rates.as_ref() {
            form.field_array("tax_rates", values.as_slice());
        }
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsUpi {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUpiMandateOptions>,
    pub setup_future_usage:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUpiSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmount {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Shipping rates defined in each available currency option."]
    pub currency_options: Option<std::collections::HashMap<String, stripe_types_07::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmountCurrencyOptionsValue>>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
    }
}
