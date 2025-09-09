use std::fs;
use std::path::Path;
use quick_xml::Reader;
use quick_xml::events::Event;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[derive(Debug)]
struct IsoCurrency {
    alpha3: String,
    numeric: u16,
    minor_units: u8,
    name: String,
}

fn main() {
    println!("test");
}
/*fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.six-group.com/dam/downloads/excel-list-one.xml";
    let xml = reqwest::blocking::get(url)?.text()?;

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut currencies = Vec::new();
    let mut current: Option<IsoCurrency> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"CcyNtry" => {
                current = Some(IsoCurrency {
                    alpha3: String::new(),
                    numeric: 0,
                    minor_units: 2,
                    name: String::new(),
                });
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"CcyNtry" => {
                if let Some(c) = current.take() {
                    currencies.push(c);
                }
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"Ccy" => {
                if let Ok(Event::Text(t)) = reader.read_event() {
                    if let Some(ref mut c) = current {
                        c.alpha3 = t.unescape()?.to_string();
                    }
                }
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"CcyNbr" => {
                if let Ok(Event::Text(t)) = reader.read_event() {
                    if let Some(ref mut c) = current {
                        c.numeric = t.unescape()?.parse()?;
                    }
                }
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"CcyMnrUnts" => {
                if let Ok(Event::Text(t)) = reader.read_event() {
                    if let Some(ref mut c) = current {
                        c.minor_units = t.unescape()?.parse()?;
                    }
                }
            }
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"CcyNm" => {
                if let Ok(Event::Text(t)) = reader.read_event() {
                    if let Some(ref mut c) = current {
                        c.name = t.unescape()?.to_string();
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    let variants: Vec<TokenStream> = currencies.iter().map(|c| {
        let name = Ident::new(&c.alpha3, proc_macro2::Span::call_site());
        let doc = &c.name;
        quote! {
            #[doc = #doc]
            #name,
        }
    }).collect();

    let matches: Vec<TokenStream> = currencies.iter().map(|c| {
        let name = Ident::new(&c.alpha3, proc_macro2::Span::call_site());
        let alpha3 = &c.alpha3;
        let numeric = c.numeric;
        let minor_units = c.minor_units;
        quote! {
            Currency::#name => Some(CurrencyInfo { alpha3: #alpha3, numeric: #numeric, minor_units: #minor_units }),
        }
    }).collect();

    let generated = quote! {
        //! AUTO-GENERATED — do not edit manually
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum Currency {
            #(#variants)*
            Other(String),
        }

        pub struct CurrencyInfo {
            pub alpha3: &'static str,
            pub numeric: u16,
            pub minor_units: u8,
        }

        impl Currency {
            pub fn info(&self) -> Option<CurrencyInfo> {
                match self {
                    #(#matches)*
                    Currency::Other(_) => None,
                }
            }
        }
    };

    let out_path = Path::new("src/currency.rs");
    fs::write(out_path, generated.to_string())?;

    println!("Arquivo src/currency.rs gerado com sucesso.");

    Ok(())
}*/