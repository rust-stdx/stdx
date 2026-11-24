#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardholderParamsPreferredLocalesItem {
    De,
    En,
    Es,
    Fr,
    It,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingCardholderParamsPreferredLocalesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardholderParamsPreferredLocalesItem::De => "de",
            CreateIssuingCardholderParamsPreferredLocalesItem::En => "en",
            CreateIssuingCardholderParamsPreferredLocalesItem::Es => "es",
            CreateIssuingCardholderParamsPreferredLocalesItem::Fr => "fr",
            CreateIssuingCardholderParamsPreferredLocalesItem::It => "it",
            CreateIssuingCardholderParamsPreferredLocalesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardholderParamsPreferredLocalesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardholderParamsPreferredLocalesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "de" => CreateIssuingCardholderParamsPreferredLocalesItem::De,
            "en" => CreateIssuingCardholderParamsPreferredLocalesItem::En,
            "es" => CreateIssuingCardholderParamsPreferredLocalesItem::Es,
            "fr" => CreateIssuingCardholderParamsPreferredLocalesItem::Fr,
            "it" => CreateIssuingCardholderParamsPreferredLocalesItem::It,
            other => CreateIssuingCardholderParamsPreferredLocalesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsPreferredLocalesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem {
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
    Miscellaneous,
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

#[cfg(feature = "issuing")]
impl CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AcRefrigerationRepair => "ac_refrigeration_repair",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AdvertisingServices => "advertising_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AgriculturalCooperative => "agricultural_cooperative",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AirlinesAirCarriers => "airlines_air_carriers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AirportsFlyingFields => "airports_flying_fields",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AmbulanceServices => "ambulance_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AmusementParksCarnivals => "amusement_parks_carnivals",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AntiqueReproductions => "antique_reproductions",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AntiqueShops => "antique_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Aquariums => "aquariums",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ArchitecturalSurveyingServices => "architectural_surveying_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ArtDealersAndGalleries => "art_dealers_and_galleries",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoBodyRepairShops => "auto_body_repair_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoPaintShops => "auto_paint_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoServiceShops => "auto_service_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomatedCashDisburse => "automated_cash_disburse",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomatedFuelDispensers => "automated_fuel_dispensers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomobileAssociations => "automobile_associations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomotiveTireStores => "automotive_tire_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BailAndBondPayments => "bail_and_bond_payments",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Bakeries => "bakeries",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BandsOrchestras => "bands_orchestras",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BarberAndBeautyShops => "barber_and_beauty_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BettingCasinoGambling => "betting_casino_gambling",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BicycleShops => "bicycle_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BilliardPoolEstablishments => "billiard_pool_establishments",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BoatDealers => "boat_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BoatRentalsAndLeases => "boat_rentals_and_leases",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BookStores => "book_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BowlingAlleys => "bowling_alleys",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BusLines => "bus_lines",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BusinessSecretarialSchools => "business_secretarial_schools",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BuyingShoppingServices => "buying_shopping_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CableSatelliteAndOtherPayTelevisionAndRadio => "cable_satellite_and_other_pay_television_and_radio",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarRentalAgencies => "car_rental_agencies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarWashes => "car_washes",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarpentryServices => "carpentry_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Caterers => "caterers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CharitableAndSocialServiceOrganizationsFundraising => "charitable_and_social_service_organizations_fundraising",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChildCareServices => "child_care_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Chiropractors => "chiropractors",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CigarStoresAndStands => "cigar_stores_and_stands",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CleaningAndMaintenance => "cleaning_and_maintenance",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ClothingRental => "clothing_rental",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CollegesUniversities => "colleges_universities",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommercialEquipment => "commercial_equipment",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommercialFootwear => "commercial_footwear",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerNetworkServices => "computer_network_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerProgramming => "computer_programming",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerRepair => "computer_repair",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerSoftwareStores => "computer_software_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ConcreteWorkServices => "concrete_work_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ConstructionMaterials => "construction_materials",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ConsultingPublicRelations => "consulting_public_relations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CorrespondenceSchools => "correspondence_schools",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CosmeticStores => "cosmetic_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CounselingServices => "counseling_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CountryClubs => "country_clubs",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CourierServices => "courier_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CourtCosts => "court_costs",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CreditReportingAgencies => "credit_reporting_agencies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CruiseLines => "cruise_lines",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DairyProductsStores => "dairy_products_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DanceHallStudiosSchools => "dance_hall_studios_schools",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DatingEscortServices => "dating_escort_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DentistsOrthodontists => "dentists_orthodontists",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DepartmentStores => "department_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DetectiveAgencies => "detective_agencies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsApplications => "digital_goods_applications",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsGames => "digital_goods_games",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsMedia => "digital_goods_media",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingCombinationCatalogAndRetailMerchant => "direct_marketing_combination_catalog_and_retail_merchant",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingOther => "direct_marketing_other",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingSubscription => "direct_marketing_subscription",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingTravel => "direct_marketing_travel",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DiscountStores => "discount_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Doctors => "doctors",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DoorToDoorSales => "door_to_door_sales",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DraperyWindowCoveringAndUpholsteryStores => "drapery_window_covering_and_upholstery_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DrinkingPlaces => "drinking_places",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DrugsDrugProprietariesAndDruggistSundries => "drugs_drug_proprietaries_and_druggist_sundries",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DryCleaners => "dry_cleaners",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DurableGoods => "durable_goods",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DutyFreeStores => "duty_free_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EatingPlacesRestaurants => "eating_places_restaurants",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EducationalServices => "educational_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricRazorStores => "electric_razor_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricVehicleCharging => "electric_vehicle_charging",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricalServices => "electrical_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectronicsRepairShops => "electronics_repair_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectronicsStores => "electronics_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElementarySecondarySchools => "elementary_secondary_schools",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EmergencyServicesGcasVisaUseOnly => "emergency_services_gcas_visa_use_only",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EmploymentTempAgencies => "employment_temp_agencies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EquipmentRental => "equipment_rental",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ExterminatingServices => "exterminating_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FamilyClothingStores => "family_clothing_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FastFoodRestaurants => "fast_food_restaurants",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FinancialInstitutions => "financial_institutions",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FinesGovernmentAdministrativeEntities => "fines_government_administrative_entities",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FireplaceFireplaceScreensAndAccessoriesStores => "fireplace_fireplace_screens_and_accessories_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FloorCoveringStores => "floor_covering_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Florists => "florists",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FloristsSuppliesNurseryStockAndFlowers => "florists_supplies_nursery_stock_and_flowers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FuneralServicesCrematories => "funeral_services_crematories",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => "furniture_home_furnishings_and_equipment_stores_except_appliances",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FurriersAndFurShops => "furriers_and_fur_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GeneralServices => "general_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GlasswareCrystalStores => "glassware_crystal_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GolfCoursesPublic => "golf_courses_public",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentLicensedHorseDogRacingUsRegionOnly => "government_licensed_horse_dog_racing_us_region_only",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly => "government_licensed_online_casions_online_gambling_us_region_only",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentOwnedLotteriesNonUsRegion => "government_owned_lotteries_non_us_region",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentOwnedLotteriesUsRegionOnly => "government_owned_lotteries_us_region_only",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentServices => "government_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HardwareStores => "hardware_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HealthAndBeautySpas => "health_and_beauty_spas",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HeatingPlumbingAC => "heating_plumbing_a_c",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Hospitals => "hospitals",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HouseholdApplianceStores => "household_appliance_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::IndustrialSupplies => "industrial_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::InformationRetrievalServices => "information_retrieval_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::InsuranceDefault => "insurance_default",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::IntraCompanyPurchases => "intra_company_purchases",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::JewelryStoresWatchesClocksAndSilverwareStores => "jewelry_stores_watches_clocks_and_silverware_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LandscapingServices => "landscaping_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Laundries => "laundries",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LaundryCleaningServices => "laundry_cleaning_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LegalServicesAttorneys => "legal_services_attorneys",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ManualCashDisburse => "manual_cash_disburse",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Marketplaces => "marketplaces",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MassageParlors => "massage_parlors",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MedicalAndDentalLabs => "medical_and_dental_labs",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => "medical_dental_ophthalmic_and_hospital_equipment_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MedicalServices => "medical_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MembershipOrganizations => "membership_organizations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MensAndBoysClothingAndAccessoriesStores => "mens_and_boys_clothing_and_accessories_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MensWomensClothingStores => "mens_womens_clothing_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MetalServiceCenters => "metal_service_centers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Miscellaneous => "miscellaneous",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousApparelAndAccessoryShops => "miscellaneous_apparel_and_accessory_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousBusinessServices => "miscellaneous_business_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousFoodStores => "miscellaneous_food_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousGeneralServices => "miscellaneous_general_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousHomeFurnishingSpecialtyStores => "miscellaneous_home_furnishing_specialty_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MobileHomeDealers => "mobile_home_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotionPictureTheaters => "motion_picture_theaters",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorHomesDealers => "motor_homes_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MusicStoresMusicalInstrumentsPianosAndSheetMusic => "music_stores_musical_instruments_pianos_and_sheet_music",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NonFiMoneyOrders => "non_fi_money_orders",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NondurableGoods => "nondurable_goods",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NursingPersonalCare => "nursing_personal_care",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OpticiansEyeglasses => "opticians_eyeglasses",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Osteopaths => "osteopaths",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ParkingLotsGarages => "parking_lots_garages",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PassengerRailways => "passenger_railways",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PawnShops => "pawn_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PhotoDeveloping => "photo_developing",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => "photographic_photocopy_microfilm_equipment_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PhotographicStudios => "photographic_studios",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PictureVideoProduction => "picture_video_production",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PoliticalOrganizations => "political_organizations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PostalServicesGovernmentOnly => "postal_services_government_only",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PreciousStonesAndMetalsWatchesAndJewelry => "precious_stones_and_metals_watches_and_jewelry",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ProfessionalServices => "professional_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Railroads => "railroads",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RecordStores => "record_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ReligiousGoodsStores => "religious_goods_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ReligiousOrganizations => "religious_organizations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SecretarialSupportServices => "secretarial_support_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SecurityBrokersDealers => "security_brokers_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ServiceStations => "service_stations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SewingNeedleworkFabricAndPieceGoodsStores => "sewing_needlework_fabric_and_piece_goods_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ShoeStores => "shoe_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SmallApplianceRepair => "small_appliance_repair",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SnowmobileDealers => "snowmobile_dealers",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SpecialTradeServices => "special_trade_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SpecialtyCleaning => "specialty_cleaning",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportingGoodsStores => "sporting_goods_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportingRecreationCamps => "sporting_recreation_camps",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportsClubsFields => "sports_clubs_fields",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::StampAndCoinStores => "stamp_and_coin_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::StationaryOfficeSuppliesPrintingAndWritingPaper => "stationary_office_supplies_printing_and_writing_paper",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::StationeryStoresOfficeAndSchoolSupplyStores => "stationery_stores_office_and_school_supply_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SwimmingPoolsSales => "swimming_pools_sales",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TUiTravelGermany => "t_ui_travel_germany",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TailorsAlterations => "tailors_alterations",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TaxPreparationServices => "tax_preparation_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TaxicabsLimousines => "taxicabs_limousines",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TelecommunicationEquipmentAndTelephoneSales => "telecommunication_equipment_and_telephone_sales",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TelecommunicationServices => "telecommunication_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TelegraphServices => "telegraph_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TentAndAwningShops => "tent_and_awning_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TestingLaboratories => "testing_laboratories",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Timeshares => "timeshares",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TireRetreadingAndRepair => "tire_retreading_and_repair",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TollsBridgeFees => "tolls_bridge_fees",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TowingServices => "towing_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TransportationServices => "transportation_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TruckStopIteration => "truck_stop_iteration",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TypesettingPlateMakingAndRelatedServices => "typesetting_plate_making_and_related_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TypewriterStores => "typewriter_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::USFederalGovernmentAgenciesOrDepartments => "u_s_federal_government_agencies_or_departments",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::UniformsCommercialClothing => "uniforms_commercial_clothing",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Utilities => "utilities",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VarietyStores => "variety_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VeterinaryServices => "veterinary_services",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VideoGameArcades => "video_game_arcades",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VideoTapeRentalStores => "video_tape_rental_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VocationalTradeSchools => "vocational_trade_schools",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WatchJewelryRepair => "watch_jewelry_repair",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WeldingRepair => "welding_repair",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WholesaleClubs => "wholesale_clubs",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WigAndToupeeStores => "wig_and_toupee_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WiresMoneyOrders => "wires_money_orders",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
            CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ac_refrigeration_repair" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AcRefrigerationRepair,
            "accounting_bookkeeping_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AccountingBookkeepingServices,
            "advertising_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AdvertisingServices,
            "agricultural_cooperative" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AgriculturalCooperative,
            "airlines_air_carriers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AirlinesAirCarriers,
            "airports_flying_fields" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AirportsFlyingFields,
            "ambulance_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AmbulanceServices,
            "amusement_parks_carnivals" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AmusementParksCarnivals,
            "antique_reproductions" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AntiqueReproductions,
            "antique_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AntiqueShops,
            "aquariums" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Aquariums,
            "architectural_surveying_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ArchitecturalSurveyingServices,
            "art_dealers_and_galleries" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ArtDealersAndGalleries,
            "artists_supply_and_craft_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ArtistsSupplyAndCraftShops,
            "auto_and_home_supply_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoAndHomeSupplyStores,
            "auto_body_repair_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoBodyRepairShops,
            "auto_paint_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoPaintShops,
            "auto_service_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutoServiceShops,
            "automated_cash_disburse" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomatedCashDisburse,
            "automated_fuel_dispensers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomatedFuelDispensers,
            "automobile_associations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomobileAssociations,
            "automotive_parts_and_accessories_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomotivePartsAndAccessoriesStores,
            "automotive_tire_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::AutomotiveTireStores,
            "bail_and_bond_payments" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BailAndBondPayments,
            "bakeries" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Bakeries,
            "bands_orchestras" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BandsOrchestras,
            "barber_and_beauty_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BarberAndBeautyShops,
            "betting_casino_gambling" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BettingCasinoGambling,
            "bicycle_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BicycleShops,
            "billiard_pool_establishments" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BilliardPoolEstablishments,
            "boat_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BoatDealers,
            "boat_rentals_and_leases" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BoatRentalsAndLeases,
            "book_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BookStores,
            "books_periodicals_and_newspapers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BooksPeriodicalsAndNewspapers,
            "bowling_alleys" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BowlingAlleys,
            "bus_lines" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BusLines,
            "business_secretarial_schools" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BusinessSecretarialSchools,
            "buying_shopping_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::BuyingShoppingServices,
            "cable_satellite_and_other_pay_television_and_radio" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CableSatelliteAndOtherPayTelevisionAndRadio,
            "camera_and_photographic_supply_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CameraAndPhotographicSupplyStores,
            "candy_nut_and_confectionery_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CandyNutAndConfectioneryStores,
            "car_and_truck_dealers_new_used" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarAndTruckDealersNewUsed,
            "car_and_truck_dealers_used_only" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarAndTruckDealersUsedOnly,
            "car_rental_agencies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarRentalAgencies,
            "car_washes" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarWashes,
            "carpentry_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarpentryServices,
            "carpet_upholstery_cleaning" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CarpetUpholsteryCleaning,
            "caterers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Caterers,
            "charitable_and_social_service_organizations_fundraising" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CharitableAndSocialServiceOrganizationsFundraising,
            "chemicals_and_allied_products" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChemicalsAndAlliedProducts,
            "child_care_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChildCareServices,
            "childrens_and_infants_wear_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChildrensAndInfantsWearStores,
            "chiropodists_podiatrists" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ChiropodistsPodiatrists,
            "chiropractors" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Chiropractors,
            "cigar_stores_and_stands" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CigarStoresAndStands,
            "civic_social_fraternal_associations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CivicSocialFraternalAssociations,
            "cleaning_and_maintenance" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CleaningAndMaintenance,
            "clothing_rental" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ClothingRental,
            "colleges_universities" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CollegesUniversities,
            "commercial_equipment" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommercialEquipment,
            "commercial_footwear" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommercialFootwear,
            "commercial_photography_art_and_graphics" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommercialPhotographyArtAndGraphics,
            "commuter_transport_and_ferries" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CommuterTransportAndFerries,
            "computer_network_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerNetworkServices,
            "computer_programming" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerProgramming,
            "computer_repair" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerRepair,
            "computer_software_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputerSoftwareStores,
            "computers_peripherals_and_software" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ComputersPeripheralsAndSoftware,
            "concrete_work_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ConcreteWorkServices,
            "construction_materials" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ConstructionMaterials,
            "consulting_public_relations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ConsultingPublicRelations,
            "correspondence_schools" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CorrespondenceSchools,
            "cosmetic_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CosmeticStores,
            "counseling_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CounselingServices,
            "country_clubs" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CountryClubs,
            "courier_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CourierServices,
            "court_costs" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CourtCosts,
            "credit_reporting_agencies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CreditReportingAgencies,
            "cruise_lines" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::CruiseLines,
            "dairy_products_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DairyProductsStores,
            "dance_hall_studios_schools" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DanceHallStudiosSchools,
            "dating_escort_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DatingEscortServices,
            "dentists_orthodontists" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DentistsOrthodontists,
            "department_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DepartmentStores,
            "detective_agencies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DetectiveAgencies,
            "digital_goods_applications" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsApplications,
            "digital_goods_games" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsGames,
            "digital_goods_large_volume" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsLargeVolume,
            "digital_goods_media" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DigitalGoodsMedia,
            "direct_marketing_catalog_merchant" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingCatalogMerchant,
            "direct_marketing_combination_catalog_and_retail_merchant" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingCombinationCatalogAndRetailMerchant,
            "direct_marketing_inbound_telemarketing" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingInboundTelemarketing,
            "direct_marketing_insurance_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingInsuranceServices,
            "direct_marketing_other" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingOther,
            "direct_marketing_outbound_telemarketing" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingOutboundTelemarketing,
            "direct_marketing_subscription" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingSubscription,
            "direct_marketing_travel" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DirectMarketingTravel,
            "discount_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DiscountStores,
            "doctors" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Doctors,
            "door_to_door_sales" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DoorToDoorSales,
            "drapery_window_covering_and_upholstery_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DraperyWindowCoveringAndUpholsteryStores,
            "drinking_places" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DrinkingPlaces,
            "drug_stores_and_pharmacies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DrugStoresAndPharmacies,
            "drugs_drug_proprietaries_and_druggist_sundries" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DrugsDrugProprietariesAndDruggistSundries,
            "dry_cleaners" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DryCleaners,
            "durable_goods" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DurableGoods,
            "duty_free_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::DutyFreeStores,
            "eating_places_restaurants" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EatingPlacesRestaurants,
            "educational_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EducationalServices,
            "electric_razor_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricRazorStores,
            "electric_vehicle_charging" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricVehicleCharging,
            "electrical_parts_and_equipment" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricalPartsAndEquipment,
            "electrical_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectricalServices,
            "electronics_repair_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectronicsRepairShops,
            "electronics_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElectronicsStores,
            "elementary_secondary_schools" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ElementarySecondarySchools,
            "emergency_services_gcas_visa_use_only" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EmergencyServicesGcasVisaUseOnly,
            "employment_temp_agencies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EmploymentTempAgencies,
            "equipment_rental" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::EquipmentRental,
            "exterminating_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ExterminatingServices,
            "family_clothing_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FamilyClothingStores,
            "fast_food_restaurants" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FastFoodRestaurants,
            "financial_institutions" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FinancialInstitutions,
            "fines_government_administrative_entities" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FinesGovernmentAdministrativeEntities,
            "fireplace_fireplace_screens_and_accessories_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FireplaceFireplaceScreensAndAccessoriesStores,
            "floor_covering_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FloorCoveringStores,
            "florists" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Florists,
            "florists_supplies_nursery_stock_and_flowers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FloristsSuppliesNurseryStockAndFlowers,
            "freezer_and_locker_meat_provisioners" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FreezerAndLockerMeatProvisioners,
            "fuel_dealers_non_automotive" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FuelDealersNonAutomotive,
            "funeral_services_crematories" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FuneralServicesCrematories,
            "furniture_home_furnishings_and_equipment_stores_except_appliances" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
            "furniture_repair_refinishing" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FurnitureRepairRefinishing,
            "furriers_and_fur_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::FurriersAndFurShops,
            "general_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GeneralServices,
            "gift_card_novelty_and_souvenir_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GiftCardNoveltyAndSouvenirShops,
            "glass_paint_and_wallpaper_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GlassPaintAndWallpaperStores,
            "glassware_crystal_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GlasswareCrystalStores,
            "golf_courses_public" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GolfCoursesPublic,
            "government_licensed_horse_dog_racing_us_region_only" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentLicensedHorseDogRacingUsRegionOnly,
            "government_licensed_online_casions_online_gambling_us_region_only" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentLicensedOnlineCasionsOnlineGamblingUsRegionOnly,
            "government_owned_lotteries_non_us_region" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentOwnedLotteriesNonUsRegion,
            "government_owned_lotteries_us_region_only" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentOwnedLotteriesUsRegionOnly,
            "government_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GovernmentServices,
            "grocery_stores_supermarkets" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::GroceryStoresSupermarkets,
            "hardware_equipment_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HardwareEquipmentAndSupplies,
            "hardware_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HardwareStores,
            "health_and_beauty_spas" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HealthAndBeautySpas,
            "hearing_aids_sales_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HearingAidsSalesAndSupplies,
            "heating_plumbing_a_c" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HeatingPlumbingAC,
            "hobby_toy_and_game_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HobbyToyAndGameShops,
            "home_supply_warehouse_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HomeSupplyWarehouseStores,
            "hospitals" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Hospitals,
            "hotels_motels_and_resorts" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HotelsMotelsAndResorts,
            "household_appliance_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::HouseholdApplianceStores,
            "industrial_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::IndustrialSupplies,
            "information_retrieval_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::InformationRetrievalServices,
            "insurance_default" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::InsuranceDefault,
            "insurance_underwriting_premiums" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::InsuranceUnderwritingPremiums,
            "intra_company_purchases" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::IntraCompanyPurchases,
            "jewelry_stores_watches_clocks_and_silverware_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::JewelryStoresWatchesClocksAndSilverwareStores,
            "landscaping_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LandscapingServices,
            "laundries" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Laundries,
            "laundry_cleaning_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LaundryCleaningServices,
            "legal_services_attorneys" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LegalServicesAttorneys,
            "luggage_and_leather_goods_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LuggageAndLeatherGoodsStores,
            "lumber_building_materials_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::LumberBuildingMaterialsStores,
            "manual_cash_disburse" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ManualCashDisburse,
            "marinas_service_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MarinasServiceAndSupplies,
            "marketplaces" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Marketplaces,
            "masonry_stonework_and_plaster" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MasonryStoneworkAndPlaster,
            "massage_parlors" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MassageParlors,
            "medical_and_dental_labs" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MedicalAndDentalLabs,
            "medical_dental_ophthalmic_and_hospital_equipment_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
            "medical_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MedicalServices,
            "membership_organizations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MembershipOrganizations,
            "mens_and_boys_clothing_and_accessories_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MensAndBoysClothingAndAccessoriesStores,
            "mens_womens_clothing_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MensWomensClothingStores,
            "metal_service_centers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MetalServiceCenters,
            "miscellaneous" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Miscellaneous,
            "miscellaneous_apparel_and_accessory_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousApparelAndAccessoryShops,
            "miscellaneous_auto_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousAutoDealers,
            "miscellaneous_business_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousBusinessServices,
            "miscellaneous_food_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousFoodStores,
            "miscellaneous_general_merchandise" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousGeneralMerchandise,
            "miscellaneous_general_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousGeneralServices,
            "miscellaneous_home_furnishing_specialty_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousHomeFurnishingSpecialtyStores,
            "miscellaneous_publishing_and_printing" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousPublishingAndPrinting,
            "miscellaneous_recreation_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousRecreationServices,
            "miscellaneous_repair_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousRepairShops,
            "miscellaneous_specialty_retail" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MiscellaneousSpecialtyRetail,
            "mobile_home_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MobileHomeDealers,
            "motion_picture_theaters" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotionPictureTheaters,
            "motor_freight_carriers_and_trucking" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorFreightCarriersAndTrucking,
            "motor_homes_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorHomesDealers,
            "motor_vehicle_supplies_and_new_parts" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorVehicleSuppliesAndNewParts,
            "motorcycle_shops_and_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorcycleShopsAndDealers,
            "motorcycle_shops_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MotorcycleShopsDealers,
            "music_stores_musical_instruments_pianos_and_sheet_music" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::MusicStoresMusicalInstrumentsPianosAndSheetMusic,
            "news_dealers_and_newsstands" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NewsDealersAndNewsstands,
            "non_fi_money_orders" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NonFiMoneyOrders,
            "non_fi_stored_value_card_purchase_load" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NonFiStoredValueCardPurchaseLoad,
            "nondurable_goods" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NondurableGoods,
            "nurseries_lawn_and_garden_supply_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NurseriesLawnAndGardenSupplyStores,
            "nursing_personal_care" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::NursingPersonalCare,
            "office_and_commercial_furniture" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OfficeAndCommercialFurniture,
            "opticians_eyeglasses" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OpticiansEyeglasses,
            "optometrists_ophthalmologist" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OptometristsOphthalmologist,
            "orthopedic_goods_prosthetic_devices" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::OrthopedicGoodsProstheticDevices,
            "osteopaths" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Osteopaths,
            "package_stores_beer_wine_and_liquor" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PackageStoresBeerWineAndLiquor,
            "paints_varnishes_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PaintsVarnishesAndSupplies,
            "parking_lots_garages" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ParkingLotsGarages,
            "passenger_railways" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PassengerRailways,
            "pawn_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PawnShops,
            "pet_shops_pet_food_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PetShopsPetFoodAndSupplies,
            "petroleum_and_petroleum_products" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PetroleumAndPetroleumProducts,
            "photo_developing" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PhotoDeveloping,
            "photographic_photocopy_microfilm_equipment_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
            "photographic_studios" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PhotographicStudios,
            "picture_video_production" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PictureVideoProduction,
            "piece_goods_notions_and_other_dry_goods" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PieceGoodsNotionsAndOtherDryGoods,
            "plumbing_heating_equipment_and_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PlumbingHeatingEquipmentAndSupplies,
            "political_organizations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PoliticalOrganizations,
            "postal_services_government_only" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PostalServicesGovernmentOnly,
            "precious_stones_and_metals_watches_and_jewelry" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PreciousStonesAndMetalsWatchesAndJewelry,
            "professional_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ProfessionalServices,
            "public_warehousing_and_storage" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::PublicWarehousingAndStorage,
            "quick_copy_repro_and_blueprint" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::QuickCopyReproAndBlueprint,
            "railroads" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Railroads,
            "real_estate_agents_and_managers_rentals" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RealEstateAgentsAndManagersRentals,
            "record_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RecordStores,
            "recreational_vehicle_rentals" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RecreationalVehicleRentals,
            "religious_goods_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ReligiousGoodsStores,
            "religious_organizations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ReligiousOrganizations,
            "roofing_siding_sheet_metal" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::RoofingSidingSheetMetal,
            "secretarial_support_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SecretarialSupportServices,
            "security_brokers_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SecurityBrokersDealers,
            "service_stations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ServiceStations,
            "sewing_needlework_fabric_and_piece_goods_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SewingNeedleworkFabricAndPieceGoodsStores,
            "shoe_repair_hat_cleaning" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ShoeRepairHatCleaning,
            "shoe_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::ShoeStores,
            "small_appliance_repair" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SmallApplianceRepair,
            "snowmobile_dealers" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SnowmobileDealers,
            "special_trade_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SpecialTradeServices,
            "specialty_cleaning" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SpecialtyCleaning,
            "sporting_goods_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportingGoodsStores,
            "sporting_recreation_camps" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportingRecreationCamps,
            "sports_and_riding_apparel_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportsAndRidingApparelStores,
            "sports_clubs_fields" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SportsClubsFields,
            "stamp_and_coin_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::StampAndCoinStores,
            "stationary_office_supplies_printing_and_writing_paper" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::StationaryOfficeSuppliesPrintingAndWritingPaper,
            "stationery_stores_office_and_school_supply_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::StationeryStoresOfficeAndSchoolSupplyStores,
            "swimming_pools_sales" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::SwimmingPoolsSales,
            "t_ui_travel_germany" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TUiTravelGermany,
            "tailors_alterations" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TailorsAlterations,
            "tax_payments_government_agencies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TaxPaymentsGovernmentAgencies,
            "tax_preparation_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TaxPreparationServices,
            "taxicabs_limousines" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TaxicabsLimousines,
            "telecommunication_equipment_and_telephone_sales" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TelecommunicationEquipmentAndTelephoneSales,
            "telecommunication_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TelecommunicationServices,
            "telegraph_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TelegraphServices,
            "tent_and_awning_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TentAndAwningShops,
            "testing_laboratories" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TestingLaboratories,
            "theatrical_ticket_agencies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TheatricalTicketAgencies,
            "timeshares" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Timeshares,
            "tire_retreading_and_repair" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TireRetreadingAndRepair,
            "tolls_bridge_fees" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TollsBridgeFees,
            "tourist_attractions_and_exhibits" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TouristAttractionsAndExhibits,
            "towing_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TowingServices,
            "trailer_parks_campgrounds" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TrailerParksCampgrounds,
            "transportation_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TransportationServices,
            "travel_agencies_tour_operators" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TravelAgenciesTourOperators,
            "truck_stop_iteration" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TruckStopIteration,
            "truck_utility_trailer_rentals" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TruckUtilityTrailerRentals,
            "typesetting_plate_making_and_related_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TypesettingPlateMakingAndRelatedServices,
            "typewriter_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::TypewriterStores,
            "u_s_federal_government_agencies_or_departments" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::USFederalGovernmentAgenciesOrDepartments,
            "uniforms_commercial_clothing" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::UniformsCommercialClothing,
            "used_merchandise_and_secondhand_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::UsedMerchandiseAndSecondhandStores,
            "utilities" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Utilities,
            "variety_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VarietyStores,
            "veterinary_services" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VeterinaryServices,
            "video_amusement_game_supplies" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VideoAmusementGameSupplies,
            "video_game_arcades" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VideoGameArcades,
            "video_tape_rental_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VideoTapeRentalStores,
            "vocational_trade_schools" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::VocationalTradeSchools,
            "watch_jewelry_repair" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WatchJewelryRepair,
            "welding_repair" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WeldingRepair,
            "wholesale_clubs" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WholesaleClubs,
            "wig_and_toupee_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WigAndToupeeStores,
            "wires_money_orders" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WiresMoneyOrders,
            "womens_accessory_and_specialty_shops" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WomensAccessoryAndSpecialtyShops,
            "womens_ready_to_wear_stores" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WomensReadyToWearStores,
            "wrecking_and_salvage_yards" => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::WreckingAndSalvageYards,
            other => CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsSpendingControlsAllowedCategoriesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::AllTime => "all_time",
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Daily => "daily",
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Monthly => "monthly",
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::PerAuthorization => {
                "per_authorization"
            }
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Weekly => "weekly",
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Yearly => "yearly",
            CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "all_time" => CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::AllTime,
            "daily" => CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Daily,
            "monthly" => CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Monthly,
            "per_authorization" => {
                CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::PerAuthorization
            }
            "weekly" => CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Weekly,
            "yearly" => CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Yearly,
            other => {
                CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingCardholderParamsSpendingControlsSpendingLimitsItemInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct CreateIssuingDisputeParamsEvidenceDuplicate {
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute."]
    pub additional_documentation: Option<json::OwnedValue>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the produ..."]
    pub card_statement: Option<json::OwnedValue>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had ..."]
    pub cash_receipt: Option<json::OwnedValue>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was..."]
    pub check_image: Option<json::OwnedValue>,
    #[doc = "Explanation of why the cardholder is disputing this transaction."]
    pub explanation: Option<json::OwnedValue>,
    #[doc = "Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of."]
    pub original_transaction: Option<String>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for CreateIssuingDisputeParamsEvidenceDuplicate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("original_transaction", self.original_transaction.as_ref());
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
pub struct IssuingAuthorizationAmountDetails {
    #[doc = "The fee charged by the ATM for the cash withdrawal."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub atm_fee: Option<i64>,
    #[doc = "The amount of cash requested by the cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashback_amount: Option<i64>,
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
pub enum IssuingAuthorizationCardPresence {
    NotPresent,
    Present,
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
impl IssuingAuthorizationCardPresence {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationCardPresence::NotPresent => "not_present",
            IssuingAuthorizationCardPresence::Present => "present",
            IssuingAuthorizationCardPresence::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingAuthorizationCardPresence {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationCardPresence::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "not_present" => IssuingAuthorizationCardPresence::NotPresent,
            "present" => IssuingAuthorizationCardPresence::Present,
            other => IssuingAuthorizationCardPresence::Unknown(other.to_string()),
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
pub enum IssuingAuthorizationFleetDataPurchaseType {
    FuelAndNonFuelPurchase,
    FuelPurchase,
    NonFuelPurchase,
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
impl IssuingAuthorizationFleetDataPurchaseType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationFleetDataPurchaseType::FuelAndNonFuelPurchase => "fuel_and_non_fuel_purchase",
            IssuingAuthorizationFleetDataPurchaseType::FuelPurchase => "fuel_purchase",
            IssuingAuthorizationFleetDataPurchaseType::NonFuelPurchase => "non_fuel_purchase",
            IssuingAuthorizationFleetDataPurchaseType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingAuthorizationFleetDataPurchaseType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationFleetDataPurchaseType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fuel_and_non_fuel_purchase" => IssuingAuthorizationFleetDataPurchaseType::FuelAndNonFuelPurchase,
            "fuel_purchase" => IssuingAuthorizationFleetDataPurchaseType::FuelPurchase,
            "non_fuel_purchase" => IssuingAuthorizationFleetDataPurchaseType::NonFuelPurchase,
            other => IssuingAuthorizationFleetDataPurchaseType::Unknown(other.to_string()),
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
pub enum IssuingAuthorizationFraudChallengeStatus {
    Expired,
    Pending,
    Rejected,
    Undeliverable,
    Verified,
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
impl IssuingAuthorizationFraudChallengeStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationFraudChallengeStatus::Expired => "expired",
            IssuingAuthorizationFraudChallengeStatus::Pending => "pending",
            IssuingAuthorizationFraudChallengeStatus::Rejected => "rejected",
            IssuingAuthorizationFraudChallengeStatus::Undeliverable => "undeliverable",
            IssuingAuthorizationFraudChallengeStatus::Verified => "verified",
            IssuingAuthorizationFraudChallengeStatus::Unknown(value) => value,
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
impl Default for IssuingAuthorizationFraudChallengeStatus {
    fn default() -> Self {
        IssuingAuthorizationFraudChallengeStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingAuthorizationFraudChallengeStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationFraudChallengeStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "expired" => IssuingAuthorizationFraudChallengeStatus::Expired,
            "pending" => IssuingAuthorizationFraudChallengeStatus::Pending,
            "rejected" => IssuingAuthorizationFraudChallengeStatus::Rejected,
            "undeliverable" => IssuingAuthorizationFraudChallengeStatus::Undeliverable,
            "verified" => IssuingAuthorizationFraudChallengeStatus::Verified,
            other => IssuingAuthorizationFraudChallengeStatus::Unknown(other.to_string()),
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
pub enum IssuingAuthorizationRequestReason {
    AccountDisabled,
    CardActive,
    CardCanceled,
    CardExpired,
    CardInactive,
    CardholderBlocked,
    CardholderInactive,
    CardholderVerificationRequired,
    InsecureAuthorizationMethod,
    InsufficientFunds,
    NetworkFallback,
    NotAllowed,
    PinBlocked,
    SpendingControls,
    StripeInternalError,
    SuspectedFraud,
    VerificationFailed,
    WebhookApproved,
    WebhookDeclined,
    WebhookError,
    WebhookTimeout,
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
impl IssuingAuthorizationRequestReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingAuthorizationRequestReason::AccountDisabled => "account_disabled",
            IssuingAuthorizationRequestReason::CardActive => "card_active",
            IssuingAuthorizationRequestReason::CardCanceled => "card_canceled",
            IssuingAuthorizationRequestReason::CardExpired => "card_expired",
            IssuingAuthorizationRequestReason::CardInactive => "card_inactive",
            IssuingAuthorizationRequestReason::CardholderBlocked => "cardholder_blocked",
            IssuingAuthorizationRequestReason::CardholderInactive => "cardholder_inactive",
            IssuingAuthorizationRequestReason::CardholderVerificationRequired => "cardholder_verification_required",
            IssuingAuthorizationRequestReason::InsecureAuthorizationMethod => "insecure_authorization_method",
            IssuingAuthorizationRequestReason::InsufficientFunds => "insufficient_funds",
            IssuingAuthorizationRequestReason::NetworkFallback => "network_fallback",
            IssuingAuthorizationRequestReason::NotAllowed => "not_allowed",
            IssuingAuthorizationRequestReason::PinBlocked => "pin_blocked",
            IssuingAuthorizationRequestReason::SpendingControls => "spending_controls",
            IssuingAuthorizationRequestReason::StripeInternalError => "stripe_internal_error",
            IssuingAuthorizationRequestReason::SuspectedFraud => "suspected_fraud",
            IssuingAuthorizationRequestReason::VerificationFailed => "verification_failed",
            IssuingAuthorizationRequestReason::WebhookApproved => "webhook_approved",
            IssuingAuthorizationRequestReason::WebhookDeclined => "webhook_declined",
            IssuingAuthorizationRequestReason::WebhookError => "webhook_error",
            IssuingAuthorizationRequestReason::WebhookTimeout => "webhook_timeout",
            IssuingAuthorizationRequestReason::Unknown(value) => value,
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
impl Default for IssuingAuthorizationRequestReason {
    fn default() -> Self {
        IssuingAuthorizationRequestReason::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingAuthorizationRequestReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingAuthorizationRequestReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_disabled" => IssuingAuthorizationRequestReason::AccountDisabled,
            "card_active" => IssuingAuthorizationRequestReason::CardActive,
            "card_canceled" => IssuingAuthorizationRequestReason::CardCanceled,
            "card_expired" => IssuingAuthorizationRequestReason::CardExpired,
            "card_inactive" => IssuingAuthorizationRequestReason::CardInactive,
            "cardholder_blocked" => IssuingAuthorizationRequestReason::CardholderBlocked,
            "cardholder_inactive" => IssuingAuthorizationRequestReason::CardholderInactive,
            "cardholder_verification_required" => IssuingAuthorizationRequestReason::CardholderVerificationRequired,
            "insecure_authorization_method" => IssuingAuthorizationRequestReason::InsecureAuthorizationMethod,
            "insufficient_funds" => IssuingAuthorizationRequestReason::InsufficientFunds,
            "network_fallback" => IssuingAuthorizationRequestReason::NetworkFallback,
            "not_allowed" => IssuingAuthorizationRequestReason::NotAllowed,
            "pin_blocked" => IssuingAuthorizationRequestReason::PinBlocked,
            "spending_controls" => IssuingAuthorizationRequestReason::SpendingControls,
            "stripe_internal_error" => IssuingAuthorizationRequestReason::StripeInternalError,
            "suspected_fraud" => IssuingAuthorizationRequestReason::SuspectedFraud,
            "verification_failed" => IssuingAuthorizationRequestReason::VerificationFailed,
            "webhook_approved" => IssuingAuthorizationRequestReason::WebhookApproved,
            "webhook_declined" => IssuingAuthorizationRequestReason::WebhookDeclined,
            "webhook_error" => IssuingAuthorizationRequestReason::WebhookError,
            "webhook_timeout" => IssuingAuthorizationRequestReason::WebhookTimeout,
            other => IssuingAuthorizationRequestReason::Unknown(other.to_string()),
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
pub enum IssuingCardObject {
    IssuingCard,
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
impl IssuingCardObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingCardObject::IssuingCard => "issuing.card",
            IssuingCardObject::Unknown(value) => value,
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
impl Default for IssuingCardObject {
    fn default() -> Self {
        IssuingCardObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingCardObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingCardObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.card" => IssuingCardObject::IssuingCard,
            other => IssuingCardObject::Unknown(other.to_string()),
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
pub enum IssuingCardShippingAddressValidationResult {
    Indeterminate,
    LikelyDeliverable,
    LikelyUndeliverable,
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
impl IssuingCardShippingAddressValidationResult {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingCardShippingAddressValidationResult::Indeterminate => "indeterminate",
            IssuingCardShippingAddressValidationResult::LikelyDeliverable => "likely_deliverable",
            IssuingCardShippingAddressValidationResult::LikelyUndeliverable => "likely_undeliverable",
            IssuingCardShippingAddressValidationResult::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for IssuingCardShippingAddressValidationResult {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingCardShippingAddressValidationResult::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "indeterminate" => IssuingCardShippingAddressValidationResult::Indeterminate,
            "likely_deliverable" => IssuingCardShippingAddressValidationResult::LikelyDeliverable,
            "likely_undeliverable" => IssuingCardShippingAddressValidationResult::LikelyUndeliverable,
            other => IssuingCardShippingAddressValidationResult::Unknown(other.to_string()),
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
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
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
impl IssuingCardShippingService {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingCardShippingService::Express => "express",
            IssuingCardShippingService::Priority => "priority",
            IssuingCardShippingService::Standard => "standard",
            IssuingCardShippingService::Unknown(value) => value,
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
impl Default for IssuingCardShippingService {
    fn default() -> Self {
        IssuingCardShippingService::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingCardShippingService {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingCardShippingService::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "express" => IssuingCardShippingService::Express,
            "priority" => IssuingCardShippingService::Priority,
            "standard" => IssuingCardShippingService::Standard,
            other => IssuingCardShippingService::Unknown(other.to_string()),
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
pub enum IssuingCardSpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
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
impl IssuingCardSpendingLimitInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingCardSpendingLimitInterval::AllTime => "all_time",
            IssuingCardSpendingLimitInterval::Daily => "daily",
            IssuingCardSpendingLimitInterval::Monthly => "monthly",
            IssuingCardSpendingLimitInterval::PerAuthorization => "per_authorization",
            IssuingCardSpendingLimitInterval::Weekly => "weekly",
            IssuingCardSpendingLimitInterval::Yearly => "yearly",
            IssuingCardSpendingLimitInterval::Unknown(value) => value,
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
impl Default for IssuingCardSpendingLimitInterval {
    fn default() -> Self {
        IssuingCardSpendingLimitInterval::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingCardSpendingLimitInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingCardSpendingLimitInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "all_time" => IssuingCardSpendingLimitInterval::AllTime,
            "daily" => IssuingCardSpendingLimitInterval::Daily,
            "monthly" => IssuingCardSpendingLimitInterval::Monthly,
            "per_authorization" => IssuingCardSpendingLimitInterval::PerAuthorization,
            "weekly" => IssuingCardSpendingLimitInterval::Weekly,
            "yearly" => IssuingCardSpendingLimitInterval::Yearly,
            other => IssuingCardSpendingLimitInterval::Unknown(other.to_string()),
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
pub struct IssuingNetworkTokenVisa {
    #[doc = "A unique reference ID from Visa to represent the card account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_reference_id: Option<String>,
    #[doc = "The network-unique identifier for the token."]
    #[json(default)]
    pub token_reference_id: String,
    #[doc = "The ID of the entity requesting tokenization, specific to Visa."]
    #[json(default)]
    pub token_requestor_id: String,
    #[doc = "Degree of risk associated with the token between `01` and `99`, with higher number indicating higher risk."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub token_risk_score: Option<String>,
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
pub enum IssuingPersonalizationDesignObject {
    IssuingPersonalizationDesign,
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
impl IssuingPersonalizationDesignObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingPersonalizationDesignObject::IssuingPersonalizationDesign => "issuing.personalization_design",
            IssuingPersonalizationDesignObject::Unknown(value) => value,
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
impl Default for IssuingPersonalizationDesignObject {
    fn default() -> Self {
        IssuingPersonalizationDesignObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingPersonalizationDesignObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingPersonalizationDesignObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.personalization_design" => IssuingPersonalizationDesignObject::IssuingPersonalizationDesign,
            other => IssuingPersonalizationDesignObject::Unknown(other.to_string()),
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
pub enum IssuingTokenObject {
    IssuingToken,
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
impl IssuingTokenObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingTokenObject::IssuingToken => "issuing.token",
            IssuingTokenObject::Unknown(value) => value,
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
impl Default for IssuingTokenObject {
    fn default() -> Self {
        IssuingTokenObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingTokenObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingTokenObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.token" => IssuingTokenObject::IssuingToken,
            other => IssuingTokenObject::Unknown(other.to_string()),
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
pub struct IssuingTransactionFleetFuelPriceData {
    #[doc = "Gross fuel amount that should equal Fuel Volume multiplied by Fuel Unit Cost, inclusive of taxes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gross_amount_decimal: Option<String>,
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
pub struct IssuingTransactionFuelData {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[doc = "The units for `quantity_decimal`."]
    #[json(default)]
    pub unit: String,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    #[json(default)]
    pub unit_cost_decimal: String,
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
pub struct IssuingTransactionLodgingData {
    #[doc = "The time of checking into the lodging."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<i64>,
    #[doc = "The number of nights stayed at the lodging."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
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
pub enum IssuingTransactionObject {
    IssuingTransaction,
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
impl IssuingTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            IssuingTransactionObject::IssuingTransaction => "issuing.transaction",
            IssuingTransactionObject::Unknown(value) => value,
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
impl Default for IssuingTransactionObject {
    fn default() -> Self {
        IssuingTransactionObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for IssuingTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(IssuingTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.transaction" => IssuingTransactionObject::IssuingTransaction,
            other => IssuingTransactionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIssuingCardsParamsType {
    Physical,
    Virtual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl ListIssuingCardsParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIssuingCardsParamsType::Physical => "physical",
            ListIssuingCardsParamsType::Virtual => "virtual",
            ListIssuingCardsParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for ListIssuingCardsParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIssuingCardsParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "physical" => ListIssuingCardsParamsType::Physical,
            "virtual" => ListIssuingCardsParamsType::Virtual,
            other => ListIssuingCardsParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingCardsParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIssuingDisputesParamsStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl ListIssuingDisputesParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIssuingDisputesParamsStatus::Expired => "expired",
            ListIssuingDisputesParamsStatus::Lost => "lost",
            ListIssuingDisputesParamsStatus::Submitted => "submitted",
            ListIssuingDisputesParamsStatus::Unsubmitted => "unsubmitted",
            ListIssuingDisputesParamsStatus::Won => "won",
            ListIssuingDisputesParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for ListIssuingDisputesParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIssuingDisputesParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "expired" => ListIssuingDisputesParamsStatus::Expired,
            "lost" => ListIssuingDisputesParamsStatus::Lost,
            "submitted" => ListIssuingDisputesParamsStatus::Submitted,
            "unsubmitted" => ListIssuingDisputesParamsStatus::Unsubmitted,
            "won" => ListIssuingDisputesParamsStatus::Won,
            other => ListIssuingDisputesParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingDisputesParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIssuingPhysicalBundlesParamsStatus {
    Active,
    Inactive,
    Review,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl ListIssuingPhysicalBundlesParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIssuingPhysicalBundlesParamsStatus::Active => "active",
            ListIssuingPhysicalBundlesParamsStatus::Inactive => "inactive",
            ListIssuingPhysicalBundlesParamsStatus::Review => "review",
            ListIssuingPhysicalBundlesParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for ListIssuingPhysicalBundlesParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIssuingPhysicalBundlesParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ListIssuingPhysicalBundlesParamsStatus::Active,
            "inactive" => ListIssuingPhysicalBundlesParamsStatus::Inactive,
            "review" => ListIssuingPhysicalBundlesParamsStatus::Review,
            other => ListIssuingPhysicalBundlesParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingPhysicalBundlesParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListIssuingTransactionsParamsType {
    Capture,
    Refund,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "issuing")]
impl ListIssuingTransactionsParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListIssuingTransactionsParamsType::Capture => "capture",
            ListIssuingTransactionsParamsType::Refund => "refund",
            ListIssuingTransactionsParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "issuing")]
impl<'json> json::Decode<'json> for ListIssuingTransactionsParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListIssuingTransactionsParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "capture" => ListIssuingTransactionsParamsType::Capture,
            "refund" => ListIssuingTransactionsParamsType::Refund,
            other => ListIssuingTransactionsParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for ListIssuingTransactionsParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "issuing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateIssuingAuthorizationParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "issuing")]
impl crate::params::FormEncode for UpdateIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
