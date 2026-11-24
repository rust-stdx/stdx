#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => {
                "business"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => {
                "personal"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json>
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business
            }
            "personal" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal
            }
            other => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::Month => "month",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "month" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::Month,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    #[doc = "The desired country code of the bank account information."]
    pub country: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Fixed => "fixed",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Maximum => "maximum",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Fixed,
            "maximum" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Maximum,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPendingInvoiceItemsBehavior {
    Exclude,
    Include,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPendingInvoiceItemsBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPendingInvoiceItemsBehavior::Exclude => "exclude",
            CreateInvoiceParamsPendingInvoiceItemsBehavior::Include => "include",
            CreateInvoiceParamsPendingInvoiceItemsBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPendingInvoiceItemsBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsPendingInvoiceItemsBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclude" => CreateInvoiceParamsPendingInvoiceItemsBehavior::Exclude,
            "include" => CreateInvoiceParamsPendingInvoiceItemsBehavior::Include,
            other => CreateInvoiceParamsPendingInvoiceItemsBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPendingInvoiceItemsBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsShippingCostShippingRateDataType {
    FixedAmount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsShippingCostShippingRateDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsShippingCostShippingRateDataType::FixedAmount => "fixed_amount",
            CreateInvoiceParamsShippingCostShippingRateDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsShippingCostShippingRateDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsShippingCostShippingRateDataType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed_amount" => CreateInvoiceParamsShippingCostShippingRateDataType::FixedAmount,
            other => CreateInvoiceParamsShippingCostShippingRateDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsCustomerDetailsTax {
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    pub ip_address: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsCustomerDetailsTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsPreviewMode {
    Next,
    Recurring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsPreviewMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsPreviewMode::Next => "next",
            CreatePreviewParamsPreviewMode::Recurring => "recurring",
            CreatePreviewParamsPreviewMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsPreviewMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsPreviewMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "next" => CreatePreviewParamsPreviewMode::Next,
            "recurring" => CreatePreviewParamsPreviewMode::Recurring,
            other => CreatePreviewParamsPreviewMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsPreviewMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType {
    MaxItemPeriodStart,
    PhaseStart,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::MaxItemPeriodStart => {
                "max_item_period_start"
            }
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::PhaseStart => "phase_start",
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::Timestamp => "timestamp",
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "max_item_period_start" => {
                CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::MaxItemPeriodStart
            }
            "phase_start" => CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::PhaseStart,
            "timestamp" => CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::Timestamp,
            other => CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStartType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemBillingThresholds {
    #[doc = "Monetary threshold that triggers the subscription to advance to a new billing period"]
    pub amount_gte: Option<i64>,
    #[doc = "Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached."]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemBillingThresholds {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_gte", self.amount_gte.as_ref());
        form.field_opt("reset_billing_cycle_anchor", self.reset_billing_cycle_anchor.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemTransferData {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub amount_percent: Option<f64>,
    #[doc = "ID of an existing, connected Stripe account."]
    pub destination: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_percent", self.amount_percent.as_ref());
        form.field("destination", &self.destination);
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl InvoiceCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceCollectionMethod::ChargeAutomatically => "charge_automatically",
            InvoiceCollectionMethod::SendInvoice => "send_invoice",
            InvoiceCollectionMethod::Unknown(value) => value,
        }
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
impl Default for InvoiceCollectionMethod {
    fn default() -> Self {
        InvoiceCollectionMethod::Unknown(String::new())
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
impl<'json> json::Decode<'json> for InvoiceCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => InvoiceCollectionMethod::ChargeAutomatically,
            "send_invoice" => InvoiceCollectionMethod::SendInvoice,
            other => InvoiceCollectionMethod::Unknown(other.to_string()),
        })
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl InvoiceCustomerTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceCustomerTaxExempt::Exempt => "exempt",
            InvoiceCustomerTaxExempt::None => "none",
            InvoiceCustomerTaxExempt::Reverse => "reverse",
            InvoiceCustomerTaxExempt::Unknown(value) => value,
        }
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
impl<'json> json::Decode<'json> for InvoiceCustomerTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceCustomerTaxExempt::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exempt" => InvoiceCustomerTaxExempt::Exempt,
            "none" => InvoiceCustomerTaxExempt::None,
            "reverse" => InvoiceCustomerTaxExempt::Reverse,
            other => InvoiceCustomerTaxExempt::Unknown(other.to_string()),
        })
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
pub struct InvoiceItemThresholdReason {
    #[doc = "The IDs of the line items that triggered the threshold invoice."]
    #[json(default)]
    pub line_item_ids: Vec<String>,
    #[doc = "The quantity threshold boundary that applied to the given line item."]
    #[json(default)]
    pub usage_gte: i64,
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
pub struct InvoiceLineItemPeriod {
    #[doc = "The end of the period, which must be greater than or equal to the start."]
    #[json(default)]
    pub end: i64,
    #[doc = "The start of the period."]
    #[json(default)]
    pub start: i64,
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceMandateOptionsCardAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl InvoiceMandateOptionsCardAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceMandateOptionsCardAmountType::Fixed => "fixed",
            InvoiceMandateOptionsCardAmountType::Maximum => "maximum",
            InvoiceMandateOptionsCardAmountType::Unknown(value) => value,
        }
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
impl<'json> json::Decode<'json> for InvoiceMandateOptionsCardAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceMandateOptionsCardAmountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => InvoiceMandateOptionsCardAmountType::Fixed,
            "maximum" => InvoiceMandateOptionsCardAmountType::Maximum,
            other => InvoiceMandateOptionsCardAmountType::Unknown(other.to_string()),
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
pub struct InvoiceSettingCheckoutRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<String>,
    #[doc = "ID of the invoice rendering template to be used for the generated invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
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
pub struct InvoiceSettingCustomField {
    #[doc = "The name of the custom field."]
    #[json(default)]
    pub name: String,
    #[doc = "The value of the custom field."]
    #[json(default)]
    pub value: String,
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
pub struct InvoicesResourceConfirmationSecret {
    #[doc = "The client_secret of the payment that Stripe creates for the invoice after finalization."]
    #[json(default)]
    pub client_secret: String,
    #[doc = "The type of client_secret."]
    #[json(rename = "type", default)]
    pub r#type: String,
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoicesResourceInvoiceTaxIdType {
    AdNrt,
    AeTrn,
    AlTin,
    AmTin,
    AoTin,
    ArCuit,
    AuAbn,
    AuArn,
    AwTin,
    AzTin,
    BaTin,
    BbTin,
    BdBin,
    BfIfu,
    BgUic,
    BhVat,
    BjIfu,
    BoTin,
    BrCnpj,
    BrCpf,
    BsTin,
    ByTin,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    CdNif,
    ChUid,
    ChVat,
    ClTin,
    CmNiu,
    CnTin,
    CoNit,
    CrTin,
    CvNif,
    DeStn,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EtTin,
    EuOssVat,
    EuVat,
    FoVat,
    GbVat,
    GeVat,
    GiTin,
    GnNif,
    HkBr,
    HrOib,
    HuTin,
    IcNif,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    ItCf,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KgTin,
    KhTin,
    KrBrn,
    KzBin,
    LaTin,
    LiUid,
    LiVat,
    LkVat,
    MaVat,
    MdVat,
    MePib,
    MkVat,
    MrNif,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NpPan,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    PlNip,
    PyRuc,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SnNinea,
    SrFin,
    SvNit,
    ThVat,
    TjTin,
    TrTin,
    TwVat,
    TzVat,
    UaVat,
    UgTin,
    UnknownValue,
    UsEin,
    UyRuc,
    UzTin,
    UzVat,
    VeRif,
    VnTin,
    ZaVat,
    ZmTin,
    ZwTin,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl InvoicesResourceInvoiceTaxIdType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicesResourceInvoiceTaxIdType::AdNrt => "ad_nrt",
            InvoicesResourceInvoiceTaxIdType::AeTrn => "ae_trn",
            InvoicesResourceInvoiceTaxIdType::AlTin => "al_tin",
            InvoicesResourceInvoiceTaxIdType::AmTin => "am_tin",
            InvoicesResourceInvoiceTaxIdType::AoTin => "ao_tin",
            InvoicesResourceInvoiceTaxIdType::ArCuit => "ar_cuit",
            InvoicesResourceInvoiceTaxIdType::AuAbn => "au_abn",
            InvoicesResourceInvoiceTaxIdType::AuArn => "au_arn",
            InvoicesResourceInvoiceTaxIdType::AwTin => "aw_tin",
            InvoicesResourceInvoiceTaxIdType::AzTin => "az_tin",
            InvoicesResourceInvoiceTaxIdType::BaTin => "ba_tin",
            InvoicesResourceInvoiceTaxIdType::BbTin => "bb_tin",
            InvoicesResourceInvoiceTaxIdType::BdBin => "bd_bin",
            InvoicesResourceInvoiceTaxIdType::BfIfu => "bf_ifu",
            InvoicesResourceInvoiceTaxIdType::BgUic => "bg_uic",
            InvoicesResourceInvoiceTaxIdType::BhVat => "bh_vat",
            InvoicesResourceInvoiceTaxIdType::BjIfu => "bj_ifu",
            InvoicesResourceInvoiceTaxIdType::BoTin => "bo_tin",
            InvoicesResourceInvoiceTaxIdType::BrCnpj => "br_cnpj",
            InvoicesResourceInvoiceTaxIdType::BrCpf => "br_cpf",
            InvoicesResourceInvoiceTaxIdType::BsTin => "bs_tin",
            InvoicesResourceInvoiceTaxIdType::ByTin => "by_tin",
            InvoicesResourceInvoiceTaxIdType::CaBn => "ca_bn",
            InvoicesResourceInvoiceTaxIdType::CaGstHst => "ca_gst_hst",
            InvoicesResourceInvoiceTaxIdType::CaPstBc => "ca_pst_bc",
            InvoicesResourceInvoiceTaxIdType::CaPstMb => "ca_pst_mb",
            InvoicesResourceInvoiceTaxIdType::CaPstSk => "ca_pst_sk",
            InvoicesResourceInvoiceTaxIdType::CaQst => "ca_qst",
            InvoicesResourceInvoiceTaxIdType::CdNif => "cd_nif",
            InvoicesResourceInvoiceTaxIdType::ChUid => "ch_uid",
            InvoicesResourceInvoiceTaxIdType::ChVat => "ch_vat",
            InvoicesResourceInvoiceTaxIdType::ClTin => "cl_tin",
            InvoicesResourceInvoiceTaxIdType::CmNiu => "cm_niu",
            InvoicesResourceInvoiceTaxIdType::CnTin => "cn_tin",
            InvoicesResourceInvoiceTaxIdType::CoNit => "co_nit",
            InvoicesResourceInvoiceTaxIdType::CrTin => "cr_tin",
            InvoicesResourceInvoiceTaxIdType::CvNif => "cv_nif",
            InvoicesResourceInvoiceTaxIdType::DeStn => "de_stn",
            InvoicesResourceInvoiceTaxIdType::DoRcn => "do_rcn",
            InvoicesResourceInvoiceTaxIdType::EcRuc => "ec_ruc",
            InvoicesResourceInvoiceTaxIdType::EgTin => "eg_tin",
            InvoicesResourceInvoiceTaxIdType::EsCif => "es_cif",
            InvoicesResourceInvoiceTaxIdType::EtTin => "et_tin",
            InvoicesResourceInvoiceTaxIdType::EuOssVat => "eu_oss_vat",
            InvoicesResourceInvoiceTaxIdType::EuVat => "eu_vat",
            InvoicesResourceInvoiceTaxIdType::FoVat => "fo_vat",
            InvoicesResourceInvoiceTaxIdType::GbVat => "gb_vat",
            InvoicesResourceInvoiceTaxIdType::GeVat => "ge_vat",
            InvoicesResourceInvoiceTaxIdType::GiTin => "gi_tin",
            InvoicesResourceInvoiceTaxIdType::GnNif => "gn_nif",
            InvoicesResourceInvoiceTaxIdType::HkBr => "hk_br",
            InvoicesResourceInvoiceTaxIdType::HrOib => "hr_oib",
            InvoicesResourceInvoiceTaxIdType::HuTin => "hu_tin",
            InvoicesResourceInvoiceTaxIdType::IcNif => "ic_nif",
            InvoicesResourceInvoiceTaxIdType::IdNpwp => "id_npwp",
            InvoicesResourceInvoiceTaxIdType::IlVat => "il_vat",
            InvoicesResourceInvoiceTaxIdType::InGst => "in_gst",
            InvoicesResourceInvoiceTaxIdType::IsVat => "is_vat",
            InvoicesResourceInvoiceTaxIdType::ItCf => "it_cf",
            InvoicesResourceInvoiceTaxIdType::JpCn => "jp_cn",
            InvoicesResourceInvoiceTaxIdType::JpRn => "jp_rn",
            InvoicesResourceInvoiceTaxIdType::JpTrn => "jp_trn",
            InvoicesResourceInvoiceTaxIdType::KePin => "ke_pin",
            InvoicesResourceInvoiceTaxIdType::KgTin => "kg_tin",
            InvoicesResourceInvoiceTaxIdType::KhTin => "kh_tin",
            InvoicesResourceInvoiceTaxIdType::KrBrn => "kr_brn",
            InvoicesResourceInvoiceTaxIdType::KzBin => "kz_bin",
            InvoicesResourceInvoiceTaxIdType::LaTin => "la_tin",
            InvoicesResourceInvoiceTaxIdType::LiUid => "li_uid",
            InvoicesResourceInvoiceTaxIdType::LiVat => "li_vat",
            InvoicesResourceInvoiceTaxIdType::LkVat => "lk_vat",
            InvoicesResourceInvoiceTaxIdType::MaVat => "ma_vat",
            InvoicesResourceInvoiceTaxIdType::MdVat => "md_vat",
            InvoicesResourceInvoiceTaxIdType::MePib => "me_pib",
            InvoicesResourceInvoiceTaxIdType::MkVat => "mk_vat",
            InvoicesResourceInvoiceTaxIdType::MrNif => "mr_nif",
            InvoicesResourceInvoiceTaxIdType::MxRfc => "mx_rfc",
            InvoicesResourceInvoiceTaxIdType::MyFrp => "my_frp",
            InvoicesResourceInvoiceTaxIdType::MyItn => "my_itn",
            InvoicesResourceInvoiceTaxIdType::MySst => "my_sst",
            InvoicesResourceInvoiceTaxIdType::NgTin => "ng_tin",
            InvoicesResourceInvoiceTaxIdType::NoVat => "no_vat",
            InvoicesResourceInvoiceTaxIdType::NoVoec => "no_voec",
            InvoicesResourceInvoiceTaxIdType::NpPan => "np_pan",
            InvoicesResourceInvoiceTaxIdType::NzGst => "nz_gst",
            InvoicesResourceInvoiceTaxIdType::OmVat => "om_vat",
            InvoicesResourceInvoiceTaxIdType::PeRuc => "pe_ruc",
            InvoicesResourceInvoiceTaxIdType::PhTin => "ph_tin",
            InvoicesResourceInvoiceTaxIdType::PlNip => "pl_nip",
            InvoicesResourceInvoiceTaxIdType::PyRuc => "py_ruc",
            InvoicesResourceInvoiceTaxIdType::RoTin => "ro_tin",
            InvoicesResourceInvoiceTaxIdType::RsPib => "rs_pib",
            InvoicesResourceInvoiceTaxIdType::RuInn => "ru_inn",
            InvoicesResourceInvoiceTaxIdType::RuKpp => "ru_kpp",
            InvoicesResourceInvoiceTaxIdType::SaVat => "sa_vat",
            InvoicesResourceInvoiceTaxIdType::SgGst => "sg_gst",
            InvoicesResourceInvoiceTaxIdType::SgUen => "sg_uen",
            InvoicesResourceInvoiceTaxIdType::SiTin => "si_tin",
            InvoicesResourceInvoiceTaxIdType::SnNinea => "sn_ninea",
            InvoicesResourceInvoiceTaxIdType::SrFin => "sr_fin",
            InvoicesResourceInvoiceTaxIdType::SvNit => "sv_nit",
            InvoicesResourceInvoiceTaxIdType::ThVat => "th_vat",
            InvoicesResourceInvoiceTaxIdType::TjTin => "tj_tin",
            InvoicesResourceInvoiceTaxIdType::TrTin => "tr_tin",
            InvoicesResourceInvoiceTaxIdType::TwVat => "tw_vat",
            InvoicesResourceInvoiceTaxIdType::TzVat => "tz_vat",
            InvoicesResourceInvoiceTaxIdType::UaVat => "ua_vat",
            InvoicesResourceInvoiceTaxIdType::UgTin => "ug_tin",
            InvoicesResourceInvoiceTaxIdType::UnknownValue => "unknown",
            InvoicesResourceInvoiceTaxIdType::UsEin => "us_ein",
            InvoicesResourceInvoiceTaxIdType::UyRuc => "uy_ruc",
            InvoicesResourceInvoiceTaxIdType::UzTin => "uz_tin",
            InvoicesResourceInvoiceTaxIdType::UzVat => "uz_vat",
            InvoicesResourceInvoiceTaxIdType::VeRif => "ve_rif",
            InvoicesResourceInvoiceTaxIdType::VnTin => "vn_tin",
            InvoicesResourceInvoiceTaxIdType::ZaVat => "za_vat",
            InvoicesResourceInvoiceTaxIdType::ZmTin => "zm_tin",
            InvoicesResourceInvoiceTaxIdType::ZwTin => "zw_tin",
            InvoicesResourceInvoiceTaxIdType::Unknown(value) => value,
        }
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
impl Default for InvoicesResourceInvoiceTaxIdType {
    fn default() -> Self {
        InvoicesResourceInvoiceTaxIdType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for InvoicesResourceInvoiceTaxIdType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoicesResourceInvoiceTaxIdType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ad_nrt" => InvoicesResourceInvoiceTaxIdType::AdNrt,
            "ae_trn" => InvoicesResourceInvoiceTaxIdType::AeTrn,
            "al_tin" => InvoicesResourceInvoiceTaxIdType::AlTin,
            "am_tin" => InvoicesResourceInvoiceTaxIdType::AmTin,
            "ao_tin" => InvoicesResourceInvoiceTaxIdType::AoTin,
            "ar_cuit" => InvoicesResourceInvoiceTaxIdType::ArCuit,
            "au_abn" => InvoicesResourceInvoiceTaxIdType::AuAbn,
            "au_arn" => InvoicesResourceInvoiceTaxIdType::AuArn,
            "aw_tin" => InvoicesResourceInvoiceTaxIdType::AwTin,
            "az_tin" => InvoicesResourceInvoiceTaxIdType::AzTin,
            "ba_tin" => InvoicesResourceInvoiceTaxIdType::BaTin,
            "bb_tin" => InvoicesResourceInvoiceTaxIdType::BbTin,
            "bd_bin" => InvoicesResourceInvoiceTaxIdType::BdBin,
            "bf_ifu" => InvoicesResourceInvoiceTaxIdType::BfIfu,
            "bg_uic" => InvoicesResourceInvoiceTaxIdType::BgUic,
            "bh_vat" => InvoicesResourceInvoiceTaxIdType::BhVat,
            "bj_ifu" => InvoicesResourceInvoiceTaxIdType::BjIfu,
            "bo_tin" => InvoicesResourceInvoiceTaxIdType::BoTin,
            "br_cnpj" => InvoicesResourceInvoiceTaxIdType::BrCnpj,
            "br_cpf" => InvoicesResourceInvoiceTaxIdType::BrCpf,
            "bs_tin" => InvoicesResourceInvoiceTaxIdType::BsTin,
            "by_tin" => InvoicesResourceInvoiceTaxIdType::ByTin,
            "ca_bn" => InvoicesResourceInvoiceTaxIdType::CaBn,
            "ca_gst_hst" => InvoicesResourceInvoiceTaxIdType::CaGstHst,
            "ca_pst_bc" => InvoicesResourceInvoiceTaxIdType::CaPstBc,
            "ca_pst_mb" => InvoicesResourceInvoiceTaxIdType::CaPstMb,
            "ca_pst_sk" => InvoicesResourceInvoiceTaxIdType::CaPstSk,
            "ca_qst" => InvoicesResourceInvoiceTaxIdType::CaQst,
            "cd_nif" => InvoicesResourceInvoiceTaxIdType::CdNif,
            "ch_uid" => InvoicesResourceInvoiceTaxIdType::ChUid,
            "ch_vat" => InvoicesResourceInvoiceTaxIdType::ChVat,
            "cl_tin" => InvoicesResourceInvoiceTaxIdType::ClTin,
            "cm_niu" => InvoicesResourceInvoiceTaxIdType::CmNiu,
            "cn_tin" => InvoicesResourceInvoiceTaxIdType::CnTin,
            "co_nit" => InvoicesResourceInvoiceTaxIdType::CoNit,
            "cr_tin" => InvoicesResourceInvoiceTaxIdType::CrTin,
            "cv_nif" => InvoicesResourceInvoiceTaxIdType::CvNif,
            "de_stn" => InvoicesResourceInvoiceTaxIdType::DeStn,
            "do_rcn" => InvoicesResourceInvoiceTaxIdType::DoRcn,
            "ec_ruc" => InvoicesResourceInvoiceTaxIdType::EcRuc,
            "eg_tin" => InvoicesResourceInvoiceTaxIdType::EgTin,
            "es_cif" => InvoicesResourceInvoiceTaxIdType::EsCif,
            "et_tin" => InvoicesResourceInvoiceTaxIdType::EtTin,
            "eu_oss_vat" => InvoicesResourceInvoiceTaxIdType::EuOssVat,
            "eu_vat" => InvoicesResourceInvoiceTaxIdType::EuVat,
            "fo_vat" => InvoicesResourceInvoiceTaxIdType::FoVat,
            "gb_vat" => InvoicesResourceInvoiceTaxIdType::GbVat,
            "ge_vat" => InvoicesResourceInvoiceTaxIdType::GeVat,
            "gi_tin" => InvoicesResourceInvoiceTaxIdType::GiTin,
            "gn_nif" => InvoicesResourceInvoiceTaxIdType::GnNif,
            "hk_br" => InvoicesResourceInvoiceTaxIdType::HkBr,
            "hr_oib" => InvoicesResourceInvoiceTaxIdType::HrOib,
            "hu_tin" => InvoicesResourceInvoiceTaxIdType::HuTin,
            "ic_nif" => InvoicesResourceInvoiceTaxIdType::IcNif,
            "id_npwp" => InvoicesResourceInvoiceTaxIdType::IdNpwp,
            "il_vat" => InvoicesResourceInvoiceTaxIdType::IlVat,
            "in_gst" => InvoicesResourceInvoiceTaxIdType::InGst,
            "is_vat" => InvoicesResourceInvoiceTaxIdType::IsVat,
            "it_cf" => InvoicesResourceInvoiceTaxIdType::ItCf,
            "jp_cn" => InvoicesResourceInvoiceTaxIdType::JpCn,
            "jp_rn" => InvoicesResourceInvoiceTaxIdType::JpRn,
            "jp_trn" => InvoicesResourceInvoiceTaxIdType::JpTrn,
            "ke_pin" => InvoicesResourceInvoiceTaxIdType::KePin,
            "kg_tin" => InvoicesResourceInvoiceTaxIdType::KgTin,
            "kh_tin" => InvoicesResourceInvoiceTaxIdType::KhTin,
            "kr_brn" => InvoicesResourceInvoiceTaxIdType::KrBrn,
            "kz_bin" => InvoicesResourceInvoiceTaxIdType::KzBin,
            "la_tin" => InvoicesResourceInvoiceTaxIdType::LaTin,
            "li_uid" => InvoicesResourceInvoiceTaxIdType::LiUid,
            "li_vat" => InvoicesResourceInvoiceTaxIdType::LiVat,
            "lk_vat" => InvoicesResourceInvoiceTaxIdType::LkVat,
            "ma_vat" => InvoicesResourceInvoiceTaxIdType::MaVat,
            "md_vat" => InvoicesResourceInvoiceTaxIdType::MdVat,
            "me_pib" => InvoicesResourceInvoiceTaxIdType::MePib,
            "mk_vat" => InvoicesResourceInvoiceTaxIdType::MkVat,
            "mr_nif" => InvoicesResourceInvoiceTaxIdType::MrNif,
            "mx_rfc" => InvoicesResourceInvoiceTaxIdType::MxRfc,
            "my_frp" => InvoicesResourceInvoiceTaxIdType::MyFrp,
            "my_itn" => InvoicesResourceInvoiceTaxIdType::MyItn,
            "my_sst" => InvoicesResourceInvoiceTaxIdType::MySst,
            "ng_tin" => InvoicesResourceInvoiceTaxIdType::NgTin,
            "no_vat" => InvoicesResourceInvoiceTaxIdType::NoVat,
            "no_voec" => InvoicesResourceInvoiceTaxIdType::NoVoec,
            "np_pan" => InvoicesResourceInvoiceTaxIdType::NpPan,
            "nz_gst" => InvoicesResourceInvoiceTaxIdType::NzGst,
            "om_vat" => InvoicesResourceInvoiceTaxIdType::OmVat,
            "pe_ruc" => InvoicesResourceInvoiceTaxIdType::PeRuc,
            "ph_tin" => InvoicesResourceInvoiceTaxIdType::PhTin,
            "pl_nip" => InvoicesResourceInvoiceTaxIdType::PlNip,
            "py_ruc" => InvoicesResourceInvoiceTaxIdType::PyRuc,
            "ro_tin" => InvoicesResourceInvoiceTaxIdType::RoTin,
            "rs_pib" => InvoicesResourceInvoiceTaxIdType::RsPib,
            "ru_inn" => InvoicesResourceInvoiceTaxIdType::RuInn,
            "ru_kpp" => InvoicesResourceInvoiceTaxIdType::RuKpp,
            "sa_vat" => InvoicesResourceInvoiceTaxIdType::SaVat,
            "sg_gst" => InvoicesResourceInvoiceTaxIdType::SgGst,
            "sg_uen" => InvoicesResourceInvoiceTaxIdType::SgUen,
            "si_tin" => InvoicesResourceInvoiceTaxIdType::SiTin,
            "sn_ninea" => InvoicesResourceInvoiceTaxIdType::SnNinea,
            "sr_fin" => InvoicesResourceInvoiceTaxIdType::SrFin,
            "sv_nit" => InvoicesResourceInvoiceTaxIdType::SvNit,
            "th_vat" => InvoicesResourceInvoiceTaxIdType::ThVat,
            "tj_tin" => InvoicesResourceInvoiceTaxIdType::TjTin,
            "tr_tin" => InvoicesResourceInvoiceTaxIdType::TrTin,
            "tw_vat" => InvoicesResourceInvoiceTaxIdType::TwVat,
            "tz_vat" => InvoicesResourceInvoiceTaxIdType::TzVat,
            "ua_vat" => InvoicesResourceInvoiceTaxIdType::UaVat,
            "ug_tin" => InvoicesResourceInvoiceTaxIdType::UgTin,
            "unknown" => InvoicesResourceInvoiceTaxIdType::UnknownValue,
            "us_ein" => InvoicesResourceInvoiceTaxIdType::UsEin,
            "uy_ruc" => InvoicesResourceInvoiceTaxIdType::UyRuc,
            "uz_tin" => InvoicesResourceInvoiceTaxIdType::UzTin,
            "uz_vat" => InvoicesResourceInvoiceTaxIdType::UzVat,
            "ve_rif" => InvoicesResourceInvoiceTaxIdType::VeRif,
            "vn_tin" => InvoicesResourceInvoiceTaxIdType::VnTin,
            "za_vat" => InvoicesResourceInvoiceTaxIdType::ZaVat,
            "zm_tin" => InvoicesResourceInvoiceTaxIdType::ZmTin,
            "zw_tin" => InvoicesResourceInvoiceTaxIdType::ZwTin,
            other => InvoicesResourceInvoiceTaxIdType::Unknown(other.to_string()),
        })
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoicesResourcePretaxCreditAmountType {
    CreditBalanceTransaction,
    Discount,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl InvoicesResourcePretaxCreditAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicesResourcePretaxCreditAmountType::CreditBalanceTransaction => "credit_balance_transaction",
            InvoicesResourcePretaxCreditAmountType::Discount => "discount",
            InvoicesResourcePretaxCreditAmountType::Unknown(value) => value,
        }
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
impl Default for InvoicesResourcePretaxCreditAmountType {
    fn default() -> Self {
        InvoicesResourcePretaxCreditAmountType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for InvoicesResourcePretaxCreditAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoicesResourcePretaxCreditAmountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_balance_transaction" => InvoicesResourcePretaxCreditAmountType::CreditBalanceTransaction,
            "discount" => InvoicesResourcePretaxCreditAmountType::Discount,
            other => InvoicesResourcePretaxCreditAmountType::Unknown(other.to_string()),
        })
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoicesResourceUncollectibleStatusDetailsReason {
    MaxPaymentAttempts,
    PaymentNotReceived,
    SubscriptionCanceled,
    SubscriptionPaused,
    UserForgiven,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl InvoicesResourceUncollectibleStatusDetailsReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicesResourceUncollectibleStatusDetailsReason::MaxPaymentAttempts => "max_payment_attempts",
            InvoicesResourceUncollectibleStatusDetailsReason::PaymentNotReceived => "payment_not_received",
            InvoicesResourceUncollectibleStatusDetailsReason::SubscriptionCanceled => "subscription_canceled",
            InvoicesResourceUncollectibleStatusDetailsReason::SubscriptionPaused => "subscription_paused",
            InvoicesResourceUncollectibleStatusDetailsReason::UserForgiven => "user_forgiven",
            InvoicesResourceUncollectibleStatusDetailsReason::Unknown(value) => value,
        }
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
impl<'json> json::Decode<'json> for InvoicesResourceUncollectibleStatusDetailsReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoicesResourceUncollectibleStatusDetailsReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "max_payment_attempts" => InvoicesResourceUncollectibleStatusDetailsReason::MaxPaymentAttempts,
            "payment_not_received" => InvoicesResourceUncollectibleStatusDetailsReason::PaymentNotReceived,
            "subscription_canceled" => InvoicesResourceUncollectibleStatusDetailsReason::SubscriptionCanceled,
            "subscription_paused" => InvoicesResourceUncollectibleStatusDetailsReason::SubscriptionPaused,
            "user_forgiven" => InvoicesResourceUncollectibleStatusDetailsReason::UserForgiven,
            other => InvoicesResourceUncollectibleStatusDetailsReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoicingRuleType {
    DeferUntilCreditedItemsResolved,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl InvoicingRuleType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicingRuleType::DeferUntilCreditedItemsResolved => "defer_until_credited_items_resolved",
            InvoicingRuleType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl Default for InvoicingRuleType {
    fn default() -> Self {
        InvoicingRuleType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl<'json> json::Decode<'json> for InvoicingRuleType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoicingRuleType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "defer_until_credited_items_resolved" => InvoicingRuleType::DeferUntilCreditedItemsResolved,
            other => InvoicingRuleType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoicesParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for ListInvoicesParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
