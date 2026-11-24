#[cfg(feature = "credit_notes")]
#[doc = "Create a credit note"]
pub async fn create_credit_note(
    client: &crate::Client,
    params: &stripe_types_08::credit_notes::CreateCreditNoteParams,
) -> Result<stripe_types_15::credit_notes::CreditNote, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "Retrieve a credit note"]
pub async fn get_credit_note(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_00::credit_notes::GetCreditNoteParams,
) -> Result<stripe_types_15::credit_notes::CreditNote, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "Preview a credit note"]
pub async fn get_credit_note_preview(
    client: &crate::Client,
    params: &stripe_types_11::credit_notes::GetCreditNotePreviewParams,
) -> Result<stripe_types_15::credit_notes::CreditNote, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes/preview", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "Retrieve a credit note's line items"]
pub async fn list_credit_note_lines(
    client: &crate::Client,
    credit_note: &str,
    params: &stripe_types_02::credit_notes::ListCreditNoteLinesParams,
) -> Result<crate::List<stripe_types_15::credit_notes::CreditNoteLineItem>, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes/{credit_note}/lines", &[("credit_note", credit_note)]);
    client.get(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "Retrieve a credit note preview's line items"]
pub async fn list_credit_note_preview_lines(
    client: &crate::Client,
    params: &stripe_types_10::credit_notes::ListCreditNotePreviewLinesParams,
) -> Result<crate::List<stripe_types_15::credit_notes::CreditNoteLineItem>, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes/preview/lines", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "List all credit notes"]
pub async fn list_credit_notes(
    client: &crate::Client,
    params: &stripe_types_06::credit_notes::ListCreditNotesParams,
) -> Result<crate::List<stripe_types_15::credit_notes::CreditNote>, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "Update a credit note"]
pub async fn update_credit_note(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_02::credit_notes::UpdateCreditNoteParams,
) -> Result<stripe_types_15::credit_notes::CreditNote, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "credit_notes")]
#[doc = "Void a credit note"]
pub async fn void_credit_note(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::credit_notes::VoidCreditNoteParams,
) -> Result<stripe_types_15::credit_notes::CreditNote, crate::Error> {
    let path = crate::params::format_path("/v1/credit_notes/{id}/void", &[("id", id)]);
    client.post_form(&path, params).await
}
