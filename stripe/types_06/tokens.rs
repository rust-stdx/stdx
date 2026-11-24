#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountCompanyAdministrativeAddress =
    stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountCompanyOwnershipDeclaration =
    stripe_types_00::tokens::CreateTokenParamsAccountCompanyDirectorshipDeclaration;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualDob =
    stripe_types_02::tokens::CreateTokenParamsAccountCompanyRegistrationDate;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsAccountIndividualRegisteredAddress =
    stripe_types_03::tokens::CreateTokenParamsAccountCompanyAddress;

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsBankAccount {
    #[doc = "The name of the person or business that owns the bank account."]
    pub account_holder_name: Option<String>,
    #[doc = "The type of entity that holds the account."]
    pub account_holder_type: Option<stripe_types_04::tokens::CreateTokenParamsBankAccountAccountHolderType>,
    #[doc = "The account number for the bank account, in string form."]
    pub account_number: String,
    #[doc = "The bank account type."]
    pub account_type: Option<stripe_types_04::tokens::CreateTokenParamsBankAccountAccountType>,
    #[doc = "The country in which the bank account is located."]
    pub country: String,
    #[doc = "The currency the bank account is in."]
    pub currency: Option<String>,
    #[doc = "The ID of a Payment Method with a `type` of `us_bank_account`."]
    pub payment_method: Option<String>,
    #[doc = "The routing number, sort code, or other country-appropriate institution number for the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field("account_number", &self.account_number);
        form.field_opt("account_type", self.account_type.as_ref());
        form.field("country", &self.country);
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonAddressKana = stripe_types_04::tokens::CreateTokenParamsAccountCompanyAddressKana;

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonDocumentsVisa =
    stripe_types_00::tokens::CreateTokenParamsPersonDocumentsCompanyAuthorization;

#[cfg(feature = "tokens")]
#[derive(Debug, Clone, Default)]
pub struct CreateTokenParamsPersonUsCfpbDataEthnicityDetails {
    #[doc = "The persons ethnicity"]
    pub ethnicity: Option<Vec<stripe_types_01::tokens::CreateTokenParamsPersonUsCfpbDataEthnicityDetailsEthnicityItem>>,
    #[doc = "Please specify your origin, when other is selected."]
    pub ethnicity_other: Option<String>,
}

#[cfg(feature = "tokens")]
impl crate::params::FormEncode for CreateTokenParamsPersonUsCfpbDataEthnicityDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.ethnicity.as_ref() {
            form.field_array("ethnicity", values.as_slice());
        }
        form.field_opt("ethnicity_other", self.ethnicity_other.as_ref());
    }
}

#[cfg(feature = "tokens")]
pub type CreateTokenParamsPersonVerificationAdditionalDocument =
    stripe_types_00::tokens::CreateTokenParamsAccountCompanyVerificationDocument;
