#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateRemovalsBeneficiary {
    #[doc = "Publicly displayable name for the end beneficiary of carbon removal."]
    #[json(default)]
    pub public_name: String,
}

#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateRemovalsProductsPrice {
    #[doc = "Fees for one metric ton of carbon removal in the currency's smallest unit."]
    #[json(default)]
    pub amount_fees: i64,
    #[doc = "Subtotal for one metric ton of carbon removal (excluding fees) in the currency's smallest unit."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total for one metric ton of carbon removal (including fees) in the currency's smallest unit."]
    #[json(default)]
    pub amount_total: i64,
}

#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateSupplierObject {
    ClimateSupplier,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "climate", feature = "events"))]
impl ClimateSupplierObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ClimateSupplierObject::ClimateSupplier => "climate.supplier",
            ClimateSupplierObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl Default for ClimateSupplierObject {
    fn default() -> Self {
        ClimateSupplierObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl<'json> json::Decode<'json> for ClimateSupplierObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ClimateSupplierObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "climate.supplier" => ClimateSupplierObject::ClimateSupplier,
            other => ClimateSupplierObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateSupplierRemovalPathway {
    BiomassCarbonRemovalAndStorage,
    DirectAirCapture,
    EnhancedWeathering,
    MarineCarbonRemoval,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "climate", feature = "events"))]
impl ClimateSupplierRemovalPathway {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ClimateSupplierRemovalPathway::BiomassCarbonRemovalAndStorage => "biomass_carbon_removal_and_storage",
            ClimateSupplierRemovalPathway::DirectAirCapture => "direct_air_capture",
            ClimateSupplierRemovalPathway::EnhancedWeathering => "enhanced_weathering",
            ClimateSupplierRemovalPathway::MarineCarbonRemoval => "marine_carbon_removal",
            ClimateSupplierRemovalPathway::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl Default for ClimateSupplierRemovalPathway {
    fn default() -> Self {
        ClimateSupplierRemovalPathway::Unknown(String::new())
    }
}

#[cfg(any(feature = "climate", feature = "events"))]
impl<'json> json::Decode<'json> for ClimateSupplierRemovalPathway {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ClimateSupplierRemovalPathway::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "biomass_carbon_removal_and_storage" => ClimateSupplierRemovalPathway::BiomassCarbonRemovalAndStorage,
            "direct_air_capture" => ClimateSupplierRemovalPathway::DirectAirCapture,
            "enhanced_weathering" => ClimateSupplierRemovalPathway::EnhancedWeathering,
            "marine_carbon_removal" => ClimateSupplierRemovalPathway::MarineCarbonRemoval,
            other => ClimateSupplierRemovalPathway::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "climate")]
#[derive(Debug, Clone, Default)]
pub struct ListClimateOrdersParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "climate")]
impl crate::params::FormEncode for ListClimateOrdersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
