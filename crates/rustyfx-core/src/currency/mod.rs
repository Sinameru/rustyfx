use phf::phf_map;

pub struct CurrencyInfo {
    pub code: &'static str,
    pub numeric: u16,
    pub minor_units: u8,
    pub name: &'static str,
    pub entity: &'static str,
}

pub static CURRENCIES: phf::Map<&'static str, &'static [CurrencyInfo]> = phf_map! {
    "MVR" => &[CurrencyInfo { code: r#"MVR"#, numeric: 462, minor_units: 2, name: r#"Rufiyaa"#, entity: r#"MALDIVES"# }],
    "BDT" => &[CurrencyInfo { code: r#"BDT"#, numeric: 50, minor_units: 2, name: r#"Taka"#, entity: r#"BANGLADESH"# }],
    "MWK" => &[CurrencyInfo { code: r#"MWK"#, numeric: 454, minor_units: 2, name: r#"Malawi Kwacha"#, entity: r#"MALAWI"# }],
    "LKR" => &[CurrencyInfo { code: r#"LKR"#, numeric: 144, minor_units: 2, name: r#"Sri Lanka Rupee"#, entity: r#"SRI LANKA"# }],
    "CLF" => &[CurrencyInfo { code: r#"CLF"#, numeric: 990, minor_units: 4, name: r#"Unidad de Fomento"#, entity: r#"CHILE"# }],
    "USN" => &[CurrencyInfo { code: r#"USN"#, numeric: 997, minor_units: 2, name: r#"US Dollar (Next day)"#, entity: r#"UNITED STATES OF AMERICA (THE)"# }],
    "COU" => &[CurrencyInfo { code: r#"COU"#, numeric: 970, minor_units: 2, name: r#"Unidad de Valor Real"#, entity: r#"COLOMBIA"# }],
    "XAF" => &[CurrencyInfo { code: r#"XAF"#, numeric: 950, minor_units: 0, name: r#"CFA Franc BEAC"#, entity: r#"CAMEROON"# }
             , CurrencyInfo { code: r#"XAF"#, numeric: 950, minor_units: 0, name: r#"CFA Franc BEAC"#, entity: r#"CENTRAL AFRICAN REPUBLIC (THE)"# }
             , CurrencyInfo { code: r#"XAF"#, numeric: 950, minor_units: 0, name: r#"CFA Franc BEAC"#, entity: r#"CHAD"# }
             , CurrencyInfo { code: r#"XAF"#, numeric: 950, minor_units: 0, name: r#"CFA Franc BEAC"#, entity: r#"CONGO (THE)"# }
             , CurrencyInfo { code: r#"XAF"#, numeric: 950, minor_units: 0, name: r#"CFA Franc BEAC"#, entity: r#"EQUATORIAL GUINEA"# }
             , CurrencyInfo { code: r#"XAF"#, numeric: 950, minor_units: 0, name: r#"CFA Franc BEAC"#, entity: r#"GABON"# }],
    "MDL" => &[CurrencyInfo { code: r#"MDL"#, numeric: 498, minor_units: 2, name: r#"Moldovan Leu"#, entity: r#"MOLDOVA (THE REPUBLIC OF)"# }],
    "RUB" => &[CurrencyInfo { code: r#"RUB"#, numeric: 643, minor_units: 2, name: r#"Russian Ruble"#, entity: r#"RUSSIAN FEDERATION (THE)"# }],
    "ERN" => &[CurrencyInfo { code: r#"ERN"#, numeric: 232, minor_units: 2, name: r#"Nakfa"#, entity: r#"ERITREA"# }],
    "GYD" => &[CurrencyInfo { code: r#"GYD"#, numeric: 328, minor_units: 2, name: r#"Guyana Dollar"#, entity: r#"GUYANA"# }],
    "NAD" => &[CurrencyInfo { code: r#"NAD"#, numeric: 516, minor_units: 2, name: r#"Namibia Dollar"#, entity: r#"NAMIBIA"# }],
    "OMR" => &[CurrencyInfo { code: r#"OMR"#, numeric: 512, minor_units: 3, name: r#"Rial Omani"#, entity: r#"OMAN"# }],
    "SEK" => &[CurrencyInfo { code: r#"SEK"#, numeric: 752, minor_units: 2, name: r#"Swedish Krona"#, entity: r#"SWEDEN"# }],
    "AED" => &[CurrencyInfo { code: r#"AED"#, numeric: 784, minor_units: 2, name: r#"UAE Dirham"#, entity: r#"UNITED ARAB EMIRATES (THE)"# }],
    "VED" => &[CurrencyInfo { code: r#"VED"#, numeric: 926, minor_units: 2, name: r#"Bolívar Soberano"#, entity: r#"VENEZUELA (BOLIVARIAN REPUBLIC OF)"# }],
    "YER" => &[CurrencyInfo { code: r#"YER"#, numeric: 886, minor_units: 2, name: r#"Yemeni Rial"#, entity: r#"YEMEN"# }],
    "EGP" => &[CurrencyInfo { code: r#"EGP"#, numeric: 818, minor_units: 2, name: r#"Egyptian Pound"#, entity: r#"EGYPT"# }],
    "XAU" => &[CurrencyInfo { code: r#"XAU"#, numeric: 959, minor_units: 0, name: r#"Gold"#, entity: r#"ZZ08_Gold"# }],
    "CHE" => &[CurrencyInfo { code: r#"CHE"#, numeric: 947, minor_units: 2, name: r#"WIR Euro"#, entity: r#"SWITZERLAND"# }],
    "IQD" => &[CurrencyInfo { code: r#"IQD"#, numeric: 368, minor_units: 3, name: r#"Iraqi Dinar"#, entity: r#"IRAQ"# }],
    "UZS" => &[CurrencyInfo { code: r#"UZS"#, numeric: 860, minor_units: 2, name: r#"Uzbekistan Sum"#, entity: r#"UZBEKISTAN"# }],
    "JPY" => &[CurrencyInfo { code: r#"JPY"#, numeric: 392, minor_units: 0, name: r#"Yen"#, entity: r#"JAPAN"# }],
    "BGN" => &[CurrencyInfo { code: r#"BGN"#, numeric: 975, minor_units: 2, name: r#"Bulgarian Lev"#, entity: r#"BULGARIA"# }],
    "EUR" => &[CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"ÅLAND ISLANDS"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"ANDORRA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"AUSTRIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"BELGIUM"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"CROATIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"CYPRUS"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"ESTONIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"EUROPEAN UNION"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"FINLAND"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"FRANCE"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"FRENCH GUIANA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"FRENCH SOUTHERN TERRITORIES (THE)"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"GERMANY"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"GREECE"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"GUADELOUPE"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"HOLY SEE (THE)"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"IRELAND"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"ITALY"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"LATVIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"LITHUANIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"LUXEMBOURG"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"MALTA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"MARTINIQUE"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"MAYOTTE"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"MONACO"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"MONTENEGRO"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"NETHERLANDS (THE)"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"PORTUGAL"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"RÉUNION"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SAINT BARTHÉLEMY"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SAINT MARTIN (FRENCH PART)"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SAINT PIERRE AND MIQUELON"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SAN MARINO"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SLOVAKIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SLOVENIA"# }
             , CurrencyInfo { code: r#"EUR"#, numeric: 978, minor_units: 2, name: r#"Euro"#, entity: r#"SPAIN"# }],
    "CDF" => &[CurrencyInfo { code: r#"CDF"#, numeric: 976, minor_units: 2, name: r#"Congolese Franc"#, entity: r#"CONGO (THE DEMOCRATIC REPUBLIC OF THE)"# }],
    "SSP" => &[CurrencyInfo { code: r#"SSP"#, numeric: 728, minor_units: 2, name: r#"South Sudanese Pound"#, entity: r#"SOUTH SUDAN"# }],
    "PGK" => &[CurrencyInfo { code: r#"PGK"#, numeric: 598, minor_units: 2, name: r#"Kina"#, entity: r#"PAPUA NEW GUINEA"# }],
    "CHF" => &[CurrencyInfo { code: r#"CHF"#, numeric: 756, minor_units: 2, name: r#"Swiss Franc"#, entity: r#"LIECHTENSTEIN"# }
             , CurrencyInfo { code: r#"CHF"#, numeric: 756, minor_units: 2, name: r#"Swiss Franc"#, entity: r#"SWITZERLAND"# }],
    "BIF" => &[CurrencyInfo { code: r#"BIF"#, numeric: 108, minor_units: 0, name: r#"Burundi Franc"#, entity: r#"BURUNDI"# }],
    "TWD" => &[CurrencyInfo { code: r#"TWD"#, numeric: 901, minor_units: 2, name: r#"New Taiwan Dollar"#, entity: r#"TAIWAN (PROVINCE OF CHINA)"# }],
    "VES" => &[CurrencyInfo { code: r#"VES"#, numeric: 928, minor_units: 2, name: r#"Bolívar Soberano"#, entity: r#"VENEZUELA (BOLIVARIAN REPUBLIC OF)"# }],
    "QAR" => &[CurrencyInfo { code: r#"QAR"#, numeric: 634, minor_units: 2, name: r#"Qatari Rial"#, entity: r#"QATAR"# }],
    "SGD" => &[CurrencyInfo { code: r#"SGD"#, numeric: 702, minor_units: 2, name: r#"Singapore Dollar"#, entity: r#"SINGAPORE"# }],
    "MUR" => &[CurrencyInfo { code: r#"MUR"#, numeric: 480, minor_units: 2, name: r#"Mauritius Rupee"#, entity: r#"MAURITIUS"# }],
    "CUP" => &[CurrencyInfo { code: r#"CUP"#, numeric: 192, minor_units: 2, name: r#"Cuban Peso"#, entity: r#"CUBA"# }],
    "DJF" => &[CurrencyInfo { code: r#"DJF"#, numeric: 262, minor_units: 0, name: r#"Djibouti Franc"#, entity: r#"DJIBOUTI"# }],
    "KRW" => &[CurrencyInfo { code: r#"KRW"#, numeric: 410, minor_units: 0, name: r#"Won"#, entity: r#"KOREA (THE REPUBLIC OF)"# }],
    "AUD" => &[CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"AUSTRALIA"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"CHRISTMAS ISLAND"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"COCOS (KEELING) ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"HEARD ISLAND AND McDONALD ISLANDS"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"KIRIBATI"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"NAURU"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"NORFOLK ISLAND"# }
             , CurrencyInfo { code: r#"AUD"#, numeric: 36, minor_units: 2, name: r#"Australian Dollar"#, entity: r#"TUVALU"# }],
    "XDR" => &[CurrencyInfo { code: r#"XDR"#, numeric: 960, minor_units: 0, name: r#"SDR (Special Drawing Right)"#, entity: r#"INTERNATIONAL MONETARY FUND (IMF)"# }],
    "FJD" => &[CurrencyInfo { code: r#"FJD"#, numeric: 242, minor_units: 2, name: r#"Fiji Dollar"#, entity: r#"FIJI"# }],
    "GIP" => &[CurrencyInfo { code: r#"GIP"#, numeric: 292, minor_units: 2, name: r#"Gibraltar Pound"#, entity: r#"GIBRALTAR"# }],
    "PAB" => &[CurrencyInfo { code: r#"PAB"#, numeric: 590, minor_units: 2, name: r#"Balboa"#, entity: r#"PANAMA"# }],
    "MXV" => &[CurrencyInfo { code: r#"MXV"#, numeric: 979, minor_units: 2, name: r#"Mexican Unidad de Inversion (UDI)"#, entity: r#"MEXICO"# }],
    "XPT" => &[CurrencyInfo { code: r#"XPT"#, numeric: 962, minor_units: 0, name: r#"Platinum"#, entity: r#"ZZ10_Platinum"# }],
    "SDG" => &[CurrencyInfo { code: r#"SDG"#, numeric: 938, minor_units: 2, name: r#"Sudanese Pound"#, entity: r#"SUDAN (THE)"# }],
    "XSU" => &[CurrencyInfo { code: r#"XSU"#, numeric: 994, minor_units: 0, name: r#"Sucre"#, entity: r#"SISTEMA UNITARIO DE COMPENSACION REGIONAL DE PAGOS "SUCRE""# }],
    "XOF" => &[CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"BENIN"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"BURKINA FASO"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"CÔTE D'IVOIRE"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"GUINEA-BISSAU"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"MALI"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"NIGER (THE)"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"SENEGAL"# }
             , CurrencyInfo { code: r#"XOF"#, numeric: 952, minor_units: 0, name: r#"CFA Franc BCEAO"#, entity: r#"TOGO"# }],
    "LRD" => &[CurrencyInfo { code: r#"LRD"#, numeric: 430, minor_units: 2, name: r#"Liberian Dollar"#, entity: r#"LIBERIA"# }],
    "XPF" => &[CurrencyInfo { code: r#"XPF"#, numeric: 953, minor_units: 0, name: r#"CFP Franc"#, entity: r#"FRENCH POLYNESIA"# }
             , CurrencyInfo { code: r#"XPF"#, numeric: 953, minor_units: 0, name: r#"CFP Franc"#, entity: r#"NEW CALEDONIA"# }
             , CurrencyInfo { code: r#"XPF"#, numeric: 953, minor_units: 0, name: r#"CFP Franc"#, entity: r#"WALLIS AND FUTUNA"# }],
    "RSD" => &[CurrencyInfo { code: r#"RSD"#, numeric: 941, minor_units: 2, name: r#"Serbian Dinar"#, entity: r#"SERBIA"# }],
    "KPW" => &[CurrencyInfo { code: r#"KPW"#, numeric: 408, minor_units: 2, name: r#"North Korean Won"#, entity: r#"KOREA (THE DEMOCRATIC PEOPLE’S REPUBLIC OF)"# }],
    "MAD" => &[CurrencyInfo { code: r#"MAD"#, numeric: 504, minor_units: 2, name: r#"Moroccan Dirham"#, entity: r#"MOROCCO"# }
             , CurrencyInfo { code: r#"MAD"#, numeric: 504, minor_units: 2, name: r#"Moroccan Dirham"#, entity: r#"WESTERN SAHARA"# }],
    "TOP" => &[CurrencyInfo { code: r#"TOP"#, numeric: 776, minor_units: 2, name: r#"Pa’anga"#, entity: r#"TONGA"# }],
    "BBD" => &[CurrencyInfo { code: r#"BBD"#, numeric: 52, minor_units: 2, name: r#"Barbados Dollar"#, entity: r#"BARBADOS"# }],
    "MOP" => &[CurrencyInfo { code: r#"MOP"#, numeric: 446, minor_units: 2, name: r#"Pataca"#, entity: r#"MACAO"# }],
    "CAD" => &[CurrencyInfo { code: r#"CAD"#, numeric: 124, minor_units: 2, name: r#"Canadian Dollar"#, entity: r#"CANADA"# }],
    "ALL" => &[CurrencyInfo { code: r#"ALL"#, numeric: 8, minor_units: 2, name: r#"Lek"#, entity: r#"ALBANIA"# }],
    "RWF" => &[CurrencyInfo { code: r#"RWF"#, numeric: 646, minor_units: 0, name: r#"Rwanda Franc"#, entity: r#"RWANDA"# }],
    "MMK" => &[CurrencyInfo { code: r#"MMK"#, numeric: 104, minor_units: 2, name: r#"Kyat"#, entity: r#"MYANMAR"# }],
    "IRR" => &[CurrencyInfo { code: r#"IRR"#, numeric: 364, minor_units: 2, name: r#"Iranian Rial"#, entity: r#"IRAN (ISLAMIC REPUBLIC OF)"# }],
    "KYD" => &[CurrencyInfo { code: r#"KYD"#, numeric: 136, minor_units: 2, name: r#"Cayman Islands Dollar"#, entity: r#"CAYMAN ISLANDS (THE)"# }],
    "TMT" => &[CurrencyInfo { code: r#"TMT"#, numeric: 934, minor_units: 2, name: r#"Turkmenistan New Manat"#, entity: r#"TURKMENISTAN"# }],
    "UYU" => &[CurrencyInfo { code: r#"UYU"#, numeric: 858, minor_units: 2, name: r#"Peso Uruguayo"#, entity: r#"URUGUAY"# }],
    "GHS" => &[CurrencyInfo { code: r#"GHS"#, numeric: 936, minor_units: 2, name: r#"Ghana Cedi"#, entity: r#"GHANA"# }],
    "CLP" => &[CurrencyInfo { code: r#"CLP"#, numeric: 152, minor_units: 0, name: r#"Chilean Peso"#, entity: r#"CHILE"# }],
    "HUF" => &[CurrencyInfo { code: r#"HUF"#, numeric: 348, minor_units: 2, name: r#"Forint"#, entity: r#"HUNGARY"# }],
    "BHD" => &[CurrencyInfo { code: r#"BHD"#, numeric: 48, minor_units: 3, name: r#"Bahraini Dinar"#, entity: r#"BAHRAIN"# }],
    "AOA" => &[CurrencyInfo { code: r#"AOA"#, numeric: 973, minor_units: 2, name: r#"Kwanza"#, entity: r#"ANGOLA"# }],
    "TZS" => &[CurrencyInfo { code: r#"TZS"#, numeric: 834, minor_units: 2, name: r#"Tanzanian Shilling"#, entity: r#"TANZANIA, UNITED REPUBLIC OF"# }],
    "BOB" => &[CurrencyInfo { code: r#"BOB"#, numeric: 68, minor_units: 2, name: r#"Boliviano"#, entity: r#"BOLIVIA (PLURINATIONAL STATE OF)"# }],
    "SOS" => &[CurrencyInfo { code: r#"SOS"#, numeric: 706, minor_units: 2, name: r#"Somali Shilling"#, entity: r#"SOMALIA"# }],
    "PYG" => &[CurrencyInfo { code: r#"PYG"#, numeric: 600, minor_units: 0, name: r#"Guarani"#, entity: r#"PARAGUAY"# }],
    "JMD" => &[CurrencyInfo { code: r#"JMD"#, numeric: 388, minor_units: 2, name: r#"Jamaican Dollar"#, entity: r#"JAMAICA"# }],
    "MYR" => &[CurrencyInfo { code: r#"MYR"#, numeric: 458, minor_units: 2, name: r#"Malaysian Ringgit"#, entity: r#"MALAYSIA"# }],
    "ILS" => &[CurrencyInfo { code: r#"ILS"#, numeric: 376, minor_units: 2, name: r#"New Israeli Sheqel"#, entity: r#"ISRAEL"# }],
    "ZAR" => &[CurrencyInfo { code: r#"ZAR"#, numeric: 710, minor_units: 2, name: r#"Rand"#, entity: r#"LESOTHO"# }
             , CurrencyInfo { code: r#"ZAR"#, numeric: 710, minor_units: 2, name: r#"Rand"#, entity: r#"NAMIBIA"# }
             , CurrencyInfo { code: r#"ZAR"#, numeric: 710, minor_units: 2, name: r#"Rand"#, entity: r#"SOUTH AFRICA"# }],
    "LBP" => &[CurrencyInfo { code: r#"LBP"#, numeric: 422, minor_units: 2, name: r#"Lebanese Pound"#, entity: r#"LEBANON"# }],
    "XUA" => &[CurrencyInfo { code: r#"XUA"#, numeric: 965, minor_units: 0, name: r#"ADB Unit of Account"#, entity: r#"MEMBER COUNTRIES OF THE AFRICAN DEVELOPMENT BANK GROUP"# }],
    "TTD" => &[CurrencyInfo { code: r#"TTD"#, numeric: 780, minor_units: 2, name: r#"Trinidad and Tobago Dollar"#, entity: r#"TRINIDAD AND TOBAGO"# }],
    "XCG" => &[CurrencyInfo { code: r#"XCG"#, numeric: 532, minor_units: 2, name: r#"Caribbean Guilder"#, entity: r#"CURAÇAO"# }
             , CurrencyInfo { code: r#"XCG"#, numeric: 532, minor_units: 2, name: r#"Caribbean Guilder"#, entity: r#"SINT MAARTEN (DUTCH PART)"# }],
    "AMD" => &[CurrencyInfo { code: r#"AMD"#, numeric: 51, minor_units: 2, name: r#"Armenian Dram"#, entity: r#"ARMENIA"# }],
    "BSD" => &[CurrencyInfo { code: r#"BSD"#, numeric: 44, minor_units: 2, name: r#"Bahamian Dollar"#, entity: r#"BAHAMAS (THE)"# }],
    "THB" => &[CurrencyInfo { code: r#"THB"#, numeric: 764, minor_units: 2, name: r#"Baht"#, entity: r#"THAILAND"# }],
    "AWG" => &[CurrencyInfo { code: r#"AWG"#, numeric: 533, minor_units: 2, name: r#"Aruban Florin"#, entity: r#"ARUBA"# }],
    "HTG" => &[CurrencyInfo { code: r#"HTG"#, numeric: 332, minor_units: 2, name: r#"Gourde"#, entity: r#"HAITI"# }],
    "SBD" => &[CurrencyInfo { code: r#"SBD"#, numeric: 90, minor_units: 2, name: r#"Solomon Islands Dollar"#, entity: r#"SOLOMON ISLANDS"# }],
    "SRD" => &[CurrencyInfo { code: r#"SRD"#, numeric: 968, minor_units: 2, name: r#"Surinam Dollar"#, entity: r#"SURINAME"# }],
    "XAG" => &[CurrencyInfo { code: r#"XAG"#, numeric: 961, minor_units: 0, name: r#"Silver"#, entity: r#"ZZ11_Silver"# }],
    "XXX" => &[CurrencyInfo { code: r#"XXX"#, numeric: 999, minor_units: 0, name: r#"The codes assigned for transactions where no currency is involved"#, entity: r#"ZZ07_No_Currency"# }],
    "BTN" => &[CurrencyInfo { code: r#"BTN"#, numeric: 64, minor_units: 2, name: r#"Ngultrum"#, entity: r#"BHUTAN"# }],
    "KHR" => &[CurrencyInfo { code: r#"KHR"#, numeric: 116, minor_units: 2, name: r#"Riel"#, entity: r#"CAMBODIA"# }],
    "UGX" => &[CurrencyInfo { code: r#"UGX"#, numeric: 800, minor_units: 0, name: r#"Uganda Shilling"#, entity: r#"UGANDA"# }],
    "BAM" => &[CurrencyInfo { code: r#"BAM"#, numeric: 977, minor_units: 2, name: r#"Convertible Mark"#, entity: r#"BOSNIA AND HERZEGOVINA"# }],
    "PHP" => &[CurrencyInfo { code: r#"PHP"#, numeric: 608, minor_units: 2, name: r#"Philippine Peso"#, entity: r#"PHILIPPINES (THE)"# }],
    "GTQ" => &[CurrencyInfo { code: r#"GTQ"#, numeric: 320, minor_units: 2, name: r#"Quetzal"#, entity: r#"GUATEMALA"# }],
    "BMD" => &[CurrencyInfo { code: r#"BMD"#, numeric: 60, minor_units: 2, name: r#"Bermudian Dollar"#, entity: r#"BERMUDA"# }],
    "JOD" => &[CurrencyInfo { code: r#"JOD"#, numeric: 400, minor_units: 3, name: r#"Jordanian Dinar"#, entity: r#"JORDAN"# }],
    "XPD" => &[CurrencyInfo { code: r#"XPD"#, numeric: 964, minor_units: 0, name: r#"Palladium"#, entity: r#"ZZ09_Palladium"# }],
    "UAH" => &[CurrencyInfo { code: r#"UAH"#, numeric: 980, minor_units: 2, name: r#"Hryvnia"#, entity: r#"UKRAINE"# }],
    "CHW" => &[CurrencyInfo { code: r#"CHW"#, numeric: 948, minor_units: 2, name: r#"WIR Franc"#, entity: r#"SWITZERLAND"# }],
    "AZN" => &[CurrencyInfo { code: r#"AZN"#, numeric: 944, minor_units: 2, name: r#"Azerbaijan Manat"#, entity: r#"AZERBAIJAN"# }],
    "SCR" => &[CurrencyInfo { code: r#"SCR"#, numeric: 690, minor_units: 2, name: r#"Seychelles Rupee"#, entity: r#"SEYCHELLES"# }],
    "IDR" => &[CurrencyInfo { code: r#"IDR"#, numeric: 360, minor_units: 2, name: r#"Rupiah"#, entity: r#"INDONESIA"# }],
    "XAD" => &[CurrencyInfo { code: r#"XAD"#, numeric: 396, minor_units: 2, name: r#"Arab Accounting Dinar"#, entity: r#"ARAB MONETARY FUND"# }],
    "PLN" => &[CurrencyInfo { code: r#"PLN"#, numeric: 985, minor_units: 2, name: r#"Zloty"#, entity: r#"POLAND"# }],
    "TRY" => &[CurrencyInfo { code: r#"TRY"#, numeric: 949, minor_units: 2, name: r#"Turkish Lira"#, entity: r#"TÜRKİYE"# }],
    "XBD" => &[CurrencyInfo { code: r#"XBD"#, numeric: 958, minor_units: 0, name: r#"Bond Markets Unit European Unit of Account 17 (E.U.A.-17)"#, entity: r#"ZZ04_Bond Markets Unit European_EUA-17"# }],
    "DKK" => &[CurrencyInfo { code: r#"DKK"#, numeric: 208, minor_units: 2, name: r#"Danish Krone"#, entity: r#"DENMARK"# }
             , CurrencyInfo { code: r#"DKK"#, numeric: 208, minor_units: 2, name: r#"Danish Krone"#, entity: r#"FAROE ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"DKK"#, numeric: 208, minor_units: 2, name: r#"Danish Krone"#, entity: r#"GREENLAND"# }],
    "HNL" => &[CurrencyInfo { code: r#"HNL"#, numeric: 340, minor_units: 2, name: r#"Lempira"#, entity: r#"HONDURAS"# }],
    "GEL" => &[CurrencyInfo { code: r#"GEL"#, numeric: 981, minor_units: 2, name: r#"Lari"#, entity: r#"GEORGIA"# }],
    "SZL" => &[CurrencyInfo { code: r#"SZL"#, numeric: 748, minor_units: 2, name: r#"Lilangeni"#, entity: r#"ESWATINI"# }],
    "DOP" => &[CurrencyInfo { code: r#"DOP"#, numeric: 214, minor_units: 2, name: r#"Dominican Peso"#, entity: r#"DOMINICAN REPUBLIC (THE)"# }],
    "KWD" => &[CurrencyInfo { code: r#"KWD"#, numeric: 414, minor_units: 3, name: r#"Kuwaiti Dinar"#, entity: r#"KUWAIT"# }],
    "SHP" => &[CurrencyInfo { code: r#"SHP"#, numeric: 654, minor_units: 2, name: r#"Saint Helena Pound"#, entity: r#"SAINT HELENA, ASCENSION AND TRISTAN DA CUNHA"# }],
    "TND" => &[CurrencyInfo { code: r#"TND"#, numeric: 788, minor_units: 3, name: r#"Tunisian Dinar"#, entity: r#"TUNISIA"# }],
    "CRC" => &[CurrencyInfo { code: r#"CRC"#, numeric: 188, minor_units: 2, name: r#"Costa Rican Colon"#, entity: r#"COSTA RICA"# }],
    "ARS" => &[CurrencyInfo { code: r#"ARS"#, numeric: 32, minor_units: 2, name: r#"Argentine Peso"#, entity: r#"ARGENTINA"# }],
    "CZK" => &[CurrencyInfo { code: r#"CZK"#, numeric: 203, minor_units: 2, name: r#"Czech Koruna"#, entity: r#"CZECHIA"# }],
    "ZMW" => &[CurrencyInfo { code: r#"ZMW"#, numeric: 967, minor_units: 2, name: r#"Zambian Kwacha"#, entity: r#"ZAMBIA"# }],
    "LSL" => &[CurrencyInfo { code: r#"LSL"#, numeric: 426, minor_units: 2, name: r#"Loti"#, entity: r#"LESOTHO"# }],
    "MGA" => &[CurrencyInfo { code: r#"MGA"#, numeric: 969, minor_units: 2, name: r#"Malagasy Ariary"#, entity: r#"MADAGASCAR"# }],
    "ZWG" => &[CurrencyInfo { code: r#"ZWG"#, numeric: 924, minor_units: 2, name: r#"Zimbabwe Gold"#, entity: r#"ZIMBABWE"# }],
    "NOK" => &[CurrencyInfo { code: r#"NOK"#, numeric: 578, minor_units: 2, name: r#"Norwegian Krone"#, entity: r#"BOUVET ISLAND"# }
             , CurrencyInfo { code: r#"NOK"#, numeric: 578, minor_units: 2, name: r#"Norwegian Krone"#, entity: r#"NORWAY"# }
             , CurrencyInfo { code: r#"NOK"#, numeric: 578, minor_units: 2, name: r#"Norwegian Krone"#, entity: r#"SVALBARD AND JAN MAYEN"# }],
    "XTS" => &[CurrencyInfo { code: r#"XTS"#, numeric: 963, minor_units: 0, name: r#"Codes specifically reserved for testing purposes"#, entity: r#"ZZ06_Testing_Code"# }],
    "KES" => &[CurrencyInfo { code: r#"KES"#, numeric: 404, minor_units: 2, name: r#"Kenyan Shilling"#, entity: r#"KENYA"# }],
    "COP" => &[CurrencyInfo { code: r#"COP"#, numeric: 170, minor_units: 2, name: r#"Colombian Peso"#, entity: r#"COLOMBIA"# }],
    "SLE" => &[CurrencyInfo { code: r#"SLE"#, numeric: 925, minor_units: 2, name: r#"Leone"#, entity: r#"SIERRA LEONE"# }],
    "CNY" => &[CurrencyInfo { code: r#"CNY"#, numeric: 156, minor_units: 2, name: r#"Yuan Renminbi"#, entity: r#"CHINA"# }],
    "NZD" => &[CurrencyInfo { code: r#"NZD"#, numeric: 554, minor_units: 2, name: r#"New Zealand Dollar"#, entity: r#"COOK ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"NZD"#, numeric: 554, minor_units: 2, name: r#"New Zealand Dollar"#, entity: r#"NEW ZEALAND"# }
             , CurrencyInfo { code: r#"NZD"#, numeric: 554, minor_units: 2, name: r#"New Zealand Dollar"#, entity: r#"NIUE"# }
             , CurrencyInfo { code: r#"NZD"#, numeric: 554, minor_units: 2, name: r#"New Zealand Dollar"#, entity: r#"PITCAIRN"# }
             , CurrencyInfo { code: r#"NZD"#, numeric: 554, minor_units: 2, name: r#"New Zealand Dollar"#, entity: r#"TOKELAU"# }],
    "SVC" => &[CurrencyInfo { code: r#"SVC"#, numeric: 222, minor_units: 2, name: r#"El Salvador Colon"#, entity: r#"EL SALVADOR"# }],
    "HKD" => &[CurrencyInfo { code: r#"HKD"#, numeric: 344, minor_units: 2, name: r#"Hong Kong Dollar"#, entity: r#"HONG KONG"# }],
    "PEN" => &[CurrencyInfo { code: r#"PEN"#, numeric: 604, minor_units: 2, name: r#"Sol"#, entity: r#"PERU"# }],
    "ETB" => &[CurrencyInfo { code: r#"ETB"#, numeric: 230, minor_units: 2, name: r#"Ethiopian Birr"#, entity: r#"ETHIOPIA"# }],
    "TJS" => &[CurrencyInfo { code: r#"TJS"#, numeric: 972, minor_units: 2, name: r#"Somoni"#, entity: r#"TAJIKISTAN"# }],
    "UYW" => &[CurrencyInfo { code: r#"UYW"#, numeric: 927, minor_units: 4, name: r#"Unidad Previsional"#, entity: r#"URUGUAY"# }],
    "BOV" => &[CurrencyInfo { code: r#"BOV"#, numeric: 984, minor_units: 2, name: r#"Mvdol"#, entity: r#"BOLIVIA (PLURINATIONAL STATE OF)"# }],
    "BZD" => &[CurrencyInfo { code: r#"BZD"#, numeric: 84, minor_units: 2, name: r#"Belize Dollar"#, entity: r#"BELIZE"# }],
    "FKP" => &[CurrencyInfo { code: r#"FKP"#, numeric: 238, minor_units: 2, name: r#"Falkland Islands Pound"#, entity: r#"FALKLAND ISLANDS (THE) [MALVINAS]"# }],
    "MKD" => &[CurrencyInfo { code: r#"MKD"#, numeric: 807, minor_units: 2, name: r#"Denar"#, entity: r#"NORTH MACEDONIA"# }],
    "VND" => &[CurrencyInfo { code: r#"VND"#, numeric: 704, minor_units: 0, name: r#"Dong"#, entity: r#"VIET NAM"# }],
    "XBC" => &[CurrencyInfo { code: r#"XBC"#, numeric: 957, minor_units: 0, name: r#"Bond Markets Unit European Unit of Account 9 (E.U.A.-9)"#, entity: r#"ZZ03_Bond Markets Unit European_EUA-9"# }],
    "STN" => &[CurrencyInfo { code: r#"STN"#, numeric: 930, minor_units: 2, name: r#"Dobra"#, entity: r#"SAO TOME AND PRINCIPE"# }],
    "XBB" => &[CurrencyInfo { code: r#"XBB"#, numeric: 956, minor_units: 0, name: r#"Bond Markets Unit European Monetary Unit (E.M.U.-6)"#, entity: r#"ZZ02_Bond Markets Unit European_EMU-6"# }],
    "LYD" => &[CurrencyInfo { code: r#"LYD"#, numeric: 434, minor_units: 3, name: r#"Libyan Dinar"#, entity: r#"LIBYA"# }],
    "XBA" => &[CurrencyInfo { code: r#"XBA"#, numeric: 955, minor_units: 0, name: r#"Bond Markets Unit European Composite Unit (EURCO)"#, entity: r#"ZZ01_Bond Markets Unit European_EURCO"# }],
    "NIO" => &[CurrencyInfo { code: r#"NIO"#, numeric: 558, minor_units: 2, name: r#"Cordoba Oro"#, entity: r#"NICARAGUA"# }],
    "INR" => &[CurrencyInfo { code: r#"INR"#, numeric: 356, minor_units: 2, name: r#"Indian Rupee"#, entity: r#"BHUTAN"# }
             , CurrencyInfo { code: r#"INR"#, numeric: 356, minor_units: 2, name: r#"Indian Rupee"#, entity: r#"INDIA"# }],
    "CVE" => &[CurrencyInfo { code: r#"CVE"#, numeric: 132, minor_units: 2, name: r#"Cabo Verde Escudo"#, entity: r#"CABO VERDE"# }],
    "BYN" => &[CurrencyInfo { code: r#"BYN"#, numeric: 933, minor_units: 2, name: r#"Belarusian Ruble"#, entity: r#"BELARUS"# }],
    "NPR" => &[CurrencyInfo { code: r#"NPR"#, numeric: 524, minor_units: 2, name: r#"Nepalese Rupee"#, entity: r#"NEPAL"# }],
    "RON" => &[CurrencyInfo { code: r#"RON"#, numeric: 946, minor_units: 2, name: r#"Romanian Leu"#, entity: r#"ROMANIA"# }],
    "GNF" => &[CurrencyInfo { code: r#"GNF"#, numeric: 324, minor_units: 0, name: r#"Guinean Franc"#, entity: r#"GUINEA"# }],
    "LAK" => &[CurrencyInfo { code: r#"LAK"#, numeric: 418, minor_units: 2, name: r#"Lao Kip"#, entity: r#"LAO PEOPLE’S DEMOCRATIC REPUBLIC (THE)"# }],
    "NGN" => &[CurrencyInfo { code: r#"NGN"#, numeric: 566, minor_units: 2, name: r#"Naira"#, entity: r#"NIGERIA"# }],
    "WST" => &[CurrencyInfo { code: r#"WST"#, numeric: 882, minor_units: 2, name: r#"Tala"#, entity: r#"SAMOA"# }],
    "SYP" => &[CurrencyInfo { code: r#"SYP"#, numeric: 760, minor_units: 2, name: r#"Syrian Pound"#, entity: r#"SYRIAN ARAB REPUBLIC"# }],
    "KMF" => &[CurrencyInfo { code: r#"KMF"#, numeric: 174, minor_units: 0, name: r#"Comorian Franc"#, entity: r#"COMOROS (THE)"# }],
    "XCD" => &[CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"ANGUILLA"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"ANTIGUA AND BARBUDA"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"DOMINICA"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"GRENADA"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"MONTSERRAT"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"SAINT KITTS AND NEVIS"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"SAINT LUCIA"# }
             , CurrencyInfo { code: r#"XCD"#, numeric: 951, minor_units: 2, name: r#"East Caribbean Dollar"#, entity: r#"SAINT VINCENT AND THE GRENADINES"# }],
    "KZT" => &[CurrencyInfo { code: r#"KZT"#, numeric: 398, minor_units: 2, name: r#"Tenge"#, entity: r#"KAZAKHSTAN"# }],
    "ISK" => &[CurrencyInfo { code: r#"ISK"#, numeric: 352, minor_units: 0, name: r#"Iceland Krona"#, entity: r#"ICELAND"# }],
    "DZD" => &[CurrencyInfo { code: r#"DZD"#, numeric: 12, minor_units: 2, name: r#"Algerian Dinar"#, entity: r#"ALGERIA"# }],
    "BND" => &[CurrencyInfo { code: r#"BND"#, numeric: 96, minor_units: 2, name: r#"Brunei Dollar"#, entity: r#"BRUNEI DARUSSALAM"# }],
    "AFN" => &[CurrencyInfo { code: r#"AFN"#, numeric: 971, minor_units: 2, name: r#"Afghani"#, entity: r#"AFGHANISTAN"# }],
    "GMD" => &[CurrencyInfo { code: r#"GMD"#, numeric: 270, minor_units: 2, name: r#"Dalasi"#, entity: r#"GAMBIA (THE)"# }],
    "BWP" => &[CurrencyInfo { code: r#"BWP"#, numeric: 72, minor_units: 2, name: r#"Pula"#, entity: r#"BOTSWANA"# }],
    "MXN" => &[CurrencyInfo { code: r#"MXN"#, numeric: 484, minor_units: 2, name: r#"Mexican Peso"#, entity: r#"MEXICO"# }],
    "MZN" => &[CurrencyInfo { code: r#"MZN"#, numeric: 943, minor_units: 2, name: r#"Mozambique Metical"#, entity: r#"MOZAMBIQUE"# }],
    "USD" => &[CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"AMERICAN SAMOA"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"BONAIRE, SINT EUSTATIUS AND SABA"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"BRITISH INDIAN OCEAN TERRITORY (THE)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"ECUADOR"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"EL SALVADOR"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"GUAM"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"HAITI"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"MARSHALL ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"MICRONESIA (FEDERATED STATES OF)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"NORTHERN MARIANA ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"PALAU"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"PANAMA"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"PUERTO RICO"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"TIMOR-LESTE"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"TURKS AND CAICOS ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"UNITED STATES MINOR OUTLYING ISLANDS (THE)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"UNITED STATES OF AMERICA (THE)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"VIRGIN ISLANDS (BRITISH)"# }
             , CurrencyInfo { code: r#"USD"#, numeric: 840, minor_units: 2, name: r#"US Dollar"#, entity: r#"VIRGIN ISLANDS (U.S.)"# }],
    "MNT" => &[CurrencyInfo { code: r#"MNT"#, numeric: 496, minor_units: 2, name: r#"Tugrik"#, entity: r#"MONGOLIA"# }],
    "GBP" => &[CurrencyInfo { code: r#"GBP"#, numeric: 826, minor_units: 2, name: r#"Pound Sterling"#, entity: r#"GUERNSEY"# }
             , CurrencyInfo { code: r#"GBP"#, numeric: 826, minor_units: 2, name: r#"Pound Sterling"#, entity: r#"ISLE OF MAN"# }
             , CurrencyInfo { code: r#"GBP"#, numeric: 826, minor_units: 2, name: r#"Pound Sterling"#, entity: r#"JERSEY"# }
             , CurrencyInfo { code: r#"GBP"#, numeric: 826, minor_units: 2, name: r#"Pound Sterling"#, entity: r#"UNITED KINGDOM OF GREAT BRITAIN AND NORTHERN IRELAND (THE)"# }],
    "PKR" => &[CurrencyInfo { code: r#"PKR"#, numeric: 586, minor_units: 2, name: r#"Pakistan Rupee"#, entity: r#"PAKISTAN"# }],
    "SAR" => &[CurrencyInfo { code: r#"SAR"#, numeric: 682, minor_units: 2, name: r#"Saudi Riyal"#, entity: r#"SAUDI ARABIA"# }],
    "UYI" => &[CurrencyInfo { code: r#"UYI"#, numeric: 940, minor_units: 0, name: r#"Uruguay Peso en Unidades Indexadas (UI)"#, entity: r#"URUGUAY"# }],
    "VUV" => &[CurrencyInfo { code: r#"VUV"#, numeric: 548, minor_units: 0, name: r#"Vatu"#, entity: r#"VANUATU"# }],
    "MRU" => &[CurrencyInfo { code: r#"MRU"#, numeric: 929, minor_units: 2, name: r#"Ouguiya"#, entity: r#"MAURITANIA"# }],
    "BRL" => &[CurrencyInfo { code: r#"BRL"#, numeric: 986, minor_units: 2, name: r#"Brazilian Real"#, entity: r#"BRAZIL"# }],
    "KGS" => &[CurrencyInfo { code: r#"KGS"#, numeric: 417, minor_units: 2, name: r#"Som"#, entity: r#"KYRGYZSTAN"# }],
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    AFN,
    EUR,
    ALL,
    DZD,
    USD,
    AOA,
    XCD,
    XAD,
    ARS,
    AMD,
    AWG,
    AUD,
    AZN,
    BSD,
    BHD,
    BDT,
    BBD,
    BYN,
    BZD,
    XOF,
    BMD,
    INR,
    BTN,
    BOB,
    BOV,
    BAM,
    BWP,
    NOK,
    BRL,
    BND,
    BGN,
    BIF,
    CVE,
    KHR,
    XAF,
    CAD,
    KYD,
    CLP,
    CLF,
    CNY,
    COP,
    COU,
    KMF,
    CDF,
    NZD,
    CRC,
    CUP,
    XCG,
    CZK,
    DKK,
    DJF,
    DOP,
    EGP,
    SVC,
    ERN,
    SZL,
    ETB,
    FKP,
    FJD,
    XPF,
    GMD,
    GEL,
    GHS,
    GIP,
    GTQ,
    GBP,
    GNF,
    GYD,
    HTG,
    HNL,
    HKD,
    HUF,
    ISK,
    IDR,
    XDR,
    IRR,
    IQD,
    ILS,
    JMD,
    JPY,
    JOD,
    KZT,
    KES,
    KPW,
    KRW,
    KWD,
    KGS,
    LAK,
    LBP,
    LSL,
    ZAR,
    LRD,
    LYD,
    CHF,
    MOP,
    MKD,
    MGA,
    MWK,
    MYR,
    MVR,
    MRU,
    MUR,
    XUA,
    MXN,
    MXV,
    MDL,
    MNT,
    MAD,
    MZN,
    MMK,
    NAD,
    NPR,
    NIO,
    NGN,
    OMR,
    PKR,
    PAB,
    PGK,
    PYG,
    PEN,
    PHP,
    PLN,
    QAR,
    RON,
    RUB,
    RWF,
    SHP,
    WST,
    STN,
    SAR,
    RSD,
    SCR,
    SLE,
    SGD,
    XSU,
    SBD,
    SOS,
    SSP,
    LKR,
    SDG,
    SRD,
    SEK,
    CHE,
    CHW,
    SYP,
    TWD,
    TJS,
    TZS,
    THB,
    TOP,
    TTD,
    TND,
    TRY,
    TMT,
    UGX,
    UAH,
    AED,
    USN,
    UYU,
    UYI,
    UYW,
    UZS,
    VUV,
    VES,
    VED,
    VND,
    YER,
    ZMW,
    ZWG,
    XBA,
    XBB,
    XBC,
    XBD,
    XTS,
    XXX,
    XAU,
    XPD,
    XPT,
    XAG,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::AFN => "AFN",
            Currency::EUR => "EUR",
            Currency::ALL => "ALL",
            Currency::DZD => "DZD",
            Currency::USD => "USD",
            Currency::AOA => "AOA",
            Currency::XCD => "XCD",
            Currency::XAD => "XAD",
            Currency::ARS => "ARS",
            Currency::AMD => "AMD",
            Currency::AWG => "AWG",
            Currency::AUD => "AUD",
            Currency::AZN => "AZN",
            Currency::BSD => "BSD",
            Currency::BHD => "BHD",
            Currency::BDT => "BDT",
            Currency::BBD => "BBD",
            Currency::BYN => "BYN",
            Currency::BZD => "BZD",
            Currency::XOF => "XOF",
            Currency::BMD => "BMD",
            Currency::INR => "INR",
            Currency::BTN => "BTN",
            Currency::BOB => "BOB",
            Currency::BOV => "BOV",
            Currency::BAM => "BAM",
            Currency::BWP => "BWP",
            Currency::NOK => "NOK",
            Currency::BRL => "BRL",
            Currency::BND => "BND",
            Currency::BGN => "BGN",
            Currency::BIF => "BIF",
            Currency::CVE => "CVE",
            Currency::KHR => "KHR",
            Currency::XAF => "XAF",
            Currency::CAD => "CAD",
            Currency::KYD => "KYD",
            Currency::CLP => "CLP",
            Currency::CLF => "CLF",
            Currency::CNY => "CNY",
            Currency::COP => "COP",
            Currency::COU => "COU",
            Currency::KMF => "KMF",
            Currency::CDF => "CDF",
            Currency::NZD => "NZD",
            Currency::CRC => "CRC",
            Currency::CUP => "CUP",
            Currency::XCG => "XCG",
            Currency::CZK => "CZK",
            Currency::DKK => "DKK",
            Currency::DJF => "DJF",
            Currency::DOP => "DOP",
            Currency::EGP => "EGP",
            Currency::SVC => "SVC",
            Currency::ERN => "ERN",
            Currency::SZL => "SZL",
            Currency::ETB => "ETB",
            Currency::FKP => "FKP",
            Currency::FJD => "FJD",
            Currency::XPF => "XPF",
            Currency::GMD => "GMD",
            Currency::GEL => "GEL",
            Currency::GHS => "GHS",
            Currency::GIP => "GIP",
            Currency::GTQ => "GTQ",
            Currency::GBP => "GBP",
            Currency::GNF => "GNF",
            Currency::GYD => "GYD",
            Currency::HTG => "HTG",
            Currency::HNL => "HNL",
            Currency::HKD => "HKD",
            Currency::HUF => "HUF",
            Currency::ISK => "ISK",
            Currency::IDR => "IDR",
            Currency::XDR => "XDR",
            Currency::IRR => "IRR",
            Currency::IQD => "IQD",
            Currency::ILS => "ILS",
            Currency::JMD => "JMD",
            Currency::JPY => "JPY",
            Currency::JOD => "JOD",
            Currency::KZT => "KZT",
            Currency::KES => "KES",
            Currency::KPW => "KPW",
            Currency::KRW => "KRW",
            Currency::KWD => "KWD",
            Currency::KGS => "KGS",
            Currency::LAK => "LAK",
            Currency::LBP => "LBP",
            Currency::LSL => "LSL",
            Currency::ZAR => "ZAR",
            Currency::LRD => "LRD",
            Currency::LYD => "LYD",
            Currency::CHF => "CHF",
            Currency::MOP => "MOP",
            Currency::MKD => "MKD",
            Currency::MGA => "MGA",
            Currency::MWK => "MWK",
            Currency::MYR => "MYR",
            Currency::MVR => "MVR",
            Currency::MRU => "MRU",
            Currency::MUR => "MUR",
            Currency::XUA => "XUA",
            Currency::MXN => "MXN",
            Currency::MXV => "MXV",
            Currency::MDL => "MDL",
            Currency::MNT => "MNT",
            Currency::MAD => "MAD",
            Currency::MZN => "MZN",
            Currency::MMK => "MMK",
            Currency::NAD => "NAD",
            Currency::NPR => "NPR",
            Currency::NIO => "NIO",
            Currency::NGN => "NGN",
            Currency::OMR => "OMR",
            Currency::PKR => "PKR",
            Currency::PAB => "PAB",
            Currency::PGK => "PGK",
            Currency::PYG => "PYG",
            Currency::PEN => "PEN",
            Currency::PHP => "PHP",
            Currency::PLN => "PLN",
            Currency::QAR => "QAR",
            Currency::RON => "RON",
            Currency::RUB => "RUB",
            Currency::RWF => "RWF",
            Currency::SHP => "SHP",
            Currency::WST => "WST",
            Currency::STN => "STN",
            Currency::SAR => "SAR",
            Currency::RSD => "RSD",
            Currency::SCR => "SCR",
            Currency::SLE => "SLE",
            Currency::SGD => "SGD",
            Currency::XSU => "XSU",
            Currency::SBD => "SBD",
            Currency::SOS => "SOS",
            Currency::SSP => "SSP",
            Currency::LKR => "LKR",
            Currency::SDG => "SDG",
            Currency::SRD => "SRD",
            Currency::SEK => "SEK",
            Currency::CHE => "CHE",
            Currency::CHW => "CHW",
            Currency::SYP => "SYP",
            Currency::TWD => "TWD",
            Currency::TJS => "TJS",
            Currency::TZS => "TZS",
            Currency::THB => "THB",
            Currency::TOP => "TOP",
            Currency::TTD => "TTD",
            Currency::TND => "TND",
            Currency::TRY => "TRY",
            Currency::TMT => "TMT",
            Currency::UGX => "UGX",
            Currency::UAH => "UAH",
            Currency::AED => "AED",
            Currency::USN => "USN",
            Currency::UYU => "UYU",
            Currency::UYI => "UYI",
            Currency::UYW => "UYW",
            Currency::UZS => "UZS",
            Currency::VUV => "VUV",
            Currency::VES => "VES",
            Currency::VED => "VED",
            Currency::VND => "VND",
            Currency::YER => "YER",
            Currency::ZMW => "ZMW",
            Currency::ZWG => "ZWG",
            Currency::XBA => "XBA",
            Currency::XBB => "XBB",
            Currency::XBC => "XBC",
            Currency::XBD => "XBD",
            Currency::XTS => "XTS",
            Currency::XXX => "XXX",
            Currency::XAU => "XAU",
            Currency::XPD => "XPD",
            Currency::XPT => "XPT",
            Currency::XAG => "XAG",
        }
    }

    pub fn info(&self) -> &'static [CurrencyInfo] {
        CURRENCIES.get(self.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_single_currency() {
        let code = "BDT";
        let currency_infos = CURRENCIES.get(code).expect("Currency should exist");
        println!("Currency infos for {}: {:?}", code, currency_infos);

        assert_eq!(currency_infos.len(), 1);

        let info = &currency_infos[0];
        println!("Code: {}, Numeric: {}, Minor Units: {}, Name: {}, Entity: {}",
                 info.code, info.numeric, info.minor_units, info.name, info.entity);

        assert_eq!(info.code, "BDT");
        assert_eq!(info.numeric, 50);
        assert_eq!(info.minor_units, 2);
        assert_eq!(info.name, "Taka");
        assert_eq!(info.entity, "BANGLADESH");
    }

    #[test]
    fn test_lookup_multiple_entities() {
        let code = "EUR";
        let currency_infos = CURRENCIES.get(code).expect("Currency should exist");
        println!("Currency infos for {}: {:?}", code, currency_infos);

        assert!(currency_infos.len() > 1);

        let entities: Vec<_> = currency_infos.iter().map(|c| c.entity).collect();
        println!("Entities for {}: {:?}", code, entities);

        assert!(entities.contains(&"PORTUGAL"));
        assert!(entities.contains(&"FRANCE"));
    }

    #[test]
    fn test_currency_not_found() {
        let code = "XXX_NOT_REAL";
        let result = CURRENCIES.get(code);

        assert!(result.is_none());
    }

    #[test]
    fn test_minor_units_and_numeric() {
        let code = "OMR"; // 3 minor units
        let info = CURRENCIES.get(code).unwrap().first().unwrap();
        assert_eq!(info.minor_units, 3);
        assert_eq!(info.numeric, 512);

        let code = "JPY"; // 0 minor units
        let info = CURRENCIES.get(code).unwrap().first().unwrap();
        assert_eq!(info.minor_units, 0);
        assert_eq!(info.numeric, 392);
    }

    #[test]
    fn test_unique_currency_codes() {
        for (&code, _) in CURRENCIES.entries() {
            println!("Checking code: {}", code);
            assert_eq!(code.len(), 3);
            assert!(code.chars().all(|c| c.is_ascii_uppercase()));
        }
    }
}
