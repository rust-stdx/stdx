#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlightSegmentsItem {
    #[doc = "The three-letter IATA airport code of the flight's destination."]
    pub arrival_airport_code: Option<String>,
    #[doc = "The airline carrier code."]
    pub carrier: Option<String>,
    #[doc = "The three-letter IATA airport code that the flight departed from."]
    pub departure_airport_code: Option<String>,
    #[doc = "The flight number."]
    pub flight_number: Option<String>,
    #[doc = "The flight's service class."]
    pub service_class: Option<String>,
    #[doc = "Whether a stopover is allowed on this flight."]
    pub stopover_allowed: Option<bool>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlightSegmentsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("arrival_airport_code", self.arrival_airport_code.as_ref());
        form.field_opt("carrier", self.carrier.as_ref());
        form.field_opt("departure_airport_code", self.departure_airport_code.as_ref());
        form.field_opt("flight_number", self.flight_number.as_ref());
        form.field_opt("service_class", self.service_class.as_ref());
        form.field_opt("stopover_allowed", self.stopover_allowed.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Always => "always",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Limited => "limited",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Unspecified => "unspecified",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Always,
            "limited" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Limited,
            "unspecified" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Unspecified,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetailsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetailsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataNzBankAccount {
    #[doc = "The name on the bank account."]
    pub account_holder_name: Option<String>,
    #[doc = "The account number for the bank account."]
    pub account_number: String,
    #[doc = "The numeric code for the bank account's bank."]
    pub bank_code: String,
    #[doc = "The numeric code for the bank account's bank branch."]
    pub branch_code: String,
    pub reference: Option<String>,
    #[doc = "The suffix of the bank account number."]
    pub suffix: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field("account_number", &self.account_number);
        form.field("bank_code", &self.bank_code);
        form.field("branch_code", &self.branch_code);
        form.field_opt("reference", self.reference.as_ref());
        form.field("suffix", &self.suffix);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataSepaDebit {
    #[doc = "IBAN of the bank account."]
    pub iban: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("iban", &self.iban);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry {
    AT,
    BE,
    DE,
    ES,
    IT,
    NL,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::AT => "AT",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::BE => "BE",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::DE => "DE",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::ES => "ES",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::IT => "IT",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::NL => "NL",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AT" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::AT,
            "BE" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::BE,
            "DE" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::DE,
            "ES" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::ES,
            "IT" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::IT,
            "NL" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::NL,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Checking => "checking",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Savings => "savings",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Checking,
            "savings" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Savings,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Chip => "chip",
            CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Contactless => "contactless",
            CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::KeyedIn => "keyed_in",
            CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Online => "online",
            CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Swipe => "swipe",
            CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "chip" => CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Chip,
            "contactless" => CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Contactless,
            "keyed_in" => CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::KeyedIn,
            "online" => CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Online,
            "swipe" => CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Swipe,
            other => CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType {
    FuelAndNonFuelPurchase,
    FuelPurchase,
    NonFuelPurchase,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::FuelAndNonFuelPurchase => {
                "fuel_and_non_fuel_purchase"
            }
            CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::FuelPurchase => "fuel_purchase",
            CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::NonFuelPurchase => "non_fuel_purchase",
            CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fuel_and_non_fuel_purchase" => {
                CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::FuelAndNonFuelPurchase
            }
            "fuel_purchase" => CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::FuelPurchase,
            "non_fuel_purchase" => CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::NonFuelPurchase,
            other => CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsFuelType {
    Diesel,
    Other,
    UnleadedPlus,
    UnleadedRegular,
    UnleadedSuper,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsFuelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsFuelType::Diesel => "diesel",
            CreateTestHelperIssuingAuthorizationParamsFuelType::Other => "other",
            CreateTestHelperIssuingAuthorizationParamsFuelType::UnleadedPlus => "unleaded_plus",
            CreateTestHelperIssuingAuthorizationParamsFuelType::UnleadedRegular => "unleaded_regular",
            CreateTestHelperIssuingAuthorizationParamsFuelType::UnleadedSuper => "unleaded_super",
            CreateTestHelperIssuingAuthorizationParamsFuelType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsFuelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsFuelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "diesel" => CreateTestHelperIssuingAuthorizationParamsFuelType::Diesel,
            "other" => CreateTestHelperIssuingAuthorizationParamsFuelType::Other,
            "unleaded_plus" => CreateTestHelperIssuingAuthorizationParamsFuelType::UnleadedPlus,
            "unleaded_regular" => CreateTestHelperIssuingAuthorizationParamsFuelType::UnleadedRegular,
            "unleaded_super" => CreateTestHelperIssuingAuthorizationParamsFuelType::UnleadedSuper,
            other => CreateTestHelperIssuingAuthorizationParamsFuelType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFuelType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricVehicleCharging,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmergencyServicesGcasVisaUseOnly,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentLicensedHorseDogRacingUsRegionOnly,
    GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
    GovernmentOwnedLotteriesNonUsRegion,
    GovernmentOwnedLotteriesUsRegionOnly,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    Marketplaces,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AcRefrigerationRepair => "ac_refrigeration_repair",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AdvertisingServices => "advertising_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AgriculturalCooperative => "agricultural_cooperative",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AirlinesAirCarriers => "airlines_air_carriers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AirportsFlyingFields => "airports_flying_fields",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AmbulanceServices => "ambulance_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AmusementParksCarnivals => "amusement_parks_carnivals",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AntiqueReproductions => "antique_reproductions",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AntiqueShops => "antique_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Aquariums => "aquariums",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ArchitecturalSurveyingServices => "architectural_surveying_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ArtDealersAndGalleries => "art_dealers_and_galleries",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoBodyRepairShops => "auto_body_repair_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoPaintShops => "auto_paint_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoServiceShops => "auto_service_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomatedCashDisburse => "automated_cash_disburse",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomatedFuelDispensers => "automated_fuel_dispensers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomobileAssociations => "automobile_associations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomotiveTireStores => "automotive_tire_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BailAndBondPayments => "bail_and_bond_payments",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Bakeries => "bakeries",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BandsOrchestras => "bands_orchestras",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BarberAndBeautyShops => "barber_and_beauty_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BettingCasinoGambling => "betting_casino_gambling",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BicycleShops => "bicycle_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BilliardPoolEstablishments => "billiard_pool_establishments",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BoatDealers => "boat_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BoatRentalsAndLeases => "boat_rentals_and_leases",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BookStores => "book_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BowlingAlleys => "bowling_alleys",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BusLines => "bus_lines",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BusinessSecretarialSchools => "business_secretarial_schools",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BuyingShoppingServices => "buying_shopping_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CableSatelliteAndOtherPayTelevisionAndRadio => "cable_satellite_and_other_pay_television_and_radio",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarRentalAgencies => "car_rental_agencies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarWashes => "car_washes",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarpentryServices => "carpentry_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Caterers => "caterers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CharitableAndSocialServiceOrganizationsFundraising => "charitable_and_social_service_organizations_fundraising",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChildCareServices => "child_care_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Chiropractors => "chiropractors",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CigarStoresAndStands => "cigar_stores_and_stands",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CleaningAndMaintenance => "cleaning_and_maintenance",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ClothingRental => "clothing_rental",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CollegesUniversities => "colleges_universities",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommercialEquipment => "commercial_equipment",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommercialFootwear => "commercial_footwear",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerNetworkServices => "computer_network_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerProgramming => "computer_programming",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerRepair => "computer_repair",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerSoftwareStores => "computer_software_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ConcreteWorkServices => "concrete_work_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ConstructionMaterials => "construction_materials",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ConsultingPublicRelations => "consulting_public_relations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CorrespondenceSchools => "correspondence_schools",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CosmeticStores => "cosmetic_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CounselingServices => "counseling_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CountryClubs => "country_clubs",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CourierServices => "courier_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CourtCosts => "court_costs",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CreditReportingAgencies => "credit_reporting_agencies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CruiseLines => "cruise_lines",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DairyProductsStores => "dairy_products_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DanceHallStudiosSchools => "dance_hall_studios_schools",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DatingEscortServices => "dating_escort_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DentistsOrthodontists => "dentists_orthodontists",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DepartmentStores => "department_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DetectiveAgencies => "detective_agencies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsApplications => "digital_goods_applications",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsGames => "digital_goods_games",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsMedia => "digital_goods_media",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingCombinationCatalogAndRetailMerchant => "direct_marketing_combination_catalog_and_retail_merchant",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingOther => "direct_marketing_other",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingSubscription => "direct_marketing_subscription",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingTravel => "direct_marketing_travel",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DiscountStores => "discount_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Doctors => "doctors",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DoorToDoorSales => "door_to_door_sales",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DraperyWindowCoveringAndUpholsteryStores => "drapery_window_covering_and_upholstery_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DrinkingPlaces => "drinking_places",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DrugsDrugProprietariesAndDruggistSundries => "drugs_drug_proprietaries_and_druggist_sundries",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DryCleaners => "dry_cleaners",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DurableGoods => "durable_goods",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DutyFreeStores => "duty_free_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EatingPlacesRestaurants => "eating_places_restaurants",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EducationalServices => "educational_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricRazorStores => "electric_razor_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricVehicleCharging => "electric_vehicle_charging",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricalServices => "electrical_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectronicsRepairShops => "electronics_repair_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectronicsStores => "electronics_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElementarySecondarySchools => "elementary_secondary_schools",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EmergencyServicesGcasVisaUseOnly => "emergency_services_gcas_visa_use_only",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EmploymentTempAgencies => "employment_temp_agencies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EquipmentRental => "equipment_rental",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ExterminatingServices => "exterminating_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FamilyClothingStores => "family_clothing_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FastFoodRestaurants => "fast_food_restaurants",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FinancialInstitutions => "financial_institutions",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FinesGovernmentAdministrativeEntities => "fines_government_administrative_entities",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FireplaceFireplaceScreensAndAccessoriesStores => "fireplace_fireplace_screens_and_accessories_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FloorCoveringStores => "floor_covering_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Florists => "florists",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FloristsSuppliesNurseryStockAndFlowers => "florists_supplies_nursery_stock_and_flowers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FuneralServicesCrematories => "funeral_services_crematories",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => "furniture_home_furnishings_and_equipment_stores_except_appliances",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FurriersAndFurShops => "furriers_and_fur_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GeneralServices => "general_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GlasswareCrystalStores => "glassware_crystal_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GolfCoursesPublic => "golf_courses_public",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentLicensedHorseDogRacingUsRegionOnly => "government_licensed_horse_dog_racing_us_region_only",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly => "government_licensed_online_casions_online_gambling_us_region_only",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentOwnedLotteriesNonUsRegion => "government_owned_lotteries_non_us_region",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentOwnedLotteriesUsRegionOnly => "government_owned_lotteries_us_region_only",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentServices => "government_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HardwareStores => "hardware_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HealthAndBeautySpas => "health_and_beauty_spas",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HeatingPlumbingAC => "heating_plumbing_a_c",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Hospitals => "hospitals",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HouseholdApplianceStores => "household_appliance_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::IndustrialSupplies => "industrial_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::InformationRetrievalServices => "information_retrieval_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::InsuranceDefault => "insurance_default",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::IntraCompanyPurchases => "intra_company_purchases",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::JewelryStoresWatchesClocksAndSilverwareStores => "jewelry_stores_watches_clocks_and_silverware_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LandscapingServices => "landscaping_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Laundries => "laundries",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LaundryCleaningServices => "laundry_cleaning_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LegalServicesAttorneys => "legal_services_attorneys",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ManualCashDisburse => "manual_cash_disburse",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Marketplaces => "marketplaces",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MassageParlors => "massage_parlors",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MedicalAndDentalLabs => "medical_and_dental_labs",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => "medical_dental_ophthalmic_and_hospital_equipment_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MedicalServices => "medical_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MembershipOrganizations => "membership_organizations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MensAndBoysClothingAndAccessoriesStores => "mens_and_boys_clothing_and_accessories_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MensWomensClothingStores => "mens_womens_clothing_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MetalServiceCenters => "metal_service_centers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousApparelAndAccessoryShops => "miscellaneous_apparel_and_accessory_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousBusinessServices => "miscellaneous_business_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousFoodStores => "miscellaneous_food_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousGeneralServices => "miscellaneous_general_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousHomeFurnishingSpecialtyStores => "miscellaneous_home_furnishing_specialty_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MobileHomeDealers => "mobile_home_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotionPictureTheaters => "motion_picture_theaters",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorHomesDealers => "motor_homes_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MusicStoresMusicalInstrumentsPianosAndSheetMusic => "music_stores_musical_instruments_pianos_and_sheet_music",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NonFiMoneyOrders => "non_fi_money_orders",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NondurableGoods => "nondurable_goods",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NursingPersonalCare => "nursing_personal_care",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OpticiansEyeglasses => "opticians_eyeglasses",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Osteopaths => "osteopaths",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ParkingLotsGarages => "parking_lots_garages",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PassengerRailways => "passenger_railways",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PawnShops => "pawn_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PhotoDeveloping => "photo_developing",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => "photographic_photocopy_microfilm_equipment_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PhotographicStudios => "photographic_studios",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PictureVideoProduction => "picture_video_production",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PoliticalOrganizations => "political_organizations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PostalServicesGovernmentOnly => "postal_services_government_only",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PreciousStonesAndMetalsWatchesAndJewelry => "precious_stones_and_metals_watches_and_jewelry",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ProfessionalServices => "professional_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Railroads => "railroads",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RecordStores => "record_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ReligiousGoodsStores => "religious_goods_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ReligiousOrganizations => "religious_organizations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SecretarialSupportServices => "secretarial_support_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SecurityBrokersDealers => "security_brokers_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ServiceStations => "service_stations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SewingNeedleworkFabricAndPieceGoodsStores => "sewing_needlework_fabric_and_piece_goods_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ShoeStores => "shoe_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SmallApplianceRepair => "small_appliance_repair",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SnowmobileDealers => "snowmobile_dealers",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SpecialTradeServices => "special_trade_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SpecialtyCleaning => "specialty_cleaning",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportingGoodsStores => "sporting_goods_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportingRecreationCamps => "sporting_recreation_camps",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportsClubsFields => "sports_clubs_fields",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::StampAndCoinStores => "stamp_and_coin_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::StationaryOfficeSuppliesPrintingAndWritingPaper => "stationary_office_supplies_printing_and_writing_paper",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::StationeryStoresOfficeAndSchoolSupplyStores => "stationery_stores_office_and_school_supply_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SwimmingPoolsSales => "swimming_pools_sales",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TUiTravelGermany => "t_ui_travel_germany",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TailorsAlterations => "tailors_alterations",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TaxPreparationServices => "tax_preparation_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TaxicabsLimousines => "taxicabs_limousines",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TelecommunicationEquipmentAndTelephoneSales => "telecommunication_equipment_and_telephone_sales",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TelecommunicationServices => "telecommunication_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TelegraphServices => "telegraph_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TentAndAwningShops => "tent_and_awning_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TestingLaboratories => "testing_laboratories",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Timeshares => "timeshares",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TireRetreadingAndRepair => "tire_retreading_and_repair",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TollsBridgeFees => "tolls_bridge_fees",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TowingServices => "towing_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TransportationServices => "transportation_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TruckStopIteration => "truck_stop_iteration",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TypesettingPlateMakingAndRelatedServices => "typesetting_plate_making_and_related_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TypewriterStores => "typewriter_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::USFederalGovernmentAgenciesOrDepartments => "u_s_federal_government_agencies_or_departments",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::UniformsCommercialClothing => "uniforms_commercial_clothing",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Utilities => "utilities",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VarietyStores => "variety_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VeterinaryServices => "veterinary_services",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VideoGameArcades => "video_game_arcades",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VideoTapeRentalStores => "video_tape_rental_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VocationalTradeSchools => "vocational_trade_schools",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WatchJewelryRepair => "watch_jewelry_repair",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WeldingRepair => "welding_repair",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WholesaleClubs => "wholesale_clubs",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WigAndToupeeStores => "wig_and_toupee_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WiresMoneyOrders => "wires_money_orders",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
            CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ac_refrigeration_repair" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AcRefrigerationRepair,
            "accounting_bookkeeping_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AccountingBookkeepingServices,
            "advertising_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AdvertisingServices,
            "agricultural_cooperative" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AgriculturalCooperative,
            "airlines_air_carriers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AirlinesAirCarriers,
            "airports_flying_fields" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AirportsFlyingFields,
            "ambulance_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AmbulanceServices,
            "amusement_parks_carnivals" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AmusementParksCarnivals,
            "antique_reproductions" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AntiqueReproductions,
            "antique_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AntiqueShops,
            "aquariums" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Aquariums,
            "architectural_surveying_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ArchitecturalSurveyingServices,
            "art_dealers_and_galleries" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ArtDealersAndGalleries,
            "artists_supply_and_craft_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ArtistsSupplyAndCraftShops,
            "auto_and_home_supply_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoAndHomeSupplyStores,
            "auto_body_repair_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoBodyRepairShops,
            "auto_paint_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoPaintShops,
            "auto_service_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutoServiceShops,
            "automated_cash_disburse" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomatedCashDisburse,
            "automated_fuel_dispensers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomatedFuelDispensers,
            "automobile_associations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomobileAssociations,
            "automotive_parts_and_accessories_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomotivePartsAndAccessoriesStores,
            "automotive_tire_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::AutomotiveTireStores,
            "bail_and_bond_payments" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BailAndBondPayments,
            "bakeries" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Bakeries,
            "bands_orchestras" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BandsOrchestras,
            "barber_and_beauty_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BarberAndBeautyShops,
            "betting_casino_gambling" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BettingCasinoGambling,
            "bicycle_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BicycleShops,
            "billiard_pool_establishments" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BilliardPoolEstablishments,
            "boat_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BoatDealers,
            "boat_rentals_and_leases" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BoatRentalsAndLeases,
            "book_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BookStores,
            "books_periodicals_and_newspapers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BooksPeriodicalsAndNewspapers,
            "bowling_alleys" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BowlingAlleys,
            "bus_lines" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BusLines,
            "business_secretarial_schools" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BusinessSecretarialSchools,
            "buying_shopping_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::BuyingShoppingServices,
            "cable_satellite_and_other_pay_television_and_radio" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CableSatelliteAndOtherPayTelevisionAndRadio,
            "camera_and_photographic_supply_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CameraAndPhotographicSupplyStores,
            "candy_nut_and_confectionery_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CandyNutAndConfectioneryStores,
            "car_and_truck_dealers_new_used" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarAndTruckDealersNewUsed,
            "car_and_truck_dealers_used_only" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarAndTruckDealersUsedOnly,
            "car_rental_agencies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarRentalAgencies,
            "car_washes" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarWashes,
            "carpentry_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarpentryServices,
            "carpet_upholstery_cleaning" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CarpetUpholsteryCleaning,
            "caterers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Caterers,
            "charitable_and_social_service_organizations_fundraising" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CharitableAndSocialServiceOrganizationsFundraising,
            "chemicals_and_allied_products" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChemicalsAndAlliedProducts,
            "child_care_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChildCareServices,
            "childrens_and_infants_wear_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChildrensAndInfantsWearStores,
            "chiropodists_podiatrists" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ChiropodistsPodiatrists,
            "chiropractors" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Chiropractors,
            "cigar_stores_and_stands" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CigarStoresAndStands,
            "civic_social_fraternal_associations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CivicSocialFraternalAssociations,
            "cleaning_and_maintenance" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CleaningAndMaintenance,
            "clothing_rental" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ClothingRental,
            "colleges_universities" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CollegesUniversities,
            "commercial_equipment" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommercialEquipment,
            "commercial_footwear" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommercialFootwear,
            "commercial_photography_art_and_graphics" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommercialPhotographyArtAndGraphics,
            "commuter_transport_and_ferries" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CommuterTransportAndFerries,
            "computer_network_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerNetworkServices,
            "computer_programming" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerProgramming,
            "computer_repair" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerRepair,
            "computer_software_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputerSoftwareStores,
            "computers_peripherals_and_software" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ComputersPeripheralsAndSoftware,
            "concrete_work_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ConcreteWorkServices,
            "construction_materials" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ConstructionMaterials,
            "consulting_public_relations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ConsultingPublicRelations,
            "correspondence_schools" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CorrespondenceSchools,
            "cosmetic_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CosmeticStores,
            "counseling_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CounselingServices,
            "country_clubs" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CountryClubs,
            "courier_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CourierServices,
            "court_costs" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CourtCosts,
            "credit_reporting_agencies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CreditReportingAgencies,
            "cruise_lines" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::CruiseLines,
            "dairy_products_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DairyProductsStores,
            "dance_hall_studios_schools" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DanceHallStudiosSchools,
            "dating_escort_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DatingEscortServices,
            "dentists_orthodontists" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DentistsOrthodontists,
            "department_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DepartmentStores,
            "detective_agencies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DetectiveAgencies,
            "digital_goods_applications" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsApplications,
            "digital_goods_games" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsGames,
            "digital_goods_large_volume" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsLargeVolume,
            "digital_goods_media" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DigitalGoodsMedia,
            "direct_marketing_catalog_merchant" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingCatalogMerchant,
            "direct_marketing_combination_catalog_and_retail_merchant" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingCombinationCatalogAndRetailMerchant,
            "direct_marketing_inbound_telemarketing" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingInboundTelemarketing,
            "direct_marketing_insurance_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingInsuranceServices,
            "direct_marketing_other" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingOther,
            "direct_marketing_outbound_telemarketing" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingOutboundTelemarketing,
            "direct_marketing_subscription" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingSubscription,
            "direct_marketing_travel" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DirectMarketingTravel,
            "discount_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DiscountStores,
            "doctors" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Doctors,
            "door_to_door_sales" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DoorToDoorSales,
            "drapery_window_covering_and_upholstery_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DraperyWindowCoveringAndUpholsteryStores,
            "drinking_places" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DrinkingPlaces,
            "drug_stores_and_pharmacies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DrugStoresAndPharmacies,
            "drugs_drug_proprietaries_and_druggist_sundries" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DrugsDrugProprietariesAndDruggistSundries,
            "dry_cleaners" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DryCleaners,
            "durable_goods" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DurableGoods,
            "duty_free_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::DutyFreeStores,
            "eating_places_restaurants" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EatingPlacesRestaurants,
            "educational_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EducationalServices,
            "electric_razor_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricRazorStores,
            "electric_vehicle_charging" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricVehicleCharging,
            "electrical_parts_and_equipment" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricalPartsAndEquipment,
            "electrical_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectricalServices,
            "electronics_repair_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectronicsRepairShops,
            "electronics_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElectronicsStores,
            "elementary_secondary_schools" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ElementarySecondarySchools,
            "emergency_services_gcas_visa_use_only" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EmergencyServicesGcasVisaUseOnly,
            "employment_temp_agencies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EmploymentTempAgencies,
            "equipment_rental" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::EquipmentRental,
            "exterminating_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ExterminatingServices,
            "family_clothing_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FamilyClothingStores,
            "fast_food_restaurants" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FastFoodRestaurants,
            "financial_institutions" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FinancialInstitutions,
            "fines_government_administrative_entities" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FinesGovernmentAdministrativeEntities,
            "fireplace_fireplace_screens_and_accessories_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FireplaceFireplaceScreensAndAccessoriesStores,
            "floor_covering_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FloorCoveringStores,
            "florists" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Florists,
            "florists_supplies_nursery_stock_and_flowers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FloristsSuppliesNurseryStockAndFlowers,
            "freezer_and_locker_meat_provisioners" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FreezerAndLockerMeatProvisioners,
            "fuel_dealers_non_automotive" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FuelDealersNonAutomotive,
            "funeral_services_crematories" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FuneralServicesCrematories,
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
            "furniture_repair_refinishing" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FurnitureRepairRefinishing,
            "furriers_and_fur_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::FurriersAndFurShops,
            "general_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GeneralServices,
            "gift_card_novelty_and_souvenir_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GiftCardNoveltyAndSouvenirShops,
            "glass_paint_and_wallpaper_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GlassPaintAndWallpaperStores,
            "glassware_crystal_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GlasswareCrystalStores,
            "golf_courses_public" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GolfCoursesPublic,
            "government_licensed_horse_dog_racing_us_region_only" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentLicensedHorseDogRacingUsRegionOnly,
            "government_licensed_online_casions_online_gambling_us_region_only" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
            "government_owned_lotteries_non_us_region" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentOwnedLotteriesNonUsRegion,
            "government_owned_lotteries_us_region_only" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentOwnedLotteriesUsRegionOnly,
            "government_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GovernmentServices,
            "grocery_stores_supermarkets" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::GroceryStoresSupermarkets,
            "hardware_equipment_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HardwareEquipmentAndSupplies,
            "hardware_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HardwareStores,
            "health_and_beauty_spas" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HealthAndBeautySpas,
            "hearing_aids_sales_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HearingAidsSalesAndSupplies,
            "heating_plumbing_a_c" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HeatingPlumbingAC,
            "hobby_toy_and_game_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HobbyToyAndGameShops,
            "home_supply_warehouse_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HomeSupplyWarehouseStores,
            "hospitals" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Hospitals,
            "hotels_motels_and_resorts" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HotelsMotelsAndResorts,
            "household_appliance_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::HouseholdApplianceStores,
            "industrial_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::IndustrialSupplies,
            "information_retrieval_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::InformationRetrievalServices,
            "insurance_default" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::InsuranceDefault,
            "insurance_underwriting_premiums" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::InsuranceUnderwritingPremiums,
            "intra_company_purchases" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::IntraCompanyPurchases,
            "jewelry_stores_watches_clocks_and_silverware_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::JewelryStoresWatchesClocksAndSilverwareStores,
            "landscaping_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LandscapingServices,
            "laundries" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Laundries,
            "laundry_cleaning_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LaundryCleaningServices,
            "legal_services_attorneys" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LegalServicesAttorneys,
            "luggage_and_leather_goods_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LuggageAndLeatherGoodsStores,
            "lumber_building_materials_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::LumberBuildingMaterialsStores,
            "manual_cash_disburse" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ManualCashDisburse,
            "marinas_service_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MarinasServiceAndSupplies,
            "marketplaces" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Marketplaces,
            "masonry_stonework_and_plaster" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MasonryStoneworkAndPlaster,
            "massage_parlors" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MassageParlors,
            "medical_and_dental_labs" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MedicalAndDentalLabs,
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
            "medical_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MedicalServices,
            "membership_organizations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MembershipOrganizations,
            "mens_and_boys_clothing_and_accessories_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MensAndBoysClothingAndAccessoriesStores,
            "mens_womens_clothing_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MensWomensClothingStores,
            "metal_service_centers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MetalServiceCenters,
            "miscellaneous_apparel_and_accessory_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousApparelAndAccessoryShops,
            "miscellaneous_auto_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousAutoDealers,
            "miscellaneous_business_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousBusinessServices,
            "miscellaneous_food_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousFoodStores,
            "miscellaneous_general_merchandise" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousGeneralMerchandise,
            "miscellaneous_general_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousGeneralServices,
            "miscellaneous_home_furnishing_specialty_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousHomeFurnishingSpecialtyStores,
            "miscellaneous_publishing_and_printing" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousPublishingAndPrinting,
            "miscellaneous_recreation_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousRecreationServices,
            "miscellaneous_repair_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousRepairShops,
            "miscellaneous_specialty_retail" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MiscellaneousSpecialtyRetail,
            "mobile_home_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MobileHomeDealers,
            "motion_picture_theaters" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotionPictureTheaters,
            "motor_freight_carriers_and_trucking" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorFreightCarriersAndTrucking,
            "motor_homes_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorHomesDealers,
            "motor_vehicle_supplies_and_new_parts" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorVehicleSuppliesAndNewParts,
            "motorcycle_shops_and_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorcycleShopsAndDealers,
            "motorcycle_shops_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MotorcycleShopsDealers,
            "music_stores_musical_instruments_pianos_and_sheet_music" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::MusicStoresMusicalInstrumentsPianosAndSheetMusic,
            "news_dealers_and_newsstands" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NewsDealersAndNewsstands,
            "non_fi_money_orders" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NonFiMoneyOrders,
            "non_fi_stored_value_card_purchase_load" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NonFiStoredValueCardPurchaseLoad,
            "nondurable_goods" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NondurableGoods,
            "nurseries_lawn_and_garden_supply_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NurseriesLawnAndGardenSupplyStores,
            "nursing_personal_care" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::NursingPersonalCare,
            "office_and_commercial_furniture" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OfficeAndCommercialFurniture,
            "opticians_eyeglasses" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OpticiansEyeglasses,
            "optometrists_ophthalmologist" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OptometristsOphthalmologist,
            "orthopedic_goods_prosthetic_devices" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::OrthopedicGoodsProstheticDevices,
            "osteopaths" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Osteopaths,
            "package_stores_beer_wine_and_liquor" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PackageStoresBeerWineAndLiquor,
            "paints_varnishes_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PaintsVarnishesAndSupplies,
            "parking_lots_garages" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ParkingLotsGarages,
            "passenger_railways" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PassengerRailways,
            "pawn_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PawnShops,
            "pet_shops_pet_food_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PetShopsPetFoodAndSupplies,
            "petroleum_and_petroleum_products" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PetroleumAndPetroleumProducts,
            "photo_developing" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PhotoDeveloping,
            "photographic_photocopy_microfilm_equipment_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
            "photographic_studios" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PhotographicStudios,
            "picture_video_production" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PictureVideoProduction,
            "piece_goods_notions_and_other_dry_goods" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PieceGoodsNotionsAndOtherDryGoods,
            "plumbing_heating_equipment_and_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PlumbingHeatingEquipmentAndSupplies,
            "political_organizations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PoliticalOrganizations,
            "postal_services_government_only" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PostalServicesGovernmentOnly,
            "precious_stones_and_metals_watches_and_jewelry" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PreciousStonesAndMetalsWatchesAndJewelry,
            "professional_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ProfessionalServices,
            "public_warehousing_and_storage" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::PublicWarehousingAndStorage,
            "quick_copy_repro_and_blueprint" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::QuickCopyReproAndBlueprint,
            "railroads" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Railroads,
            "real_estate_agents_and_managers_rentals" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RealEstateAgentsAndManagersRentals,
            "record_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RecordStores,
            "recreational_vehicle_rentals" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RecreationalVehicleRentals,
            "religious_goods_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ReligiousGoodsStores,
            "religious_organizations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ReligiousOrganizations,
            "roofing_siding_sheet_metal" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::RoofingSidingSheetMetal,
            "secretarial_support_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SecretarialSupportServices,
            "security_brokers_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SecurityBrokersDealers,
            "service_stations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ServiceStations,
            "sewing_needlework_fabric_and_piece_goods_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SewingNeedleworkFabricAndPieceGoodsStores,
            "shoe_repair_hat_cleaning" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ShoeRepairHatCleaning,
            "shoe_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::ShoeStores,
            "small_appliance_repair" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SmallApplianceRepair,
            "snowmobile_dealers" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SnowmobileDealers,
            "special_trade_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SpecialTradeServices,
            "specialty_cleaning" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SpecialtyCleaning,
            "sporting_goods_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportingGoodsStores,
            "sporting_recreation_camps" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportingRecreationCamps,
            "sports_and_riding_apparel_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportsAndRidingApparelStores,
            "sports_clubs_fields" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SportsClubsFields,
            "stamp_and_coin_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::StampAndCoinStores,
            "stationary_office_supplies_printing_and_writing_paper" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::StationaryOfficeSuppliesPrintingAndWritingPaper,
            "stationery_stores_office_and_school_supply_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::StationeryStoresOfficeAndSchoolSupplyStores,
            "swimming_pools_sales" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::SwimmingPoolsSales,
            "t_ui_travel_germany" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TUiTravelGermany,
            "tailors_alterations" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TailorsAlterations,
            "tax_payments_government_agencies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TaxPaymentsGovernmentAgencies,
            "tax_preparation_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TaxPreparationServices,
            "taxicabs_limousines" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TaxicabsLimousines,
            "telecommunication_equipment_and_telephone_sales" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TelecommunicationEquipmentAndTelephoneSales,
            "telecommunication_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TelecommunicationServices,
            "telegraph_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TelegraphServices,
            "tent_and_awning_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TentAndAwningShops,
            "testing_laboratories" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TestingLaboratories,
            "theatrical_ticket_agencies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TheatricalTicketAgencies,
            "timeshares" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Timeshares,
            "tire_retreading_and_repair" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TireRetreadingAndRepair,
            "tolls_bridge_fees" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TollsBridgeFees,
            "tourist_attractions_and_exhibits" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TouristAttractionsAndExhibits,
            "towing_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TowingServices,
            "trailer_parks_campgrounds" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TrailerParksCampgrounds,
            "transportation_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TransportationServices,
            "travel_agencies_tour_operators" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TravelAgenciesTourOperators,
            "truck_stop_iteration" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TruckStopIteration,
            "truck_utility_trailer_rentals" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TruckUtilityTrailerRentals,
            "typesetting_plate_making_and_related_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TypesettingPlateMakingAndRelatedServices,
            "typewriter_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::TypewriterStores,
            "u_s_federal_government_agencies_or_departments" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::USFederalGovernmentAgenciesOrDepartments,
            "uniforms_commercial_clothing" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::UniformsCommercialClothing,
            "used_merchandise_and_secondhand_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::UsedMerchandiseAndSecondhandStores,
            "utilities" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Utilities,
            "variety_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VarietyStores,
            "veterinary_services" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VeterinaryServices,
            "video_amusement_game_supplies" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VideoAmusementGameSupplies,
            "video_game_arcades" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VideoGameArcades,
            "video_tape_rental_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VideoTapeRentalStores,
            "vocational_trade_schools" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::VocationalTradeSchools,
            "watch_jewelry_repair" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WatchJewelryRepair,
            "welding_repair" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WeldingRepair,
            "wholesale_clubs" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WholesaleClubs,
            "wig_and_toupee_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WigAndToupeeStores,
            "wires_money_orders" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WiresMoneyOrders,
            "womens_accessory_and_specialty_shops" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WomensAccessoryAndSpecialtyShops,
            "womens_ready_to_wear_stores" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WomensReadyToWearStores,
            "wrecking_and_salvage_yards" => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::WreckingAndSalvageYards,
            other => CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsNetworkData {
    #[doc = "Identifier assigned to the acquirer by the card network."]
    pub acquiring_institution_id: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsNetworkData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acquiring_institution_id", self.acquiring_institution_id.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct PresentPaymentMethodParamsCardPresent {
    #[doc = "The card number, as a string without any separators."]
    pub number: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for PresentPaymentMethodParamsCardPresent {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("number", self.number.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct RefundTestHelperIssuingTransactionParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The total amount to attempt to refund."]
    pub refund_amount: Option<i64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for RefundTestHelperIssuingTransactionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("refund_amount", self.refund_amount.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum SucceedInputCollectionParamsSkipNonRequiredInputs {
    All,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl SucceedInputCollectionParamsSkipNonRequiredInputs {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SucceedInputCollectionParamsSkipNonRequiredInputs::All => "all",
            SucceedInputCollectionParamsSkipNonRequiredInputs::None => "none",
            SucceedInputCollectionParamsSkipNonRequiredInputs::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for SucceedInputCollectionParamsSkipNonRequiredInputs {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SucceedInputCollectionParamsSkipNonRequiredInputs::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "all" => SucceedInputCollectionParamsSkipNonRequiredInputs::All,
            "none" => SucceedInputCollectionParamsSkipNonRequiredInputs::None,
            other => SucceedInputCollectionParamsSkipNonRequiredInputs::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for SucceedInputCollectionParamsSkipNonRequiredInputs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
