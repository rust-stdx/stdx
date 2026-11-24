#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct AdvanceTestHelperTestClockParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The time to advance the test clock."]
    pub frozen_time: i64,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for AdvanceTestHelperTestClockParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("frozen_time", &self.frozen_time);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataBacsDebit {
    #[doc = "Account number of the bank account that the funds will be debited from."]
    pub account_number: Option<String>,
    #[doc = "Sort code of the bank account."]
    pub sort_code: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("sort_code", self.sort_code.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Company => "company",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Individual => "individual",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Company,
            "individual" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Individual,
            other => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    BnpParibas,
    Bsn,
    Cimb,
    Citibank,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    MbsbBank,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::AffinBank => "affin_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Agrobank => "agrobank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::AllianceBank => "alliance_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Ambank => "ambank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankIslam => "bank_islam",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankMuamalat => "bank_muamalat",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankOfChina => "bank_of_china",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankRakyat => "bank_rakyat",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BnpParibas => "bnp_paribas",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Bsn => "bsn",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Cimb => "cimb",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Citibank => "citibank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::DeutscheBank => "deutsche_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::HongLeongBank => "hong_leong_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Hsbc => "hsbc",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Kfh => "kfh",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Maybank2e => "maybank2e",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Maybank2u => "maybank2u",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::MbsbBank => "mbsb_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Ocbc => "ocbc",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::PbEnterprise => "pb_enterprise",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::PublicBank => "public_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Rhb => "rhb",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::StandardChartered => "standard_chartered",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Uob => "uob",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "affin_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::AffinBank,
            "agrobank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Agrobank,
            "alliance_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::AllianceBank,
            "ambank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Ambank,
            "bank_islam" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankIslam,
            "bank_muamalat" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankMuamalat,
            "bank_of_china" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankOfChina,
            "bank_rakyat" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BankRakyat,
            "bnp_paribas" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::BnpParibas,
            "bsn" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Bsn,
            "cimb" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Cimb,
            "citibank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Citibank,
            "deutsche_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::DeutscheBank,
            "hong_leong_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::HongLeongBank,
            "hsbc" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Hsbc,
            "kfh" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Kfh,
            "maybank2e" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Maybank2e,
            "maybank2u" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Maybank2u,
            "mbsb_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::MbsbBank,
            "ocbc" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Ocbc,
            "pb_enterprise" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::PbEnterprise,
            "public_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::PublicBank,
            "rhb" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Rhb,
            "standard_chartered" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::StandardChartered,
            "uob" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Uob,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank {
    AbnAmro,
    Adyen,
    AsnBank,
    Bunq,
    Buut,
    Finom,
    Handelsbanken,
    Ing,
    Knab,
    Mollie,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::AbnAmro => "abn_amro",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Adyen => "adyen",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::AsnBank => "asn_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Bunq => "bunq",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Buut => "buut",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Finom => "finom",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Handelsbanken => "handelsbanken",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Ing => "ing",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Knab => "knab",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Mollie => "mollie",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Moneyou => "moneyou",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::N26 => "n26",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Nn => "nn",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Rabobank => "rabobank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Regiobank => "regiobank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Revolut => "revolut",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::SnsBank => "sns_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::TriodosBank => "triodos_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::VanLanschot => "van_lanschot",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Yoursafe => "yoursafe",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abn_amro" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::AbnAmro,
            "adyen" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Adyen,
            "asn_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::AsnBank,
            "bunq" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Bunq,
            "buut" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Buut,
            "finom" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Finom,
            "handelsbanken" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Handelsbanken,
            "ing" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Ing,
            "knab" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Knab,
            "mollie" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Mollie,
            "moneyou" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Moneyou,
            "n26" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::N26,
            "nn" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Nn,
            "rabobank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Rabobank,
            "regiobank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Regiobank,
            "revolut" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Revolut,
            "sns_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::SnsBank,
            "triodos_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::TriodosBank,
            "van_lanschot" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::VanLanschot,
            "yoursafe" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Yoursafe,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsAmountDetails {
    #[doc = "The ATM withdrawal fee."]
    pub atm_fee: Option<i64>,
    #[doc = "The amount of cash requested by the cardholder."]
    pub cashback_amount: Option<i64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsAmountDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("atm_fee", self.atm_fee.as_ref());
        form.field_opt("cashback_amount", self.cashback_amount.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel {
    #[doc = "Gross fuel amount that should equal Fuel Volume multiplied by Fuel Unit Cost, inclusive of taxes."]
    pub gross_amount_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gross_amount_decimal", self.gross_amount_decimal.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsFuelUnit {
    ChargingMinute,
    ImperialGallon,
    Kilogram,
    KilowattHour,
    Liter,
    Other,
    Pound,
    UsGallon,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsFuelUnit {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::ChargingMinute => "charging_minute",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::ImperialGallon => "imperial_gallon",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::Kilogram => "kilogram",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::KilowattHour => "kilowatt_hour",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::Liter => "liter",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::Other => "other",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::Pound => "pound",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::UsGallon => "us_gallon",
            CreateTestHelperIssuingAuthorizationParamsFuelUnit::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsFuelUnit {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsFuelUnit::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charging_minute" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::ChargingMinute,
            "imperial_gallon" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::ImperialGallon,
            "kilogram" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::Kilogram,
            "kilowatt_hour" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::KilowattHour,
            "liter" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::Liter,
            "other" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::Other,
            "pound" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::Pound,
            "us_gallon" => CreateTestHelperIssuingAuthorizationParamsFuelUnit::UsGallon,
            other => CreateTestHelperIssuingAuthorizationParamsFuelUnit::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFuelUnit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel {
    Elevated,
    Highest,
    Low,
    Normal,
    NotAssessed,
    UnknownValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Elevated => "elevated",
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Highest => "highest",
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Low => "low",
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Normal => "normal",
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::NotAssessed => "not_assessed",
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::UnknownValue => "unknown",
            CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "elevated" => CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Elevated,
            "highest" => CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Highest,
            "low" => CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Low,
            "normal" => CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Normal,
            "not_assessed" => CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::NotAssessed,
            "unknown" => CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::UnknownValue,
            other => {
                CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy {
    Acquirer,
    Issuer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Acquirer => {
                "acquirer"
            }
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Issuer => {
                "issuer"
            }
            CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json>
    for CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acquirer" => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Acquirer
            }
            "issuer" => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Issuer
            }
            other => {
                CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode
    for CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperTestClockParams {
    #[doc = "Existing customer this test clock will be attached to."]
    pub customer: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The initial frozen time for this test clock."]
    pub frozen_time: i64,
    #[doc = "The name for this test clock."]
    pub name: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTestClockParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("frozen_time", &self.frozen_time);
        form.field_opt("name", self.name.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsUsBankAccount {
    #[doc = "The bank account holder's name."]
    pub account_holder_name: Option<String>,
    #[doc = "The bank account number."]
    pub account_number: Option<String>,
    #[doc = "The bank account's routing number."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode
    for CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsUsBankAccount
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ExpireTestHelperIssuingAuthorizationParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ExpireTestHelperIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::GeographicLocation => {
                "geographic_location"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Inappropriate => {
                "inappropriate"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::NetworkName => {
                "network_name"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::NonFiatCurrency => {
                "non_fiat_currency"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Other => "other",
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::OtherEntity => {
                "other_entity"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::PromotionalMaterial => {
                "promotional_material"
            }
            RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "geographic_location" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::GeographicLocation
            }
            "inappropriate" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Inappropriate
            }
            "network_name" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::NetworkName
            }
            "non_fiat_currency" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::NonFiatCurrency
            }
            "other" => RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Other,
            "other_entity" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::OtherEntity
            }
            "promotional_material" => {
                RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::PromotionalMaterial
            }
            other => RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct RespondTestHelperIssuingAuthorizationFraudChallengeParams {
    #[doc = "Whether to simulate the user confirming that the transaction was legitimate (true) or telling Stripe that it was frau..."]
    pub confirmed: bool,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for RespondTestHelperIssuingAuthorizationFraudChallengeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("confirmed", &self.confirmed);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
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
#[derive(Debug, Clone, PartialEq)]
pub enum TestHelpersTestClockObject {
    TestHelpersTestClock,
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
impl TestHelpersTestClockObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TestHelpersTestClockObject::TestHelpersTestClock => "test_helpers.test_clock",
            TestHelpersTestClockObject::Unknown(value) => value,
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
impl Default for TestHelpersTestClockObject {
    fn default() -> Self {
        TestHelpersTestClockObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for TestHelpersTestClockObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TestHelpersTestClockObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "test_helpers.test_clock" => TestHelpersTestClockObject::TestHelpersTestClock,
            other => TestHelpersTestClockObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsUsDomesticWire {
    #[doc = "CHIPS System Sequence Number (SSN) for funds sent over the `us_domestic_wire` network."]
    pub chips: Option<String>,
    #[doc = "IMAD for funds sent over the `us_domestic_wire` network."]
    pub imad: Option<String>,
    #[doc = "OMAD for funds sent over the `us_domestic_wire` network."]
    pub omad: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsUsDomesticWire {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("chips", self.chips.as_ref());
        form.field_opt("imad", self.imad.as_ref());
        form.field_opt("omad", self.omad.as_ref());
    }
}
