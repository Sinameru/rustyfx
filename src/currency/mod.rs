/// Represents supported currencies in rustfx
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    AUD,
    CAD,
    CHF,
    CNY,
    SEK,
    NZD,
    // TODO: See https://github.com/Sinameru/rustfx/issues/1 for planned currency additions (e.g., INR, BRL, RUB, ZAR, SGD)
}

impl Currency {
    /// Returns a short string code for the currency
    pub fn code(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::SEK => "SEK",
            Currency::NZD => "NZD",
        }
    }

    /// Parse a string code into a Currency enum
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_uppercase().as_str() {
            "USD" => Some(Currency::USD),
            "EUR" => Some(Currency::EUR),
            "GBP" => Some(Currency::GBP),
            "JPY" => Some(Currency::JPY),
            "AUD" => Some(Currency::AUD),
            "CAD" => Some(Currency::CAD),
            "CHF" => Some(Currency::CHF),
            "CNY" => Some(Currency::CNY),
            "SEK" => Some(Currency::SEK),
            "NZD" => Some(Currency::NZD),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::EUR.code(), "EUR");
    }

    #[test]
    fn test_from_code() {
        assert_eq!(Currency::from_code("usd"), Some(Currency::USD));
        assert_eq!(Currency::from_code("EUR"), Some(Currency::EUR));
        assert_eq!(Currency::from_code("abc"), None);
    }
}
