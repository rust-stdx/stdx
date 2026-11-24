#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountCompanyAddressKanji =
    stripe_types_04::tokens::CreateTokenParamsAccountCompanyAddressKana;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountCompanyPrincipalPlaceOfBusiness =
    stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountCompanyRepresentativeDeclaration =
    stripe_types_00::tokens::CreateTokenParamsAccountCompanyDirectorshipDeclaration;

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompanyVerification {
    #[doc = "A document verifying the business."]
    pub document: Option<stripe_types_00::tokens::CreateTokenParamsAccountCompanyVerificationDocument>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompanyVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualAddress = stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualAddressKana =
    stripe_types_04::tokens::CreateTokenParamsAccountCompanyAddressKana;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualAddressKanji =
    stripe_types_04::tokens::CreateTokenParamsAccountCompanyAddressKana;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualVerificationAdditionalDocument =
    stripe_types_00::tokens::CreateTokenParamsAccountCompanyVerificationDocument;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualVerificationDocument =
    stripe_types_00::tokens::CreateTokenParamsAccountCompanyVerificationDocument;

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsCardNetworks {
    #[doc = "The customer's preferred card network for co-branded cards."]
    pub preferred: Option<stripe_types_00::tokens::CreateTokenParamsCardNetworksPreferred>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsCardNetworks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred", self.preferred.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonAdditionalTosAcceptances {
    #[doc = "Details on the legal guardian's acceptance of the main Stripe service agreement."]
    pub account: Option<stripe_types_02::tokens::CreateTokenParamsPersonAdditionalTosAcceptancesAccount>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonAdditionalTosAcceptances {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
    }
}

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonAddress = stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonAddressKanji = stripe_types_04::tokens::CreateTokenParamsAccountCompanyAddressKana;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonDob = stripe_types_02::tokens::CreateTokenParamsAccountCompanyRegistrationDate;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonDocumentsPassport =
    stripe_types_00::tokens::CreateTokenParamsPersonDocumentsCompanyAuthorization;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonPoliticalExposure =
    stripe_types_04::tokens::CreateTokenParamsAccountIndividualPoliticalExposure;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonRegisteredAddress = stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress;

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonUsCfpbDataRaceDetails {
    #[doc = "The persons race."]
    pub race: Option<Vec<stripe_types_04::tokens::CreateTokenParamsPersonUsCfpbDataRaceDetailsRaceItem>>,
    #[doc = "Please specify your race, when other is selected."]
    pub race_other: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonUsCfpbDataRaceDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.race.as_ref() {
            form.field_array("race", values.as_slice());
        }
        form.field_opt("race_other", self.race_other.as_ref());
    }
}

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonVerificationDocument =
    stripe_types_00::tokens::CreateTokenParamsAccountCompanyVerificationDocument;
