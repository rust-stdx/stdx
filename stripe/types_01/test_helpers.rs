#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataBoleto {
    #[doc = "The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers)"]
    pub tax_id: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("tax_id", &self.tax_id);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding {
    Card,
    Points,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Card => "card",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Points => "points",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Card,
            "points" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Points,
            other => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::AliorBank => "alior_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankMillennium => "bank_millennium",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankPekaoSa => "bank_pekao_sa",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Blik => "blik",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BnpParibas => "bnp_paribas",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Boz => "boz",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::CitiHandlowy => "citi_handlowy",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::CreditAgricole => "credit_agricole",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Envelobank => "envelobank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::EtransferPocztowy24 => {
                "etransfer_pocztowy24"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::GetinBank => "getin_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Ideabank => "ideabank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Ing => "ing",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Inteligo => "inteligo",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::MbankMtransfer => "mbank_mtransfer",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::NestPrzelew => "nest_przelew",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::NoblePay => "noble_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::PbacZIpko => "pbac_z_ipko",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::PlusBank => "plus_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::SantanderPrzelew24 => {
                "santander_przelew24"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::TmobileUsbugiBankowe => {
                "tmobile_usbugi_bankowe"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::ToyotaBank => "toyota_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Velobank => "velobank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::VolkswagenBank => "volkswagen_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alior_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::AliorBank,
            "bank_millennium" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankMillennium,
            "bank_nowy_bfg_sa" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankNowyBfgSa,
            "bank_pekao_sa" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankPekaoSa,
            "banki_spbdzielcze" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BankiSpbdzielcze,
            "blik" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Blik,
            "bnp_paribas" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::BnpParibas,
            "boz" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Boz,
            "citi_handlowy" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::CitiHandlowy,
            "credit_agricole" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::CreditAgricole,
            "envelobank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Envelobank,
            "etransfer_pocztowy24" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::EtransferPocztowy24
            }
            "getin_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::GetinBank,
            "ideabank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Ideabank,
            "ing" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Ing,
            "inteligo" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Inteligo,
            "mbank_mtransfer" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::MbankMtransfer,
            "nest_przelew" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::NestPrzelew,
            "noble_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::NoblePay,
            "pbac_z_ipko" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::PbacZIpko,
            "plus_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::PlusBank,
            "santander_przelew24" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::SantanderPrzelew24
            }
            "tmobile_usbugi_bankowe" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::TmobileUsbugiBankowe
            }
            "toyota_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::ToyotaBank,
            "velobank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Velobank,
            "volkswagen_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::VolkswagenBank,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Fixed => "fixed",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Maximum => "maximum",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json>
    for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Fixed,
            "maximum" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Maximum,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Month => "month",
            CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json>
    for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "month" => CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Month,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode
    for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult {
    AttemptAcknowledged,
    Authenticated,
    Failed,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::AttemptAcknowledged => {
                "attempt_acknowledged"
            }
            CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Authenticated => {
                "authenticated"
            }
            CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Failed => "failed",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Required => "required",
            CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "attempt_acknowledged" => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::AttemptAcknowledged
            }
            "authenticated" => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Authenticated
            }
            "failed" => CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Failed,
            "required" => CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Required,
            other => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperTreasuryReceivedDebitParamsNetwork {
    Ach,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperTreasuryReceivedDebitParamsNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperTreasuryReceivedDebitParamsNetwork::Ach => "ach",
            CreateTestHelperTreasuryReceivedDebitParamsNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperTreasuryReceivedDebitParamsNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperTreasuryReceivedDebitParamsNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => CreateTestHelperTreasuryReceivedDebitParamsNetwork::Ach,
            other => CreateTestHelperTreasuryReceivedDebitParamsNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedDebitParamsNetwork {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::AccountClosed => "account_closed",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::AccountFrozen => "account_frozen",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::BankAccountRestricted => {
                "bank_account_restricted"
            }
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::BankOwnershipChanged => {
                "bank_ownership_changed"
            }
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::DebitNotAuthorized => "debit_not_authorized",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::IncorrectAccountHolderAddress => {
                "incorrect_account_holder_address"
            }
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::IncorrectAccountHolderName => {
                "incorrect_account_holder_name"
            }
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::IncorrectAccountHolderTaxId => {
                "incorrect_account_holder_tax_id"
            }
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::InsufficientFunds => "insufficient_funds",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::InvalidAccountNumber => {
                "invalid_account_number"
            }
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::InvalidCurrency => "invalid_currency",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::NoAccount => "no_account",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::Other => "other",
            FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_closed" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::AccountClosed,
            "account_frozen" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::AccountFrozen,
            "bank_account_restricted" => {
                FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::BankAccountRestricted
            }
            "bank_ownership_changed" => {
                FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::BankOwnershipChanged
            }
            "debit_not_authorized" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::DebitNotAuthorized,
            "incorrect_account_holder_address" => {
                FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::IncorrectAccountHolderAddress
            }
            "incorrect_account_holder_name" => {
                FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::IncorrectAccountHolderName
            }
            "incorrect_account_holder_tax_id" => {
                FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::IncorrectAccountHolderTaxId
            }
            "insufficient_funds" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::InsufficientFunds,
            "invalid_account_number" => {
                FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::InvalidAccountNumber
            }
            "invalid_currency" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::InvalidCurrency,
            "no_account" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::NoAccount,
            "other" => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::Other,
            other => FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct IncrementTestHelperIssuingAuthorizationParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The amount to increment the authorization by."]
    pub increment_amount: i64,
    #[doc = "If set `true`, you may provide amount..."]
    pub is_amount_controllable: Option<bool>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for IncrementTestHelperIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("increment_amount", &self.increment_amount);
        form.field_opt("is_amount_controllable", self.is_amount_controllable.as_ref());
    }
}
