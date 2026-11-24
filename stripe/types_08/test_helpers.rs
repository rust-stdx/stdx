#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuel {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    pub r#type:
        Option<stripe_types_06::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuelType>,
    #[doc = "The units for `quantity_decimal`."]
    pub unit: Option<stripe_types_05::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuelUnit>,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    pub unit_cost_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("industry_product_code", self.industry_product_code.as_ref());
        form.field_opt("quantity_decimal", self.quantity_decimal.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("unit", self.unit.as_ref());
        form.field_opt("unit_cost_decimal", self.unit_cost_decimal.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    pub fuel:
        Option<stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    pub non_fuel:
        Option<stripe_types_06::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownNonFuel>,
    #[doc = "Information about tax included in this transaction."]
    pub tax: Option<stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("non_fuel", self.non_fuel.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetails {
    #[doc = "The source type."]
    pub r#type: stripe_types_05::test_helpers::CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetailsType,
    #[doc = "Optional fields for `us_bank_account`."]
    pub us_bank_account: Option<stripe_types_06::test_helpers::CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetailsUsBankAccount>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParamsMerchantData {
    #[doc = "A categorization of the seller's type of business."]
    pub category: Option<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsMerchantDataCategory>,
    #[doc = "City where the seller is located"]
    pub city: Option<String>,
    #[doc = "Country where the seller is located"]
    pub country: Option<String>,
    #[doc = "Name of the seller"]
    pub name: Option<String>,
    #[doc = "Identifier assigned to the seller by the card network."]
    pub network_id: Option<String>,
    #[doc = "Postal code where the seller is located"]
    pub postal_code: Option<String>,
    #[doc = "State where the seller is located"]
    pub state: Option<String>,
    #[doc = "An ID assigned by the seller to the location of the sale."]
    pub terminal_id: Option<String>,
    #[doc = "URL provided by the merchant on a 3DS request"]
    pub url: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParamsMerchantData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("category", self.category.as_ref());
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("network_id", self.network_id.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
        form.field_opt("terminal_id", self.terminal_id.as_ref());
        form.field_opt("url", self.url.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParamsPurchaseDetailsFlight {
    #[doc = "The time that the flight departed."]
    pub departure_at: Option<i64>,
    #[doc = "The name of the passenger."]
    pub passenger_name: Option<String>,
    #[doc = "Whether the ticket is refundable."]
    pub refundable: Option<bool>,
    #[doc = "The legs of the trip."]
    pub segments:
        Option<Vec<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFlightSegmentsItem>>,
    #[doc = "The travel agency that issued the ticket."]
    pub travel_agency: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParamsPurchaseDetailsFlight {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("departure_at", self.departure_at.as_ref());
        form.field_opt("passenger_name", self.passenger_name.as_ref());
        form.field_opt("refundable", self.refundable.as_ref());
        if let Some(values) = self.segments.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("segments", index, item);
            }
        }
        form.field_opt("travel_agency", self.travel_agency.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FailTestHelperTreasuryInboundTransferParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Details about a failed InboundTransfer."]
    pub failure_details:
        Option<stripe_types_05::test_helpers::FailTestHelperTreasuryInboundTransferParamsFailureDetails>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FailTestHelperTreasuryInboundTransferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("failure_details", self.failure_details.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct PresentPaymentMethodParams {
    #[doc = "Simulated on-reader tip amount."]
    pub amount_tip: Option<i64>,
    #[doc = "Simulated data for the card payment method."]
    pub card: Option<stripe_types_00::test_helpers::PresentPaymentMethodParamsCard>,
    #[doc = "Simulated data for the card_present payment method."]
    pub card_present: Option<stripe_types_02::test_helpers::PresentPaymentMethodParamsCardPresent>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Simulated data for the interac_present payment method."]
    pub interac_present: Option<stripe_types_05::test_helpers::PresentPaymentMethodParamsInteracPresent>,
    #[doc = "Simulated payment type."]
    pub r#type: Option<stripe_types_04::test_helpers::PresentPaymentMethodParamsType>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for PresentPaymentMethodParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tip", self.amount_tip.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("interac_present", self.interac_present.as_ref());
        form.field_opt("type", self.r#type.as_ref());
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
pub struct TestHelpersTestClock {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Time at which this clock is scheduled to auto delete."]
    #[json(default)]
    pub deletes_after: i64,
    #[doc = "Time at which all objects belonging to this clock are frozen."]
    #[json(default)]
    pub frozen_time: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The custom name supplied at creation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::test_helpers::TestHelpersTestClockObject,
    #[doc = "The status of the Test Clock."]
    #[json(default)]
    pub status: stripe_types_00::test_helpers::TestHelpersTestClockStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_05::billing::BillingClocksResourceStatusDetailsStatusDetails>,
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
impl crate::PagedItem for TestHelpersTestClock {
    fn id(&self) -> &str {
        &self.id
    }
}
