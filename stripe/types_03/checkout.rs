#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAffirmPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutAffirmPaymentMethodOptionsSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAffirmPaymentMethodOptionsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAffirmPaymentMethodOptionsSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CheckoutAffirmPaymentMethodOptionsSetupFutureUsage::None,
            other => CheckoutAffirmPaymentMethodOptionsSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::None,
            "off_session" => CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::OffSession,
            other => CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Any => "any",
            CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Automatic => "automatic",
            CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Challenge => "challenge",
            CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn default() -> Self {
        CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutCardPaymentMethodOptionsRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Any,
            "automatic" => CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Automatic,
            "challenge" => CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Challenge,
            other => CheckoutCardPaymentMethodOptionsRequestThreeDSecure::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    BankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutCustomerBalancePaymentMethodOptionsFundingType::BankTransfer => "bank_transfer",
            CheckoutCustomerBalancePaymentMethodOptionsFundingType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutCustomerBalancePaymentMethodOptionsFundingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutCustomerBalancePaymentMethodOptionsFundingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_transfer" => CheckoutCustomerBalancePaymentMethodOptionsFundingType::BankTransfer,
            other => CheckoutCustomerBalancePaymentMethodOptionsFundingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Balances => "balances",
            CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Ownership => "ownership",
            CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::PaymentMethod => "payment_method",
            CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Transactions => "transactions",
            CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Balances,
            "ownership" => CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Ownership,
            "payment_method" => CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::PaymentMethod,
            "transactions" => CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Transactions,
            other => CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Balances => "balances",
            CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Ownership => "ownership",
            CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Transactions => "transactions",
            CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Balances,
            "ownership" => CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Ownership,
            "transactions" => CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Transactions,
            other => CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutSessionObject {
    CheckoutSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionObject::CheckoutSession => "checkout.session",
            CheckoutSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for CheckoutSessionObject {
    fn default() -> Self {
        CheckoutSessionObject::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checkout.session" => CheckoutSessionObject::CheckoutSession,
            other => CheckoutSessionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutSessionRedirectOnCompletion {
    Always,
    IfRequired,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionRedirectOnCompletion {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionRedirectOnCompletion::Always => "always",
            CheckoutSessionRedirectOnCompletion::IfRequired => "if_required",
            CheckoutSessionRedirectOnCompletion::Never => "never",
            CheckoutSessionRedirectOnCompletion::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionRedirectOnCompletion {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionRedirectOnCompletion::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CheckoutSessionRedirectOnCompletion::Always,
            "if_required" => CheckoutSessionRedirectOnCompletion::IfRequired,
            "never" => CheckoutSessionRedirectOnCompletion::Never,
            other => CheckoutSessionRedirectOnCompletion::Unknown(other.to_string()),
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
pub struct CheckoutSwishPaymentMethodOptions {
    #[doc = "The order reference that will be displayed to customers in the Swish application."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsAfterExpirationRecovery {
    #[doc = "Enables user redeemable promotion codes on the recovered Checkout Sessions."]
    pub allow_promotion_codes: Option<bool>,
    #[doc = "If `true`, a recovery URL will be generated to recover this Checkout Session if it expires before a successful transa..."]
    pub enabled: bool,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAfterExpirationRecovery {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_promotion_codes", self.allow_promotion_codes.as_ref());
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsBrandingSettingsIconType {
    File,
    Url,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsBrandingSettingsIconType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsBrandingSettingsIconType::File => "file",
            CreateCheckoutSessionParamsBrandingSettingsIconType::Url => "url",
            CreateCheckoutSessionParamsBrandingSettingsIconType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsBrandingSettingsIconType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsBrandingSettingsIconType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "file" => CreateCheckoutSessionParamsBrandingSettingsIconType::File,
            "url" => CreateCheckoutSessionParamsBrandingSettingsIconType::Url,
            other => CreateCheckoutSessionParamsBrandingSettingsIconType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBrandingSettingsIconType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsConsentCollectionTermsOfService {
    None,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsConsentCollectionTermsOfService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsConsentCollectionTermsOfService::None => "none",
            CreateCheckoutSessionParamsConsentCollectionTermsOfService::Required => "required",
            CreateCheckoutSessionParamsConsentCollectionTermsOfService::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsConsentCollectionTermsOfService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsConsentCollectionTermsOfService::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreateCheckoutSessionParamsConsentCollectionTermsOfService::None,
            "required" => CreateCheckoutSessionParamsConsentCollectionTermsOfService::Required,
            other => CreateCheckoutSessionParamsConsentCollectionTermsOfService::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsConsentCollectionTermsOfService {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsCustomFieldsItemType {
    Dropdown,
    Numeric,
    Text,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsCustomFieldsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsCustomFieldsItemType::Dropdown => "dropdown",
            CreateCheckoutSessionParamsCustomFieldsItemType::Numeric => "numeric",
            CreateCheckoutSessionParamsCustomFieldsItemType::Text => "text",
            CreateCheckoutSessionParamsCustomFieldsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsCustomFieldsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsCustomFieldsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dropdown" => CreateCheckoutSessionParamsCustomFieldsItemType::Dropdown,
            "numeric" => CreateCheckoutSessionParamsCustomFieldsItemType::Numeric,
            "text" => CreateCheckoutSessionParamsCustomFieldsItemType::Text,
            other => CreateCheckoutSessionParamsCustomFieldsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    Unnamed,
    ExcludeTax,
    IncludeInclusiveTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unnamed => "",
            CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
            CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unnamed,
            "exclude_tax" => CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax,
            "include_inclusive_tax" => CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax,
            other => CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Unspecified => "unspecified",
            CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Inclusive,
            "unspecified" => CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Unspecified,
            other => CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnGB,
    Es,
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    FrCA,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtBR,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhHK,
    ZhTW,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsLocale::Auto => "auto",
            CreateCheckoutSessionParamsLocale::Bg => "bg",
            CreateCheckoutSessionParamsLocale::Cs => "cs",
            CreateCheckoutSessionParamsLocale::Da => "da",
            CreateCheckoutSessionParamsLocale::De => "de",
            CreateCheckoutSessionParamsLocale::El => "el",
            CreateCheckoutSessionParamsLocale::En => "en",
            CreateCheckoutSessionParamsLocale::EnGB => "en-GB",
            CreateCheckoutSessionParamsLocale::Es => "es",
            CreateCheckoutSessionParamsLocale::Es419 => "es-419",
            CreateCheckoutSessionParamsLocale::Et => "et",
            CreateCheckoutSessionParamsLocale::Fi => "fi",
            CreateCheckoutSessionParamsLocale::Fil => "fil",
            CreateCheckoutSessionParamsLocale::Fr => "fr",
            CreateCheckoutSessionParamsLocale::FrCA => "fr-CA",
            CreateCheckoutSessionParamsLocale::Hr => "hr",
            CreateCheckoutSessionParamsLocale::Hu => "hu",
            CreateCheckoutSessionParamsLocale::Id => "id",
            CreateCheckoutSessionParamsLocale::It => "it",
            CreateCheckoutSessionParamsLocale::Ja => "ja",
            CreateCheckoutSessionParamsLocale::Ko => "ko",
            CreateCheckoutSessionParamsLocale::Lt => "lt",
            CreateCheckoutSessionParamsLocale::Lv => "lv",
            CreateCheckoutSessionParamsLocale::Ms => "ms",
            CreateCheckoutSessionParamsLocale::Mt => "mt",
            CreateCheckoutSessionParamsLocale::Nb => "nb",
            CreateCheckoutSessionParamsLocale::Nl => "nl",
            CreateCheckoutSessionParamsLocale::Pl => "pl",
            CreateCheckoutSessionParamsLocale::Pt => "pt",
            CreateCheckoutSessionParamsLocale::PtBR => "pt-BR",
            CreateCheckoutSessionParamsLocale::Ro => "ro",
            CreateCheckoutSessionParamsLocale::Ru => "ru",
            CreateCheckoutSessionParamsLocale::Sk => "sk",
            CreateCheckoutSessionParamsLocale::Sl => "sl",
            CreateCheckoutSessionParamsLocale::Sv => "sv",
            CreateCheckoutSessionParamsLocale::Th => "th",
            CreateCheckoutSessionParamsLocale::Tr => "tr",
            CreateCheckoutSessionParamsLocale::Vi => "vi",
            CreateCheckoutSessionParamsLocale::Zh => "zh",
            CreateCheckoutSessionParamsLocale::ZhHK => "zh-HK",
            CreateCheckoutSessionParamsLocale::ZhTW => "zh-TW",
            CreateCheckoutSessionParamsLocale::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsLocale::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateCheckoutSessionParamsLocale::Auto,
            "bg" => CreateCheckoutSessionParamsLocale::Bg,
            "cs" => CreateCheckoutSessionParamsLocale::Cs,
            "da" => CreateCheckoutSessionParamsLocale::Da,
            "de" => CreateCheckoutSessionParamsLocale::De,
            "el" => CreateCheckoutSessionParamsLocale::El,
            "en" => CreateCheckoutSessionParamsLocale::En,
            "en-GB" => CreateCheckoutSessionParamsLocale::EnGB,
            "es" => CreateCheckoutSessionParamsLocale::Es,
            "es-419" => CreateCheckoutSessionParamsLocale::Es419,
            "et" => CreateCheckoutSessionParamsLocale::Et,
            "fi" => CreateCheckoutSessionParamsLocale::Fi,
            "fil" => CreateCheckoutSessionParamsLocale::Fil,
            "fr" => CreateCheckoutSessionParamsLocale::Fr,
            "fr-CA" => CreateCheckoutSessionParamsLocale::FrCA,
            "hr" => CreateCheckoutSessionParamsLocale::Hr,
            "hu" => CreateCheckoutSessionParamsLocale::Hu,
            "id" => CreateCheckoutSessionParamsLocale::Id,
            "it" => CreateCheckoutSessionParamsLocale::It,
            "ja" => CreateCheckoutSessionParamsLocale::Ja,
            "ko" => CreateCheckoutSessionParamsLocale::Ko,
            "lt" => CreateCheckoutSessionParamsLocale::Lt,
            "lv" => CreateCheckoutSessionParamsLocale::Lv,
            "ms" => CreateCheckoutSessionParamsLocale::Ms,
            "mt" => CreateCheckoutSessionParamsLocale::Mt,
            "nb" => CreateCheckoutSessionParamsLocale::Nb,
            "nl" => CreateCheckoutSessionParamsLocale::Nl,
            "pl" => CreateCheckoutSessionParamsLocale::Pl,
            "pt" => CreateCheckoutSessionParamsLocale::Pt,
            "pt-BR" => CreateCheckoutSessionParamsLocale::PtBR,
            "ro" => CreateCheckoutSessionParamsLocale::Ro,
            "ru" => CreateCheckoutSessionParamsLocale::Ru,
            "sk" => CreateCheckoutSessionParamsLocale::Sk,
            "sl" => CreateCheckoutSessionParamsLocale::Sl,
            "sv" => CreateCheckoutSessionParamsLocale::Sv,
            "th" => CreateCheckoutSessionParamsLocale::Th,
            "tr" => CreateCheckoutSessionParamsLocale::Tr,
            "vi" => CreateCheckoutSessionParamsLocale::Vi,
            "zh" => CreateCheckoutSessionParamsLocale::Zh,
            "zh-HK" => CreateCheckoutSessionParamsLocale::ZhHK,
            "zh-TW" => CreateCheckoutSessionParamsLocale::ZhTW,
            other => CreateCheckoutSessionParamsLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLocale {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsMode {
    Payment,
    Setup,
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsMode::Payment => "payment",
            CreateCheckoutSessionParamsMode::Setup => "setup",
            CreateCheckoutSessionParamsMode::Subscription => "subscription",
            CreateCheckoutSessionParamsMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment" => CreateCheckoutSessionParamsMode::Payment,
            "setup" => CreateCheckoutSessionParamsMode::Setup,
            "subscription" => CreateCheckoutSessionParamsMode::Subscription,
            other => CreateCheckoutSessionParamsMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Automatic => "automatic",
            CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::AutomaticAsync => "automatic_async",
            CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Manual => "manual",
            CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Automatic,
            "automatic_async" => CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::AutomaticAsync,
            "manual" => CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Manual,
            other => CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem {
    Invoice,
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Invoice => "invoice",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Subscription => {
                "subscription"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Invoice,
            "subscription" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Subscription
            }
            other => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => {
                "combined"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => {
                "interval"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => {
                "sporadic"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "combined" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined
            }
            "interval" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval
            }
            "sporadic" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic
            }
            other => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer => {
                "eu_bank_transfer"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer => {
                "gb_bank_transfer"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer => {
                "jp_bank_transfer"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer => {
                "mx_bank_transfer"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::UsBankTransfer => {
                "us_bank_transfer"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "eu_bank_transfer" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer
            }
            "gb_bank_transfer" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer
            }
            "jp_bank_transfer" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer
            }
            "mx_bank_transfer" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer
            }
            "us_bank_transfer" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::UsBankTransfer
            }
            other => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    Unnamed,
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unnamed => "",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport => {
                "dependant_support"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government => "government",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan => "loan",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage => "mortgage",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other => "other",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension => "pension",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal => "personal",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail => "retail",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary => "salary",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax => "tax",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility => "utility",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unnamed,
            "dependant_support" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport
            }
            "government" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government,
            "loan" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan,
            "mortgage" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage,
            "other" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other,
            "pension" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension,
            "personal" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal,
            "retail" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail,
            "salary" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary,
            "tax" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax,
            "utility" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility,
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof {
    Always,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Always => "always",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Never => "never",
            CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Always,
            "never" => CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Never,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem {
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CV,
    CW,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MK,
    ML,
    MM,
    MN,
    MO,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SZ,
    TA,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
    ZZ,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AC => "AC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AD => "AD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AE => "AE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AF => "AF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AG => "AG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AI => "AI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AL => "AL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AM => "AM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AO => "AO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AQ => "AQ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AR => "AR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AT => "AT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AU => "AU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AW => "AW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AX => "AX",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AZ => "AZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BA => "BA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BB => "BB",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BD => "BD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BE => "BE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BF => "BF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BG => "BG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BH => "BH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BI => "BI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BJ => "BJ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BL => "BL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BM => "BM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BN => "BN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BO => "BO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BQ => "BQ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BR => "BR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BS => "BS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BT => "BT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BV => "BV",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BW => "BW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BY => "BY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BZ => "BZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CA => "CA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CD => "CD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CF => "CF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CG => "CG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CH => "CH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CI => "CI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CK => "CK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CL => "CL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CM => "CM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CN => "CN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CO => "CO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CR => "CR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CV => "CV",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CW => "CW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CY => "CY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CZ => "CZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DE => "DE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DJ => "DJ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DK => "DK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DM => "DM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DO => "DO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DZ => "DZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EC => "EC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EE => "EE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EG => "EG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EH => "EH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ER => "ER",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ES => "ES",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ET => "ET",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FI => "FI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FJ => "FJ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FK => "FK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FO => "FO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FR => "FR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GA => "GA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GB => "GB",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GD => "GD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GE => "GE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GF => "GF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GG => "GG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GH => "GH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GI => "GI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GL => "GL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GM => "GM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GN => "GN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GP => "GP",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GQ => "GQ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GR => "GR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GS => "GS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GT => "GT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GU => "GU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GW => "GW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GY => "GY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HK => "HK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HN => "HN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HR => "HR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HT => "HT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HU => "HU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ID => "ID",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IE => "IE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IL => "IL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IM => "IM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IN => "IN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IO => "IO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IQ => "IQ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IS => "IS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IT => "IT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JE => "JE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JM => "JM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JO => "JO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JP => "JP",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KE => "KE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KG => "KG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KH => "KH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KI => "KI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KM => "KM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KN => "KN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KR => "KR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KW => "KW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KY => "KY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KZ => "KZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LA => "LA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LB => "LB",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LC => "LC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LI => "LI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LK => "LK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LR => "LR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LS => "LS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LT => "LT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LU => "LU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LV => "LV",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LY => "LY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MA => "MA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MC => "MC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MD => "MD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ME => "ME",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MF => "MF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MG => "MG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MK => "MK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ML => "ML",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MM => "MM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MN => "MN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MO => "MO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MQ => "MQ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MR => "MR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MS => "MS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MT => "MT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MU => "MU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MV => "MV",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MW => "MW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MX => "MX",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MY => "MY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MZ => "MZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NA => "NA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NC => "NC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NE => "NE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NG => "NG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NI => "NI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NL => "NL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NO => "NO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NP => "NP",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NR => "NR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NU => "NU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NZ => "NZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::OM => "OM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PA => "PA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PE => "PE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PF => "PF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PG => "PG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PH => "PH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PK => "PK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PL => "PL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PM => "PM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PN => "PN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PR => "PR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PS => "PS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PT => "PT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PY => "PY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::QA => "QA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RE => "RE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RO => "RO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RS => "RS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RU => "RU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RW => "RW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SA => "SA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SB => "SB",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SC => "SC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SD => "SD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SE => "SE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SG => "SG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SH => "SH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SI => "SI",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SJ => "SJ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SK => "SK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SL => "SL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SM => "SM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SN => "SN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SO => "SO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SR => "SR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SS => "SS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ST => "ST",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SV => "SV",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SX => "SX",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SZ => "SZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TA => "TA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TC => "TC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TD => "TD",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TF => "TF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TG => "TG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TH => "TH",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TJ => "TJ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TK => "TK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TL => "TL",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TM => "TM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TN => "TN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TO => "TO",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TR => "TR",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TT => "TT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TV => "TV",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TW => "TW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TZ => "TZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UA => "UA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UG => "UG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::US => "US",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UY => "UY",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UZ => "UZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VA => "VA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VC => "VC",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VE => "VE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VG => "VG",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VN => "VN",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VU => "VU",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::WF => "WF",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::WS => "WS",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::XK => "XK",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::YE => "YE",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::YT => "YT",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZA => "ZA",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZM => "ZM",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZW => "ZW",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZZ => "ZZ",
            CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AC,
            "AD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AD,
            "AE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AE,
            "AF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AF,
            "AG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AG,
            "AI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AI,
            "AL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AL,
            "AM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AM,
            "AO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AO,
            "AQ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AQ,
            "AR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AR,
            "AT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AT,
            "AU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AU,
            "AW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AW,
            "AX" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AX,
            "AZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::AZ,
            "BA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BA,
            "BB" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BB,
            "BD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BD,
            "BE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BE,
            "BF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BF,
            "BG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BG,
            "BH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BH,
            "BI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BI,
            "BJ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BJ,
            "BL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BL,
            "BM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BM,
            "BN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BN,
            "BO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BO,
            "BQ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BQ,
            "BR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BR,
            "BS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BS,
            "BT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BT,
            "BV" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BV,
            "BW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BW,
            "BY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BY,
            "BZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::BZ,
            "CA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CA,
            "CD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CD,
            "CF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CF,
            "CG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CG,
            "CH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CH,
            "CI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CI,
            "CK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CK,
            "CL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CL,
            "CM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CM,
            "CN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CN,
            "CO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CO,
            "CR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CR,
            "CV" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CV,
            "CW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CW,
            "CY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CY,
            "CZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::CZ,
            "DE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DE,
            "DJ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DJ,
            "DK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DK,
            "DM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DM,
            "DO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DO,
            "DZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::DZ,
            "EC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EC,
            "EE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EE,
            "EG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EG,
            "EH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::EH,
            "ER" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ER,
            "ES" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ES,
            "ET" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ET,
            "FI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FI,
            "FJ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FJ,
            "FK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FK,
            "FO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FO,
            "FR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::FR,
            "GA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GA,
            "GB" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GB,
            "GD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GD,
            "GE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GE,
            "GF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GF,
            "GG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GG,
            "GH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GH,
            "GI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GI,
            "GL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GL,
            "GM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GM,
            "GN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GN,
            "GP" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GP,
            "GQ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GQ,
            "GR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GR,
            "GS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GS,
            "GT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GT,
            "GU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GU,
            "GW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GW,
            "GY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::GY,
            "HK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HK,
            "HN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HN,
            "HR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HR,
            "HT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HT,
            "HU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::HU,
            "ID" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ID,
            "IE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IE,
            "IL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IL,
            "IM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IM,
            "IN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IN,
            "IO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IO,
            "IQ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IQ,
            "IS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IS,
            "IT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::IT,
            "JE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JE,
            "JM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JM,
            "JO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JO,
            "JP" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::JP,
            "KE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KE,
            "KG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KG,
            "KH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KH,
            "KI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KI,
            "KM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KM,
            "KN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KN,
            "KR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KR,
            "KW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KW,
            "KY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KY,
            "KZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::KZ,
            "LA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LA,
            "LB" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LB,
            "LC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LC,
            "LI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LI,
            "LK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LK,
            "LR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LR,
            "LS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LS,
            "LT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LT,
            "LU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LU,
            "LV" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LV,
            "LY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::LY,
            "MA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MA,
            "MC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MC,
            "MD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MD,
            "ME" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ME,
            "MF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MF,
            "MG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MG,
            "MK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MK,
            "ML" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ML,
            "MM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MM,
            "MN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MN,
            "MO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MO,
            "MQ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MQ,
            "MR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MR,
            "MS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MS,
            "MT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MT,
            "MU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MU,
            "MV" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MV,
            "MW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MW,
            "MX" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MX,
            "MY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MY,
            "MZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::MZ,
            "NA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NA,
            "NC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NC,
            "NE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NE,
            "NG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NG,
            "NI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NI,
            "NL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NL,
            "NO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NO,
            "NP" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NP,
            "NR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NR,
            "NU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NU,
            "NZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::NZ,
            "OM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::OM,
            "PA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PA,
            "PE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PE,
            "PF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PF,
            "PG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PG,
            "PH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PH,
            "PK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PK,
            "PL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PL,
            "PM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PM,
            "PN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PN,
            "PR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PR,
            "PS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PS,
            "PT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PT,
            "PY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::PY,
            "QA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::QA,
            "RE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RE,
            "RO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RO,
            "RS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RS,
            "RU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RU,
            "RW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::RW,
            "SA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SA,
            "SB" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SB,
            "SC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SC,
            "SD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SD,
            "SE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SE,
            "SG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SG,
            "SH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SH,
            "SI" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SI,
            "SJ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SJ,
            "SK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SK,
            "SL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SL,
            "SM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SM,
            "SN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SN,
            "SO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SO,
            "SR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SR,
            "SS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SS,
            "ST" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ST,
            "SV" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SV,
            "SX" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SX,
            "SZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::SZ,
            "TA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TA,
            "TC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TC,
            "TD" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TD,
            "TF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TF,
            "TG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TG,
            "TH" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TH,
            "TJ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TJ,
            "TK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TK,
            "TL" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TL,
            "TM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TM,
            "TN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TN,
            "TO" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TO,
            "TR" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TR,
            "TT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TT,
            "TV" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TV,
            "TW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TW,
            "TZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::TZ,
            "UA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UA,
            "UG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UG,
            "US" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::US,
            "UY" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UY,
            "UZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::UZ,
            "VA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VA,
            "VC" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VC,
            "VE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VE,
            "VG" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VG,
            "VN" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VN,
            "VU" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::VU,
            "WF" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::WF,
            "WS" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::WS,
            "XK" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::XK,
            "YE" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::YE,
            "YT" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::YT,
            "ZA" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZA,
            "ZM" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZM,
            "ZW" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZW,
            "ZZ" => CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::ZZ,
            other => {
                CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
    Subscribe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsSubmitType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsSubmitType::Auto => "auto",
            CreateCheckoutSessionParamsSubmitType::Book => "book",
            CreateCheckoutSessionParamsSubmitType::Donate => "donate",
            CreateCheckoutSessionParamsSubmitType::Pay => "pay",
            CreateCheckoutSessionParamsSubmitType::Subscribe => "subscribe",
            CreateCheckoutSessionParamsSubmitType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsSubmitType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsSubmitType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateCheckoutSessionParamsSubmitType::Auto,
            "book" => CreateCheckoutSessionParamsSubmitType::Book,
            "donate" => CreateCheckoutSessionParamsSubmitType::Donate,
            "pay" => CreateCheckoutSessionParamsSubmitType::Pay,
            "subscribe" => CreateCheckoutSessionParamsSubmitType::Subscribe,
            other => CreateCheckoutSessionParamsSubmitType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubmitType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                "create_invoice"
            }
            CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
            CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
            "create_invoice" => {
                CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice
            }
            "pause" => CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause,
            other => CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsTaxIdCollectionRequired {
    IfSupported,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsTaxIdCollectionRequired {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsTaxIdCollectionRequired::IfSupported => "if_supported",
            CreateCheckoutSessionParamsTaxIdCollectionRequired::Never => "never",
            CreateCheckoutSessionParamsTaxIdCollectionRequired::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsTaxIdCollectionRequired {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsTaxIdCollectionRequired::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_supported" => CreateCheckoutSessionParamsTaxIdCollectionRequired::IfSupported,
            "never" => CreateCheckoutSessionParamsTaxIdCollectionRequired::Never,
            other => CreateCheckoutSessionParamsTaxIdCollectionRequired::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsTaxIdCollectionRequired {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
pub struct PaymentPagesCheckoutSessionAdaptivePricing {
    #[doc = "If enabled, Adaptive Pricing is available on eligible sessions..."]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionBusinessName {
    #[doc = "Indicates whether business name collection is enabled for the session"]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Whether the customer is required to complete the field before completing the Checkout Session."]
    #[json(default)]
    pub optional: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentCollectionPromotions {
    Auto,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionConsentCollectionPromotions {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionConsentCollectionPromotions::Auto => "auto",
            PaymentPagesCheckoutSessionConsentCollectionPromotions::None => "none",
            PaymentPagesCheckoutSessionConsentCollectionPromotions::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionConsentCollectionPromotions {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionConsentCollectionPromotions::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => PaymentPagesCheckoutSessionConsentCollectionPromotions::Auto,
            "none" => PaymentPagesCheckoutSessionConsentCollectionPromotions::None,
            other => PaymentPagesCheckoutSessionConsentCollectionPromotions::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    None,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionConsentCollectionTermsOfService::None => "none",
            PaymentPagesCheckoutSessionConsentCollectionTermsOfService::Required => "required",
            PaymentPagesCheckoutSessionConsentCollectionTermsOfService::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionConsentCollectionTermsOfService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionConsentCollectionTermsOfService::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => PaymentPagesCheckoutSessionConsentCollectionTermsOfService::None,
            "required" => PaymentPagesCheckoutSessionConsentCollectionTermsOfService::Required,
            other => PaymentPagesCheckoutSessionConsentCollectionTermsOfService::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionConsentPromotions {
    OptIn,
    OptOut,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionConsentPromotions {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionConsentPromotions::OptIn => "opt_in",
            PaymentPagesCheckoutSessionConsentPromotions::OptOut => "opt_out",
            PaymentPagesCheckoutSessionConsentPromotions::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionConsentPromotions {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionConsentPromotions::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "opt_in" => PaymentPagesCheckoutSessionConsentPromotions::OptIn,
            "opt_out" => PaymentPagesCheckoutSessionConsentPromotions::OptOut,
            other => PaymentPagesCheckoutSessionConsentPromotions::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionCustomFieldsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionCustomFieldsType::Dropdown => "dropdown",
            PaymentPagesCheckoutSessionCustomFieldsType::Numeric => "numeric",
            PaymentPagesCheckoutSessionCustomFieldsType::Text => "text",
            PaymentPagesCheckoutSessionCustomFieldsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionCustomFieldsType {
    fn default() -> Self {
        PaymentPagesCheckoutSessionCustomFieldsType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionCustomFieldsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionCustomFieldsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dropdown" => PaymentPagesCheckoutSessionCustomFieldsType::Dropdown,
            "numeric" => PaymentPagesCheckoutSessionCustomFieldsType::Numeric,
            "text" => PaymentPagesCheckoutSessionCustomFieldsType::Text,
            other => PaymentPagesCheckoutSessionCustomFieldsType::Unknown(other.to_string()),
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
pub struct PaymentPagesCheckoutSessionCustomTextPosition {
    #[doc = "Text can be up to 1200 characters in length."]
    #[json(default)]
    pub message: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    #[doc = "Indicates whether phone number collection is enabled for the session"]
    #[json(default)]
    pub enabled: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::AmericanExpress => {
                "american_express"
            }
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork => {
                "discover_global_network"
            }
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Mastercard => {
                "mastercard"
            }
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Visa => "visa",
            PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "american_express" => {
                PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::AmericanExpress
            }
            "discover_global_network" => {
                PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork
            }
            "mastercard" => {
                PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Mastercard
            }
            "visa" => PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Visa,
            other => PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckoutSessionParamsCollectedInformationShippingDetailsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: String,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: String,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsCollectedInformationShippingDetailsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field("country", &self.country);
        form.field("line1", &self.line1);
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}
