#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsMandateDataCustomerAcceptanceOnline {
    #[doc = "The IP address from which the Mandate was accepted by the customer."]
    pub ip_address: String,
    #[doc = "The user agent of the browser from which the Mandate was accepted by the customer."]
    pub user_agent: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsMandateDataCustomerAcceptanceOnline {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("ip_address", &self.ip_address);
        form.field("user_agent", &self.user_agent);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataAcssDebit {
    #[doc = "Customer's bank account number."]
    pub account_number: String,
    #[doc = "Institution number of the customer's bank."]
    pub institution_number: String,
    #[doc = "Transit number of the customer's bank."]
    pub transit_number: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_number", &self.account_number);
        form.field("institution_number", &self.institution_number);
        form.field("transit_number", &self.transit_number);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataEpsBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataEpsBank::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BankAustria => "bank_austria",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BankhausCarlSpangler => "bankhaus_carl_spangler",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BawagPskAg => "bawag_psk_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BksBankAg => "bks_bank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::Dolomitenbank => "dolomitenbank",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::EasybankAg => "easybank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::MarchfelderBank => "marchfelder_bank",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::OberbankAg => "oberbank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            CreateSetupIntentParamsPaymentMethodDataEpsBank::SchoellerbankAg => "schoellerbank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::SpardaBankWien => "sparda_bank_wien",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::VolksbankGruppe => "volksbank_gruppe",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::VrBankBraunau => "vr_bank_braunau",
            CreateSetupIntentParamsPaymentMethodDataEpsBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataEpsBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataEpsBank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "arzte_und_apotheker_bank" => CreateSetupIntentParamsPaymentMethodDataEpsBank::ArzteUndApothekerBank,
            "austrian_anadi_bank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::AustrianAnadiBankAg,
            "bank_austria" => CreateSetupIntentParamsPaymentMethodDataEpsBank::BankAustria,
            "bankhaus_carl_spangler" => CreateSetupIntentParamsPaymentMethodDataEpsBank::BankhausCarlSpangler,
            "bankhaus_schelhammer_und_schattera_ag" => {
                CreateSetupIntentParamsPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg
            }
            "bawag_psk_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::BawagPskAg,
            "bks_bank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::BksBankAg,
            "brull_kallmus_bank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::BrullKallmusBankAg,
            "btv_vier_lander_bank" => CreateSetupIntentParamsPaymentMethodDataEpsBank::BtvVierLanderBank,
            "capital_bank_grawe_gruppe_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg,
            "deutsche_bank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::DeutscheBankAg,
            "dolomitenbank" => CreateSetupIntentParamsPaymentMethodDataEpsBank::Dolomitenbank,
            "easybank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::EasybankAg,
            "erste_bank_und_sparkassen" => CreateSetupIntentParamsPaymentMethodDataEpsBank::ErsteBankUndSparkassen,
            "hypo_alpeadriabank_international_ag" => {
                CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg
            }
            "hypo_bank_burgenland_aktiengesellschaft" => {
                CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft
            }
            "hypo_noe_lb_fur_niederosterreich_u_wien" => {
                CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien
            }
            "hypo_oberosterreich_salzburg_steiermark" => {
                CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark
            }
            "hypo_tirol_bank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoTirolBankAg,
            "hypo_vorarlberg_bank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::HypoVorarlbergBankAg,
            "marchfelder_bank" => CreateSetupIntentParamsPaymentMethodDataEpsBank::MarchfelderBank,
            "oberbank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::OberbankAg,
            "raiffeisen_bankengruppe_osterreich" => {
                CreateSetupIntentParamsPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich
            }
            "schoellerbank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::SchoellerbankAg,
            "sparda_bank_wien" => CreateSetupIntentParamsPaymentMethodDataEpsBank::SpardaBankWien,
            "volksbank_gruppe" => CreateSetupIntentParamsPaymentMethodDataEpsBank::VolksbankGruppe,
            "volkskreditbank_ag" => CreateSetupIntentParamsPaymentMethodDataEpsBank::VolkskreditbankAg,
            "vr_bank_braunau" => CreateSetupIntentParamsPaymentMethodDataEpsBank::VrBankBraunau,
            other => CreateSetupIntentParamsPaymentMethodDataEpsBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataEpsBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataIdealBank {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataIdealBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataIdealBank::AbnAmro => "abn_amro",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Adyen => "adyen",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::AsnBank => "asn_bank",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Bunq => "bunq",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Buut => "buut",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Finom => "finom",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Handelsbanken => "handelsbanken",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Ing => "ing",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Knab => "knab",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Mollie => "mollie",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Moneyou => "moneyou",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::N26 => "n26",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Nn => "nn",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Rabobank => "rabobank",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Regiobank => "regiobank",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Revolut => "revolut",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::SnsBank => "sns_bank",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::TriodosBank => "triodos_bank",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::VanLanschot => "van_lanschot",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Yoursafe => "yoursafe",
            CreateSetupIntentParamsPaymentMethodDataIdealBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataIdealBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataIdealBank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abn_amro" => CreateSetupIntentParamsPaymentMethodDataIdealBank::AbnAmro,
            "adyen" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Adyen,
            "asn_bank" => CreateSetupIntentParamsPaymentMethodDataIdealBank::AsnBank,
            "bunq" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Bunq,
            "buut" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Buut,
            "finom" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Finom,
            "handelsbanken" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Handelsbanken,
            "ing" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Ing,
            "knab" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Knab,
            "mollie" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Mollie,
            "moneyou" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Moneyou,
            "n26" => CreateSetupIntentParamsPaymentMethodDataIdealBank::N26,
            "nn" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Nn,
            "rabobank" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Rabobank,
            "regiobank" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Regiobank,
            "revolut" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Revolut,
            "sns_bank" => CreateSetupIntentParamsPaymentMethodDataIdealBank::SnsBank,
            "triodos_bank" => CreateSetupIntentParamsPaymentMethodDataIdealBank::TriodosBank,
            "van_lanschot" => CreateSetupIntentParamsPaymentMethodDataIdealBank::VanLanschot,
            "yoursafe" => CreateSetupIntentParamsPaymentMethodDataIdealBank::Yoursafe,
            other => CreateSetupIntentParamsPaymentMethodDataIdealBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataIdealBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataKlarnaDob {
    #[doc = "The day of birth, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of birth, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of birth."]
    pub year: i64,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataKlarnaDob {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day", &self.day);
        form.field("month", &self.month);
        form.field("year", &self.year);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Fixed => "fixed",
            CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Maximum => "maximum",
            CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Fixed,
            "maximum" => CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Maximum,
            other => CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Day => "day",
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Month => "month",
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Sporadic => "sporadic",
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Week => "week",
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Year => "year",
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Day,
            "month" => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Month,
            "sporadic" => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Sporadic,
            "week" => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Week,
            "year" => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Year,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsPaypal {
    #[doc = "The PayPal Billing Agreement ID (BAID)."]
    pub billing_agreement_id: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_agreement_id", self.billing_agreement_id.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    Unnamed,
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unnamed => "",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Fixed => "fixed",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Maximum => "maximum",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unnamed,
            "fixed" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Fixed,
            "maximum" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Maximum,
            other => {
                CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances => {
                "balances"
            }
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership => {
                "ownership"
            }
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions => {
                "transactions"
            }
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json>
    for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => {
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances
            }
            "ownership" => {
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership
            }
            "transactions" => {
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions
            }
            other => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    Ach,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Ach => "ach",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::UsDomesticWire => {
                "us_domestic_wire"
            }
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Ach,
            "us_domestic_wire" => {
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::UsDomesticWire
            }
            other => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
pub enum SetupIntentFlowDirectionsItem {
    Inbound,
    Outbound,
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
impl SetupIntentFlowDirectionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentFlowDirectionsItem::Inbound => "inbound",
            SetupIntentFlowDirectionsItem::Outbound => "outbound",
            SetupIntentFlowDirectionsItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentFlowDirectionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentFlowDirectionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound" => SetupIntentFlowDirectionsItem::Inbound,
            "outbound" => SetupIntentFlowDirectionsItem::Outbound,
            other => SetupIntentFlowDirectionsItem::Unknown(other.to_string()),
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
pub struct SetupIntentNextActionPixDisplayQrCode {
    #[doc = "The raw data string used to generate QR code, it should be used together with QR code library."]
    #[json(default)]
    pub data: String,
    #[doc = "The date (unix timestamp) when the PIX expires."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "The URL to the hosted pix instructions page, which allows customers to view the pix QR code."]
    #[json(default)]
    pub hosted_instructions_url: String,
    #[doc = "The image_url_png string used to render png QR code"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The image_url_svg string used to render svg QR code"]
    #[json(default)]
    pub image_url_svg: String,
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
pub enum SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
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
impl SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
            SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic,
            "instant" => SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant,
            "microdeposits" => SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits,
            other => SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(other.to_string()),
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
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    India,
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
impl SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India => "india",
            SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "india" => SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India,
            other => SetupIntentPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsKlarna {
    #[doc = "The currency of the setup intent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Preferred locale of the Klarna checkout page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
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
pub enum SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    OffSession,
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
impl SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsBlikType::OffSession => "off_session",
            SetupIntentPaymentMethodOptionsMandateOptionsBlikType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsMandateOptionsBlikType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsMandateOptionsBlikType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => SetupIntentPaymentMethodOptionsMandateOptionsBlikType::OffSession,
            other => SetupIntentPaymentMethodOptionsMandateOptionsBlikType::Unknown(other.to_string()),
        })
    }
}
