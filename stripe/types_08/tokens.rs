#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsAccountCompany {
    #[doc = "The company's primary address."]
    pub address: Option<stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress>,
    #[doc = "The Kana variation of the company's primary address (Japan only)."]
    pub address_kana: Option<stripe_types_04::tokens::CreateTokenParamsAccountCompanyAddressKana>,
    #[doc = "The Kanji variation of the company's primary address (Japan only)."]
    pub address_kanji: Option<stripe_types_05::tokens::CreateTokenParamsAccountCompanyAddressKanji>,
    #[doc = "The location where the business is administered."]
    pub administrative_address: Option<stripe_types_06::tokens::CreateTokenParamsAccountCompanyAdministrativeAddress>,
    #[doc = "Whether the company's directors have been provided."]
    pub directors_provided: Option<bool>,
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct."]
    pub directorship_declaration:
        Option<stripe_types_00::tokens::CreateTokenParamsAccountCompanyDirectorshipDeclaration>,
    #[doc = "Whether the company's executives have been provided."]
    pub executives_provided: Option<bool>,
    #[doc = "The export license ID number of the company, also referred as Import Export Code (India only)."]
    pub export_license_id: Option<String>,
    #[doc = "The purpose code to use for export transactions (India only)."]
    pub export_purpose_code: Option<String>,
    #[doc = "The company's legal name."]
    pub name: Option<String>,
    #[doc = "The Kana variation of the company's legal name (Japan only)."]
    pub name_kana: Option<String>,
    #[doc = "The Kanji variation of the company's legal name (Japan only)."]
    pub name_kanji: Option<String>,
    #[doc = "Whether the company's owners have been provided."]
    pub owners_provided: Option<bool>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    pub ownership_declaration: Option<stripe_types_06::tokens::CreateTokenParamsAccountCompanyOwnershipDeclaration>,
    #[doc = "Whether the user described by the data in the token has been shown the Ownership Declaration and indicated that it is..."]
    pub ownership_declaration_shown_and_signed: Option<bool>,
    #[doc = "This value is used to determine if a business is exempt from providing ultimate beneficial owners."]
    pub ownership_exemption_reason:
        Option<stripe_types_04::tokens::CreateTokenParamsAccountCompanyOwnershipExemptionReason>,
    #[doc = "The company's phone number (used for verification)."]
    pub phone: Option<String>,
    #[doc = "The primary location where the business conducts operations."]
    pub principal_place_of_business:
        Option<stripe_types_05::tokens::CreateTokenParamsAccountCompanyPrincipalPlaceOfBusiness>,
    #[doc = "When the business was incorporated or registered."]
    pub registration_date: Option<stripe_types_02::tokens::CreateTokenParamsAccountCompanyRegistrationDate>,
    #[doc = "The identification number given to a company when it is registered or incorporated, if distinct from the identificati..."]
    pub registration_number: Option<String>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    pub representative_declaration:
        Option<stripe_types_05::tokens::CreateTokenParamsAccountCompanyRepresentativeDeclaration>,
    #[doc = "The category identifying the legal structure of the company or legal entity."]
    pub structure: Option<stripe_types_02::tokens::CreateTokenParamsAccountCompanyStructure>,
    #[doc = "The business ID number of the company, as appropriate for the company’s country."]
    pub tax_id: Option<String>,
    #[doc = "The jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    pub tax_id_registrar: Option<String>,
    #[doc = "The VAT number of the company."]
    pub vat_id: Option<String>,
    #[doc = "Information on the verification state of the company."]
    pub verification: Option<stripe_types_05::tokens::CreateTokenParamsAccountCompanyVerification>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsAccountCompany {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("administrative_address", self.administrative_address.as_ref());
        form.field_opt("directors_provided", self.directors_provided.as_ref());
        form.field_opt("directorship_declaration", self.directorship_declaration.as_ref());
        form.field_opt("executives_provided", self.executives_provided.as_ref());
        form.field_opt("export_license_id", self.export_license_id.as_ref());
        form.field_opt("export_purpose_code", self.export_purpose_code.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("name_kana", self.name_kana.as_ref());
        form.field_opt("name_kanji", self.name_kanji.as_ref());
        form.field_opt("owners_provided", self.owners_provided.as_ref());
        form.field_opt("ownership_declaration", self.ownership_declaration.as_ref());
        form.field_opt(
            "ownership_declaration_shown_and_signed",
            self.ownership_declaration_shown_and_signed.as_ref(),
        );
        form.field_opt("ownership_exemption_reason", self.ownership_exemption_reason.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("principal_place_of_business", self.principal_place_of_business.as_ref());
        form.field_opt("registration_date", self.registration_date.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("representative_declaration", self.representative_declaration.as_ref());
        form.field_opt("structure", self.structure.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
        form.field_opt("tax_id_registrar", self.tax_id_registrar.as_ref());
        form.field_opt("vat_id", self.vat_id.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsCard {
    #[doc = "City / District / Suburb / Town / Village."]
    pub address_city: Option<String>,
    #[doc = "Billing address country, if provided."]
    pub address_country: Option<String>,
    #[doc = "Address line 1 (Street address / PO Box / Company name)."]
    pub address_line1: Option<String>,
    #[doc = "Address line 2 (Apartment / Suite / Unit / Building)."]
    pub address_line2: Option<String>,
    #[doc = "State / County / Province / Region."]
    pub address_state: Option<String>,
    #[doc = "ZIP or postal code."]
    pub address_zip: Option<String>,
    #[doc = "Required in order to add the card to an account; in all other cases, this parameter is not used."]
    pub currency: Option<String>,
    #[doc = "Card security code."]
    pub cvc: Option<String>,
    #[doc = "Two-digit number representing the card's expiration month."]
    pub exp_month: String,
    #[doc = "Two- or four-digit number representing the card's expiration year."]
    pub exp_year: String,
    #[doc = "Cardholder's full name."]
    pub name: Option<String>,
    #[doc = "Contains information about card networks used to process the payment."]
    pub networks: Option<stripe_types_05::tokens::CreateTokenParamsCardNetworks>,
    #[doc = "The card number, as a string without any separators."]
    pub number: String,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address_city", self.address_city.as_ref());
        form.field_opt("address_country", self.address_country.as_ref());
        form.field_opt("address_line1", self.address_line1.as_ref());
        form.field_opt("address_line2", self.address_line2.as_ref());
        form.field_opt("address_state", self.address_state.as_ref());
        form.field_opt("address_zip", self.address_zip.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("cvc", self.cvc.as_ref());
        form.field("exp_month", &self.exp_month);
        form.field("exp_year", &self.exp_year);
        form.field_opt("name", self.name.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field("number", &self.number);
    }
}
