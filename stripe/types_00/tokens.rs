#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompanyDirectorshipDeclaration {
    #[doc = "The Unix timestamp marking when the directorship declaration attestation was made."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the directorship declaration attestation was made."]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the directorship declaration attestation was made."]
    pub user_agent: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyDirectorshipDeclaration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
        form.field_opt("user_agent", self.user_agent.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompanyVerificationDocument {
    #[doc = "The back of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of..."]
    pub back: Option<String>,
    #[doc = "The front of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value o..."]
    pub front: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyVerificationDocument {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("back", self.back.as_ref());
        form.field_opt("front", self.front.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTokenParamsCardNetworksPreferred {
    CartesBancaires,
    Mastercard,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tokens")]
impl CreateTokenParamsCardNetworksPreferred {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTokenParamsCardNetworksPreferred::CartesBancaires => "cartes_bancaires",
            CreateTokenParamsCardNetworksPreferred::Mastercard => "mastercard",
            CreateTokenParamsCardNetworksPreferred::Visa => "visa",
            CreateTokenParamsCardNetworksPreferred::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tokens")]
impl<'json> json::Decode<'json> for CreateTokenParamsCardNetworksPreferred {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTokenParamsCardNetworksPreferred::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cartes_bancaires" => CreateTokenParamsCardNetworksPreferred::CartesBancaires,
            "mastercard" => CreateTokenParamsCardNetworksPreferred::Mastercard,
            "visa" => CreateTokenParamsCardNetworksPreferred::Visa,
            other => CreateTokenParamsCardNetworksPreferred::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsCardNetworksPreferred {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonDocumentsCompanyAuthorization {
    #[doc = "One or more document ids returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value ..."]
    pub files: Option<Vec<json::OwnedValue>>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonDocumentsCompanyAuthorization {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPii {
    #[doc = "The `id_number` for the PII, in string form."]
    pub id_number: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPii {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("id_number", self.id_number.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct GetTokenParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for GetTokenParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
