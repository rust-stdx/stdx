#[cfg(feature = "terminal")]
#[doc = "Cancel the current reader action"]
pub async fn cancel_action(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_06::terminal::CancelActionParams,
) -> Result<stripe_types_15::terminal::GetTerminalReaderResponseExpanded, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}/cancel_action", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Collect inputs using a Reader"]
pub async fn collect_inputs(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_12::terminal::CollectInputsParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}/collect_inputs", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Hand off a PaymentIntent to a Reader and collect card details"]
pub async fn collect_payment_method(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_07::terminal::CollectPaymentMethodParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path =
        crate::params::format_path("/v1/terminal/readers/{reader}/collect_payment_method", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Confirm a PaymentIntent on the Reader"]
pub async fn confirm_payment_intent(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_06::terminal::ConfirmPaymentIntentParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path =
        crate::params::format_path("/v1/terminal/readers/{reader}/confirm_payment_intent", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Create a Configuration"]
pub async fn create_terminal_configuration(
    client: &crate::Client,
    params: &stripe_types_11::terminal::CreateTerminalConfigurationParams,
) -> Result<stripe_types_10::terminal::TerminalConfiguration, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/configurations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Create a Connection Token"]
pub async fn create_terminal_connection_token(
    client: &crate::Client,
    params: &stripe_types_04::terminal::CreateTerminalConnectionTokenParams,
) -> Result<stripe_types_05::terminal::TerminalConnectionToken, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/connection_tokens", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Create a Location"]
pub async fn create_terminal_location(
    client: &crate::Client,
    params: &stripe_types_09::terminal::CreateTerminalLocationParams,
) -> Result<stripe_types_06::terminal::TerminalLocation, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/locations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Create an Onboarding Link"]
pub async fn create_terminal_onboarding_link(
    client: &crate::Client,
    params: &stripe_types_09::terminal::CreateTerminalOnboardingLinkParams,
) -> Result<stripe_types_08::terminal::TerminalOnboardingLink, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/onboarding_links", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Create a Reader"]
pub async fn create_terminal_reader(
    client: &crate::Client,
    params: &stripe_types_02::terminal::CreateTerminalReaderParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Delete a Configuration"]
pub async fn delete_terminal_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::terminal::DeletedTerminalConfiguration, crate::Error> {
    let path = crate::params::format_path(
        "/v1/terminal/configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.delete_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Delete a Location"]
pub async fn delete_terminal_location(
    client: &crate::Client,
    location: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::terminal::DeletedTerminalLocation, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/locations/{location}", &[("location", location)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Delete a Reader"]
pub async fn delete_terminal_reader(
    client: &crate::Client,
    reader: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::terminal::DeletedTerminalReader, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}", &[("reader", reader)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Retrieve a Configuration"]
pub async fn get_terminal_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &stripe_types_02::terminal::GetTerminalConfigurationParams,
) -> Result<stripe_types_12::terminal::GetTerminalConfigurationResponseExpanded, crate::Error> {
    let path = crate::params::format_path(
        "/v1/terminal/configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Retrieve a Location"]
pub async fn get_terminal_location(
    client: &crate::Client,
    location: &str,
    params: &stripe_types_06::terminal::GetTerminalLocationParams,
) -> Result<stripe_types_08::terminal::GetTerminalLocationResponseExpanded, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/locations/{location}", &[("location", location)]);
    client.get(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Retrieve a Reader"]
pub async fn get_terminal_reader(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_06::terminal::GetTerminalReaderParams,
) -> Result<stripe_types_15::terminal::GetTerminalReaderResponseExpanded, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}", &[("reader", reader)]);
    client.get(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "List all Configurations"]
pub async fn list_terminal_configurations(
    client: &crate::Client,
    params: &stripe_types_02::terminal::ListTerminalConfigurationsParams,
) -> Result<crate::List<stripe_types_10::terminal::TerminalConfiguration>, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/configurations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "List all Locations"]
pub async fn list_terminal_locations(
    client: &crate::Client,
    params: &stripe_types_01::terminal::ListTerminalLocationsParams,
) -> Result<crate::List<stripe_types_06::terminal::TerminalLocation>, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/locations", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "List all Readers"]
pub async fn list_terminal_readers(
    client: &crate::Client,
    params: &stripe_types_05::terminal::ListTerminalReadersParams,
) -> Result<crate::List<stripe_types_15::terminal::TerminalReader>, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Hand-off a PaymentIntent to a Reader"]
pub async fn process_payment_intent(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_10::terminal::ProcessPaymentIntentParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path =
        crate::params::format_path("/v1/terminal/readers/{reader}/process_payment_intent", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Hand-off a SetupIntent to a Reader"]
pub async fn process_setup_intent(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_08::terminal::ProcessSetupIntentParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}/process_setup_intent", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Refund a Charge or a PaymentIntent in-person"]
pub async fn refund_payment(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_08::terminal::RefundPaymentParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}/refund_payment", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Set reader display"]
pub async fn set_reader_display(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_08::terminal::SetReaderDisplayParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}/set_reader_display", &[("reader", reader)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Update a Configuration"]
pub async fn update_terminal_configuration(
    client: &crate::Client,
    configuration: &str,
    params: &stripe_types_10::terminal::UpdateTerminalConfigurationParams,
) -> Result<stripe_types_12::terminal::GetTerminalConfigurationResponseExpanded, crate::Error> {
    let path = crate::params::format_path(
        "/v1/terminal/configurations/{configuration}",
        &[("configuration", configuration)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Update a Location"]
pub async fn update_terminal_location(
    client: &crate::Client,
    location: &str,
    params: &stripe_types_09::terminal::UpdateTerminalLocationParams,
) -> Result<stripe_types_08::terminal::GetTerminalLocationResponseExpanded, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/locations/{location}", &[("location", location)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "terminal")]
#[doc = "Update a Reader"]
pub async fn update_terminal_reader(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_04::terminal::UpdateTerminalReaderParams,
) -> Result<stripe_types_15::terminal::GetTerminalReaderResponseExpanded, crate::Error> {
    let path = crate::params::format_path("/v1/terminal/readers/{reader}", &[("reader", reader)]);
    client.post_form(&path, params).await
}
