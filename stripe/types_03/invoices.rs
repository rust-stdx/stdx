#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic,
            "instant" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant,
            "microdeposits" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits
            }
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    Bonus,
    FixedCount,
    Revolving,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Bonus => "bonus",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount => "fixed_count",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Revolving => "revolving",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bonus" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Bonus,
            "fixed_count" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount,
            "revolving" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Revolving,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof {
    Always,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Always => "always",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Never => "never",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Always,
            "never" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Never,
            other => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPixAmountIncludesIof {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances => "balances",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership => "ownership",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions => "transactions",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json>
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances,
            "ownership" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership,
            "transactions" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Exclusive => {
                "exclusive"
            }
            CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Inclusive => {
                "inclusive"
            }
            CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Unspecified => {
                "unspecified"
            }
            CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json>
    for CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => {
                CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Exclusive
            }
            "inclusive" => {
                CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Inclusive
            }
            "unspecified" => {
                CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Unspecified
            }
            other => {
                CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsEndBehavior {
    Cancel,
    Release,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsEndBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsEndBehavior::Cancel => "cancel",
            CreatePreviewParamsScheduleDetailsEndBehavior::Release => "release",
            CreatePreviewParamsScheduleDetailsEndBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsEndBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsScheduleDetailsEndBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => CreatePreviewParamsScheduleDetailsEndBehavior::Cancel,
            "release" => CreatePreviewParamsScheduleDetailsEndBehavior::Release,
            other => CreatePreviewParamsScheduleDetailsEndBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType {
    MinItemPeriodEnd,
    PhaseEnd,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::MinItemPeriodEnd => {
                "min_item_period_end"
            }
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::PhaseEnd => "phase_end",
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::Timestamp => "timestamp",
            CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "min_item_period_end" => {
                CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::MinItemPeriodEnd
            }
            "phase_end" => CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::PhaseEnd,
            "timestamp" => CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::Timestamp,
            other => {
                CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEndType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsSubscriptionDetailsItemsItemCurrentTrial {
    #[doc = "The ID of the trial offer to apply to the subscription item."]
    pub trial_offer: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsItemsItemCurrentTrial {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("trial_offer", &self.trial_offer);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsSubscriptionDetailsResumeAt {
    Now,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsSubscriptionDetailsResumeAt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsSubscriptionDetailsResumeAt::Now => "now",
            CreatePreviewParamsSubscriptionDetailsResumeAt::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsSubscriptionDetailsResumeAt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsSubscriptionDetailsResumeAt::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => CreatePreviewParamsSubscriptionDetailsResumeAt::Now,
            other => CreatePreviewParamsSubscriptionDetailsResumeAt::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsResumeAt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceItemProrationCreditedItemsType {
    InvoiceItem,
    InvoiceLineItems,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl InvoiceItemProrationCreditedItemsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceItemProrationCreditedItemsType::InvoiceItem => "invoice_item",
            InvoiceItemProrationCreditedItemsType::InvoiceLineItems => "invoice_line_items",
            InvoiceItemProrationCreditedItemsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl Default for InvoiceItemProrationCreditedItemsType {
    fn default() -> Self {
        InvoiceItemProrationCreditedItemsType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl<'json> json::Decode<'json> for InvoiceItemProrationCreditedItemsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceItemProrationCreditedItemsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice_item" => InvoiceItemProrationCreditedItemsType::InvoiceItem,
            "invoice_line_items" => InvoiceItemProrationCreditedItemsType::InvoiceLineItems,
            other => InvoiceItemProrationCreditedItemsType::Unknown(other.to_string()),
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
pub enum InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    Charge,
    PaymentIntent,
    PaymentRecord,
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
impl InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Charge => "charge",
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::PaymentIntent => "payment_intent",
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::PaymentRecord => "payment_record",
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Unknown(value) => value,
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
impl Default for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn default() -> Self {
        InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge" => InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Charge,
            "payment_intent" => InvoicesPaymentsInvoicePaymentAssociatedPaymentType::PaymentIntent,
            "payment_record" => InvoicesPaymentsInvoicePaymentAssociatedPaymentType::PaymentRecord,
            other => InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct PayInvoiceParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "In cases where the source used to pay the invoice has insufficient funds, passing `forgive=true` controls whether a c..."]
    pub forgive: Option<bool>,
    #[doc = "ID of the mandate to be used for this invoice."]
    pub mandate: Option<json::OwnedValue>,
    #[doc = "Indicates if a customer is on or off-session while an invoice payment is attempted."]
    pub off_session: Option<bool>,
    #[doc = "Boolean representing whether an invoice is paid outside of Stripe."]
    pub paid_out_of_band: Option<bool>,
    #[doc = "A PaymentMethod to be charged."]
    pub payment_method: Option<String>,
    #[doc = "A payment source to be charged."]
    pub source: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for PayInvoiceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("forgive", self.forgive.as_ref());
        form.field_opt("off_session", self.off_session.as_ref());
        form.field_opt("paid_out_of_band", self.paid_out_of_band.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("source", self.source.as_ref());
    }
}
