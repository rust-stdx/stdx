#![no_std]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Code used as a fallback when a country is not found.
pub const UNKNOWN_CODE: &str = "XX";

/// Name returned when a country is not found.
pub const UNKNOWN: &str = "Unknown";

/// A country with its ISO 3166-1 alpha-2 code and name.
///
/// # Examples
///
/// ```
/// use countries::Country;
///
/// let country = Country { code: "FR", name: "France" };
/// assert_eq!(country.code, "FR");
/// assert_eq!(country.name, "France");
/// ```
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Country {
    pub code: &'static str,
    pub name: &'static str,
}

/// Returns the default country — [`UNKNOWN_CODE`] / [`UNKNOWN`].
impl Default for Country {
    fn default() -> Self {
        Self {
            code: UNKNOWN_CODE,
            name: UNKNOWN,
        }
    }
}

const _: () = {
    let mut i = 0;
    while i < COUNTRIES_DATA.len() - 1 {
        let a = COUNTRIES_DATA[i].code.as_bytes();
        let b = COUNTRIES_DATA[i + 1].code.as_bytes();
        assert!(a.len() == 2 && b.len() == 2);
        let a_val = (a[0] as u16) << 8 | a[1] as u16;
        let b_val = (b[0] as u16) << 8 | b[1] as u16;
        assert!(a_val < b_val);
        i += 1;
    }
};

/// Returns the full list of known countries, ordered by ISO 3166-1 alpha-2 code.
///
/// # Examples
///
/// ```
/// use countries::Country;
///
/// assert!(countries::countries().iter().any(|c| c.code == "GB"));
/// ```
pub const fn countries() -> &'static [Country] {
    COUNTRIES_DATA
}

/// Look up a country name by its ISO 3166-1 alpha-2 code.
///
/// Returns [`UNKNOWN`] when the code is not recognised or is not exactly 2
/// characters long.
///
/// # Examples
///
/// ```
/// assert_eq!(countries::name("FR"), "France");
/// assert_eq!(countries::name("XX"), "Unknown");
/// assert_eq!(countries::name("??"), "Unknown");
/// ```
pub const fn name(code: &str) -> &'static str {
    // Binary search over `COUNTRIES_DATA`, which is sorted by code and checked
    // at compile time by the `const _: ()` assertion above.  All codes in the
    // table are exactly 2 ASCII bytes, so we can compare them as a single u16
    // value for a cheaper inner-loop comparison.
    let code = code.as_bytes();
    if code.len() != 2 {
        return UNKNOWN;
    }
    let needle = (code[0] as u16) << 8 | code[1] as u16;
    let mut lo = 0usize;
    let mut hi = COUNTRIES_DATA.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let entry = COUNTRIES_DATA[mid].code.as_bytes();
        let mid_val = (entry[0] as u16) << 8 | entry[1] as u16;
        if needle < mid_val {
            hi = mid;
        } else if needle > mid_val {
            lo = mid + 1;
        } else {
            return COUNTRIES_DATA[mid].name;
        }
    }
    UNKNOWN
}

const COUNTRIES_DATA: &[Country] = &[
    Country {
        code: "AD",
        name: "Andorra",
    },
    Country {
        code: "AE",
        name: "United Arab Emirates",
    },
    Country {
        code: "AF",
        name: "Afghanistan",
    },
    Country {
        code: "AG",
        name: "Antigua and Barbuda",
    },
    Country {
        code: "AI",
        name: "Anguilla",
    },
    Country {
        code: "AL",
        name: "Albania",
    },
    Country {
        code: "AM",
        name: "Armenia",
    },
    Country {
        code: "AN",
        name: "Netherlands Antilles",
    },
    Country {
        code: "AO",
        name: "Angola",
    },
    Country {
        code: "AQ",
        name: "Antarctica",
    },
    Country {
        code: "AR",
        name: "Argentina",
    },
    Country {
        code: "AS",
        name: "American Samoa",
    },
    Country {
        code: "AT",
        name: "Austria",
    },
    Country {
        code: "AU",
        name: "Australia",
    },
    Country {
        code: "AW",
        name: "Aruba",
    },
    Country {
        code: "AX",
        name: "Åland Islands",
    },
    Country {
        code: "AZ",
        name: "Azerbaijan",
    },
    Country {
        code: "BA",
        name: "Bosnia and Herzegovina",
    },
    Country {
        code: "BB",
        name: "Barbados",
    },
    Country {
        code: "BD",
        name: "Bangladesh",
    },
    Country {
        code: "BE",
        name: "Belgium",
    },
    Country {
        code: "BF",
        name: "Burkina Faso",
    },
    Country {
        code: "BG",
        name: "Bulgaria",
    },
    Country {
        code: "BH",
        name: "Bahrain",
    },
    Country {
        code: "BI",
        name: "Burundi",
    },
    Country {
        code: "BJ",
        name: "Benin",
    },
    Country {
        code: "BL",
        name: "Saint Barthélemy",
    },
    Country {
        code: "BM",
        name: "Bermuda",
    },
    Country {
        code: "BN",
        name: "Brunei Darussalam",
    },
    Country {
        code: "BO",
        name: "Bolivia",
    },
    Country {
        code: "BQ",
        name: "Bonaire, Sint Eustatius and Saba",
    },
    Country {
        code: "BR",
        name: "Brazil",
    },
    Country {
        code: "BS",
        name: "Bahamas",
    },
    Country {
        code: "BT",
        name: "Bhutan",
    },
    Country {
        code: "BV",
        name: "Bouvet Island",
    },
    Country {
        code: "BW",
        name: "Botswana",
    },
    Country {
        code: "BY",
        name: "Belarus",
    },
    Country {
        code: "BZ",
        name: "Belize",
    },
    Country {
        code: "CA",
        name: "Canada",
    },
    Country {
        code: "CC",
        name: "Cocos (Keeling) Islands",
    },
    Country {
        code: "CD",
        name: "Congo, The Democratic Republic of the",
    },
    Country {
        code: "CF",
        name: "Central African Republic",
    },
    Country {
        code: "CG",
        name: "Congo",
    },
    Country {
        code: "CH",
        name: "Switzerland",
    },
    Country {
        code: "CI",
        name: "Cote D'Ivoire",
    },
    Country {
        code: "CK",
        name: "Cook Islands",
    },
    Country {
        code: "CL",
        name: "Chile",
    },
    Country {
        code: "CM",
        name: "Cameroon",
    },
    Country {
        code: "CN",
        name: "China",
    },
    Country {
        code: "CO",
        name: "Colombia",
    },
    Country {
        code: "CR",
        name: "Costa Rica",
    },
    Country {
        code: "CU",
        name: "Cuba",
    },
    Country {
        code: "CV",
        name: "Cape Verde",
    },
    Country {
        code: "CW",
        name: "Curacao",
    },
    Country {
        code: "CX",
        name: "Christmas Island",
    },
    Country {
        code: "CY",
        name: "Cyprus",
    },
    Country {
        code: "CZ",
        name: "Czech Republic",
    },
    Country {
        code: "DE",
        name: "Germany",
    },
    Country {
        code: "DJ",
        name: "Djibouti",
    },
    Country {
        code: "DK",
        name: "Denmark",
    },
    Country {
        code: "DM",
        name: "Dominica",
    },
    Country {
        code: "DO",
        name: "Dominican Republic",
    },
    Country {
        code: "DZ",
        name: "Algeria",
    },
    Country {
        code: "EC",
        name: "Ecuador",
    },
    Country {
        code: "EE",
        name: "Estonia",
    },
    Country {
        code: "EG",
        name: "Egypt",
    },
    Country {
        code: "EH",
        name: "Western Sahara",
    },
    Country {
        code: "ER",
        name: "Eritrea",
    },
    Country {
        code: "ES",
        name: "Spain",
    },
    Country {
        code: "ET",
        name: "Ethiopia",
    },
    Country {
        code: "FI",
        name: "Finland",
    },
    Country {
        code: "FJ",
        name: "Fiji",
    },
    Country {
        code: "FK",
        name: "Falkland Islands (Malvinas)",
    },
    Country {
        code: "FM",
        name: "Micronesia, Federated States of",
    },
    Country {
        code: "FO",
        name: "Faroe Islands",
    },
    Country {
        code: "FR",
        name: "France",
    },
    Country {
        code: "GA",
        name: "Gabon",
    },
    Country {
        code: "GB",
        name: "United Kingdom",
    },
    Country {
        code: "GD",
        name: "Grenada",
    },
    Country {
        code: "GE",
        name: "Georgia",
    },
    Country {
        code: "GF",
        name: "French Guiana",
    },
    Country {
        code: "GG",
        name: "Guernsey",
    },
    Country {
        code: "GH",
        name: "Ghana",
    },
    Country {
        code: "GI",
        name: "Gibraltar",
    },
    Country {
        code: "GL",
        name: "Greenland",
    },
    Country {
        code: "GM",
        name: "Gambia",
    },
    Country {
        code: "GN",
        name: "Guinea",
    },
    Country {
        code: "GP",
        name: "Guadeloupe",
    },
    Country {
        code: "GQ",
        name: "Equatorial Guinea",
    },
    Country {
        code: "GR",
        name: "Greece",
    },
    Country {
        code: "GS",
        name: "South Georgia and the South Sandwich Islands",
    },
    Country {
        code: "GT",
        name: "Guatemala",
    },
    Country {
        code: "GU",
        name: "Guam",
    },
    Country {
        code: "GW",
        name: "Guinea-Bissau",
    },
    Country {
        code: "GY",
        name: "Guyana",
    },
    Country {
        code: "HK",
        name: "Hong Kong",
    },
    Country {
        code: "HM",
        name: "Heard Island and Mcdonald Islands",
    },
    Country {
        code: "HN",
        name: "Honduras",
    },
    Country {
        code: "HR",
        name: "Croatia",
    },
    Country {
        code: "HT",
        name: "Haiti",
    },
    Country {
        code: "HU",
        name: "Hungary",
    },
    Country {
        code: "ID",
        name: "Indonesia",
    },
    Country {
        code: "IE",
        name: "Ireland",
    },
    Country {
        code: "IL",
        name: "Israel",
    },
    Country {
        code: "IM",
        name: "Isle of Man",
    },
    Country {
        code: "IN",
        name: "India",
    },
    Country {
        code: "IO",
        name: "British Indian Ocean Territory",
    },
    Country {
        code: "IQ",
        name: "Iraq",
    },
    Country {
        code: "IR",
        name: "Iran, Islamic Republic Of",
    },
    Country {
        code: "IS",
        name: "Iceland",
    },
    Country {
        code: "IT",
        name: "Italy",
    },
    Country {
        code: "JE",
        name: "Jersey",
    },
    Country {
        code: "JM",
        name: "Jamaica",
    },
    Country {
        code: "JO",
        name: "Jordan",
    },
    Country {
        code: "JP",
        name: "Japan",
    },
    Country {
        code: "KE",
        name: "Kenya",
    },
    Country {
        code: "KG",
        name: "Kyrgyzstan",
    },
    Country {
        code: "KH",
        name: "Cambodia",
    },
    Country {
        code: "KI",
        name: "Kiribati",
    },
    Country {
        code: "KM",
        name: "Comoros",
    },
    Country {
        code: "KN",
        name: "Saint Kitts and Nevis",
    },
    Country {
        code: "KP",
        name: "Korea, Democratic People's Republic of",
    },
    Country {
        code: "KR",
        name: "Korea, Republic of",
    },
    Country {
        code: "KW",
        name: "Kuwait",
    },
    Country {
        code: "KY",
        name: "Cayman Islands",
    },
    Country {
        code: "KZ",
        name: "Kazakhstan",
    },
    Country {
        code: "LA",
        name: "Lao People's Democratic Republic",
    },
    Country {
        code: "LB",
        name: "Lebanon",
    },
    Country {
        code: "LC",
        name: "Saint Lucia",
    },
    Country {
        code: "LI",
        name: "Liechtenstein",
    },
    Country {
        code: "LK",
        name: "Sri Lanka",
    },
    Country {
        code: "LR",
        name: "Liberia",
    },
    Country {
        code: "LS",
        name: "Lesotho",
    },
    Country {
        code: "LT",
        name: "Lithuania",
    },
    Country {
        code: "LU",
        name: "Luxembourg",
    },
    Country {
        code: "LV",
        name: "Latvia",
    },
    Country {
        code: "LY",
        name: "Libyan Arab Jamahiriya",
    },
    Country {
        code: "MA",
        name: "Morocco",
    },
    Country {
        code: "MC",
        name: "Monaco",
    },
    Country {
        code: "MD",
        name: "Moldova, Republic of",
    },
    Country {
        code: "ME",
        name: "Montenegro",
    },
    Country {
        code: "MF",
        name: "Saint Martin",
    },
    Country {
        code: "MG",
        name: "Madagascar",
    },
    Country {
        code: "MH",
        name: "Marshall Islands",
    },
    Country {
        code: "MK",
        name: "North Macedonia",
    },
    Country {
        code: "ML",
        name: "Mali",
    },
    Country {
        code: "MM",
        name: "Myanmar",
    },
    Country {
        code: "MN",
        name: "Mongolia",
    },
    Country {
        code: "MO",
        name: "Macao",
    },
    Country {
        code: "MP",
        name: "Northern Mariana Islands",
    },
    Country {
        code: "MQ",
        name: "Martinique",
    },
    Country {
        code: "MR",
        name: "Mauritania",
    },
    Country {
        code: "MS",
        name: "Montserrat",
    },
    Country {
        code: "MT",
        name: "Malta",
    },
    Country {
        code: "MU",
        name: "Mauritius",
    },
    Country {
        code: "MV",
        name: "Maldives",
    },
    Country {
        code: "MW",
        name: "Malawi",
    },
    Country {
        code: "MX",
        name: "Mexico",
    },
    Country {
        code: "MY",
        name: "Malaysia",
    },
    Country {
        code: "MZ",
        name: "Mozambique",
    },
    Country {
        code: "NA",
        name: "Namibia",
    },
    Country {
        code: "NC",
        name: "New Caledonia",
    },
    Country {
        code: "NE",
        name: "Niger",
    },
    Country {
        code: "NF",
        name: "Norfolk Island",
    },
    Country {
        code: "NG",
        name: "Nigeria",
    },
    Country {
        code: "NI",
        name: "Nicaragua",
    },
    Country {
        code: "NL",
        name: "Netherlands",
    },
    Country {
        code: "NO",
        name: "Norway",
    },
    Country {
        code: "NP",
        name: "Nepal",
    },
    Country {
        code: "NR",
        name: "Nauru",
    },
    Country {
        code: "NU",
        name: "Niue",
    },
    Country {
        code: "NZ",
        name: "New Zealand",
    },
    Country {
        code: "OM",
        name: "Oman",
    },
    Country {
        code: "PA",
        name: "Panama",
    },
    Country {
        code: "PE",
        name: "Peru",
    },
    Country {
        code: "PF",
        name: "French Polynesia",
    },
    Country {
        code: "PG",
        name: "Papua New Guinea",
    },
    Country {
        code: "PH",
        name: "Philippines",
    },
    Country {
        code: "PK",
        name: "Pakistan",
    },
    Country {
        code: "PL",
        name: "Poland",
    },
    Country {
        code: "PM",
        name: "Saint Pierre and Miquelon",
    },
    Country {
        code: "PN",
        name: "Pitcairn",
    },
    Country {
        code: "PR",
        name: "Puerto Rico",
    },
    Country {
        code: "PS",
        name: "Palestinian Territory, Occupied",
    },
    Country {
        code: "PT",
        name: "Portugal",
    },
    Country {
        code: "PW",
        name: "Palau",
    },
    Country {
        code: "PY",
        name: "Paraguay",
    },
    Country {
        code: "QA",
        name: "Qatar",
    },
    Country {
        code: "RE",
        name: "Reunion",
    },
    Country {
        code: "RO",
        name: "Romania",
    },
    Country {
        code: "RS",
        name: "Serbia",
    },
    Country {
        code: "RU",
        name: "Russian Federation",
    },
    Country {
        code: "RW",
        name: "Rwanda",
    },
    Country {
        code: "SA",
        name: "Saudi Arabia",
    },
    Country {
        code: "SB",
        name: "Solomon Islands",
    },
    Country {
        code: "SC",
        name: "Seychelles",
    },
    Country {
        code: "SD",
        name: "Sudan",
    },
    Country {
        code: "SE",
        name: "Sweden",
    },
    Country {
        code: "SG",
        name: "Singapore",
    },
    Country {
        code: "SH",
        name: "Saint Helena",
    },
    Country {
        code: "SI",
        name: "Slovenia",
    },
    Country {
        code: "SJ",
        name: "Svalbard and Jan Mayen",
    },
    Country {
        code: "SK",
        name: "Slovakia",
    },
    Country {
        code: "SL",
        name: "Sierra Leone",
    },
    Country {
        code: "SM",
        name: "San Marino",
    },
    Country {
        code: "SN",
        name: "Senegal",
    },
    Country {
        code: "SO",
        name: "Somalia",
    },
    Country {
        code: "SR",
        name: "Suriname",
    },
    Country {
        code: "SS",
        name: "South Sudan",
    },
    Country {
        code: "ST",
        name: "Sao Tome and Principe",
    },
    Country {
        code: "SV",
        name: "El Salvador",
    },
    Country {
        code: "SX",
        name: "Sint Maarten (Dutch part)",
    },
    Country {
        code: "SY",
        name: "Syrian Arab Republic",
    },
    Country {
        code: "SZ",
        name: "Eswatini",
    },
    Country {
        code: "TC",
        name: "Turks and Caicos Islands",
    },
    Country {
        code: "TD",
        name: "Chad",
    },
    Country {
        code: "TF",
        name: "French Southern Territories",
    },
    Country {
        code: "TG",
        name: "Togo",
    },
    Country {
        code: "TH",
        name: "Thailand",
    },
    Country {
        code: "TJ",
        name: "Tajikistan",
    },
    Country {
        code: "TK",
        name: "Tokelau",
    },
    Country {
        code: "TL",
        name: "Timor-Leste",
    },
    Country {
        code: "TM",
        name: "Turkmenistan",
    },
    Country {
        code: "TN",
        name: "Tunisia",
    },
    Country {
        code: "TO",
        name: "Tonga",
    },
    Country {
        code: "TR",
        name: "Turkey",
    },
    Country {
        code: "TT",
        name: "Trinidad and Tobago",
    },
    Country {
        code: "TV",
        name: "Tuvalu",
    },
    Country {
        code: "TW",
        name: "Taiwan",
    },
    Country {
        code: "TZ",
        name: "Tanzania, United Republic of",
    },
    Country {
        code: "UA",
        name: "Ukraine",
    },
    Country {
        code: "UG",
        name: "Uganda",
    },
    Country {
        code: "UM",
        name: "United States Minor Outlying Islands",
    },
    Country {
        code: "US",
        name: "United States",
    },
    Country {
        code: "UY",
        name: "Uruguay",
    },
    Country {
        code: "UZ",
        name: "Uzbekistan",
    },
    Country {
        code: "VA",
        name: "Holy See (Vatican City State)",
    },
    Country {
        code: "VC",
        name: "Saint Vincent and the Grenadines",
    },
    Country {
        code: "VE",
        name: "Venezuela",
    },
    Country {
        code: "VG",
        name: "Virgin Islands, British",
    },
    Country {
        code: "VI",
        name: "Virgin Islands, U.S.",
    },
    Country {
        code: "VN",
        name: "Viet Nam",
    },
    Country {
        code: "VU",
        name: "Vanuatu",
    },
    Country {
        code: "WF",
        name: "Wallis and Futuna",
    },
    Country {
        code: "WS",
        name: "Samoa",
    },
    Country {
        code: "XK",
        name: "Kosovo",
    },
    Country {
        code: "YE",
        name: "Yemen",
    },
    Country {
        code: "YT",
        name: "Mayotte",
    },
    Country {
        code: "ZA",
        name: "South Africa",
    },
    Country {
        code: "ZM",
        name: "Zambia",
    },
    Country {
        code: "ZW",
        name: "Zimbabwe",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_country() {
        assert_eq!(name("FR"), "France");
        assert_eq!(name("GB"), "United Kingdom");
        assert_eq!(name("US"), "United States");
        assert_eq!(name("DE"), "Germany");
    }

    #[test]
    fn unknown_country() {
        assert_eq!(name("ZZ"), "Unknown");
        assert_eq!(name("??"), "Unknown");
    }

    #[test]
    fn wrong_length() {
        assert_eq!(name(""), "Unknown");
        assert_eq!(name("F"), "Unknown");
        assert_eq!(name("FRA"), "Unknown");
    }

    #[test]
    fn default_country() {
        let c = Country::default();
        assert_eq!(c.code, "XX");
        assert_eq!(c.name, "Unknown");
    }

    #[test]
    fn countries_slice_is_sorted() {
        for win in countries().windows(2) {
            assert!(win[0].code < win[1].code, "out of order: {} >= {}", win[0].code, win[1].code);
        }
    }

    #[test]
    fn correct_entry_count() {
        assert_eq!(countries().len(), 251);
    }

    #[test]
    fn round_trip_all() {
        for c in countries() {
            assert_eq!(name(c.code), c.name, "failed for {}", c.code);
        }
    }
}
