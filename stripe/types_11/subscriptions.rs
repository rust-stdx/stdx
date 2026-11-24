#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsAddInvoiceItemsItem {
    #[doc = "Controls whether discounts apply to this invoice item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons to redeem into discounts for the item."]
    pub discounts:
        Option<Vec<stripe_types_02::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemDiscountsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_07::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPeriod>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_05::subscriptions::CreateSubscriptionParamsAddInvoiceItemsItemPriceData>,
    #[doc = "Quantity for this item."]
    pub quantity: Option<i64>,
    #[doc = "The tax rates which apply to the item."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("discountable", self.discountable.as_ref());
        if let Some(values) = self.discounts.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("discounts", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("period", self.period.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsBillingSchedulesItem {
    #[doc = "Configure billing schedule differently for individual subscription items."]
    pub applies_to:
        Option<Vec<stripe_types_06::subscriptions::CreateSubscriptionParamsBillingSchedulesItemAppliesToItem>>,
    #[doc = "The end date for the billing schedule."]
    pub bill_until: stripe_types_08::subscriptions::CreateSubscriptionParamsBillingSchedulesItemBillUntil,
    #[doc = "Specify a key for the billing schedule."]
    pub key: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.applies_to.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("applies_to", index, item);
            }
        }
        form.field("bill_until", &self.bill_until);
        form.field_opt("key", self.key.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionParamsItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: stripe_types_09::subscriptions::CreateSubscriptionParamsItemsItemPriceDataRecurring,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::subscriptions::CreateSubscriptionParamsItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field("recurring", &self.recurring);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_07::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<stripe_types_06::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct MigrateSubscriptionParamsBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_09::subscriptions::MigrateSubscriptionParamsBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_04::subscriptions::MigrateSubscriptionParamsBillingModeType,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for MigrateSubscriptionParamsBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct PauseSubscriptionParams {
    #[doc = "Controls what to bill for when pausing the subscription."]
    pub bill_for: Option<stripe_types_07::subscriptions::PauseSubscriptionParamsBillFor>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Determines how to handle debits and credits when pausing."]
    pub invoicing_behavior: Option<stripe_types_01::subscriptions::PauseSubscriptionParamsInvoicingBehavior>,
    #[doc = "The type of pause to apply."]
    pub r#type: Option<stripe_types_04::subscriptions::PauseSubscriptionParamsType>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bill_for", self.bill_for.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoicing_behavior", self.invoicing_behavior.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct ResumeSubscriptionParams {
    #[doc = "The billing cycle anchor that applies when the subscription is resumed."]
    pub billing_cycle_anchor: Option<stripe_types_07::subscriptions::ResumeSubscriptionParamsBillingCycleAnchor>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Controls whether Stripe attempts payment on the resumption invoice in the resume request, and how payment on that inv..."]
    pub payment_behavior: Option<stripe_types_00::subscriptions::ResumeSubscriptionParamsPaymentBehavior>,
    #[doc = "Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) resulting from the `b..."]
    pub proration_behavior: Option<stripe_types_06::subscriptions::ResumeSubscriptionParamsProrationBehavior>,
    #[doc = "If set, prorations will be calculated as though the subscription was resumed at the given time."]
    pub proration_date: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ResumeSubscriptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("payment_behavior", self.payment_behavior.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("proration_date", self.proration_date.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsAutomaticTax {
    #[doc = "Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscri..."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_09::subscriptions::UpdateSubscriptionParamsAutomaticTaxLiability>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillie {
    #[doc = "Registration details about the buyer's organization."]
    pub company_details: Option<
        stripe_types_07::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetails,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("company_details", self.company_details.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCard {
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options: Option<stripe_types_07::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "Selected network to process this Subscription on."]
    pub network: Option<stripe_types_06::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure: Option<stripe_types_06::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("network", self.network.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer: Option<stripe_types_09::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options: Option<
        stripe_types_07::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_08::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_06::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_06::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("permissions", index, item);
            }
        }
        if let Some(values) = self.prefetch.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prefetch", index, item);
            }
        }
    }
}
