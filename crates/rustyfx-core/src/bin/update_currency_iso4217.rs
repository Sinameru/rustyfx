use anyhow::{Context, Result};
use reqwest::blocking::get;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str;

#[derive(Default)]
struct TmpCurrency {
    code: Option<String>,
    numeric: Option<u16>,
    minor_units: Option<u8>,
    name: Option<String>,
    entity: Option<String>,
}

fn main() -> Result<()> {
    const GEN_CURRENCY_OUT_PATH: &str = "./generated_currency.rs";
    const SOURCE_URL: &str =
        "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml";

    let args: Vec<String> = std::env::args().collect();
    let out_path = args
        .get(1)
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|| PathBuf::from(GEN_CURRENCY_OUT_PATH));

    println!("Fetching ISO 4217 list from: {}", SOURCE_URL);
    let resp = get(SOURCE_URL)
        .with_context(|| format!("failed to GET {}", SOURCE_URL))?
        .text()
        .context("failed to read response body")?;

    let mut reader = Reader::from_str(&resp);
  //  reader.trim_text(true);
    let mut buf = Vec::new();

    let mut tmp = TmpCurrency::default();
    let mut tmp_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut enum_variants: Vec<String> = Vec::new();
    let mut variants_set: HashSet<String> = HashSet::new();
    let mut current_tag: Option<String> = None;

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match &event {
            Event::Start(e) => {
                current_tag = Some(str::from_utf8(e.name().as_ref())?.to_string());
            }
            Event::Text(e) => {
                let text = str::from_utf8(e.as_ref())?.trim();
                if text.is_empty() {
                    buf.clear();
                    continue;
                }

                if let Some(tag) = &current_tag {
                    match tag.as_str() {
                        "Ccy" => tmp.code = Some(text.to_string()),
                        "CcyNbr" => tmp.numeric = text.parse::<u16>().ok(),
                        "CcyMnrUnts" => tmp.minor_units =
                            if text == "N.A." { Some(0) } else { text.parse::<u8>().ok() },
                        "CcyNm" => tmp.name = Some(text.to_string()),
                        "CtryNm" => tmp.entity = Some(text.to_string()),
                        _ => {}
                    }
                }
            }
            Event::End(e) if e.name().as_ref() == b"CcyNtry" => {
                if let Some(code) = &tmp.code {
                    let entry = format!(
                        "CurrencyInfo {{ code: r#\"{}\"#, numeric: {}, minor_units: {}, name: r#\"{}\"#, entity: r#\"{}\"# }}",
                        code,
                        tmp.numeric.unwrap_or(0),
                        tmp.minor_units.unwrap_or(0),
                        tmp.name.clone().unwrap_or_default(),
                        tmp.entity.clone().unwrap_or_default()
                    );

                    tmp_map.entry(code.clone()).or_default().push(entry);

                    // Criar variante de enum segura
                    let variant = code
                        .chars()
                        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                        .collect::<String>();

                    if variants_set.insert(variant.clone()) {
                        enum_variants.push(variant);
                    }
                } else {
                    eprintln!("Skipping entry with missing currency code: {:?}", tmp.name);
                }

                tmp = TmpCurrency::default();
                current_tag = None;
            }
            Event::End(_) => {
                current_tag = None;
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    let mut out_file = File::create(out_path)?;

    writeln!(out_file, "use phf::phf_map;\n")?;
    writeln!(out_file, "pub struct CurrencyInfo {{")?;
    writeln!(out_file, "    pub code: &'static str,")?;
    writeln!(out_file, "    pub numeric: u16,")?;
    writeln!(out_file, "    pub minor_units: u8,")?;
    writeln!(out_file, "    pub name: &'static str,")?;
    writeln!(out_file, "    pub entity: &'static str,")?;
    writeln!(out_file, "}}\n")?;

    writeln!(
        out_file,
        "pub static CURRENCIES: phf::Map<&'static str, &'static [CurrencyInfo]> = phf_map! {{"
    )?;
    for (code, entries) in &tmp_map {
        writeln!(out_file, "    \"{}\" => &[{}],", code, entries.join("\n             , "))?;
    }
    writeln!(out_file, "}};\n")?;

    writeln!(out_file, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(out_file, "pub enum Currency {{")?;
    for v in &enum_variants {
        writeln!(out_file, "    {},", v)?;
    }
    writeln!(out_file, "}}\n")?;

    writeln!(out_file, "impl Currency {{")?;
    writeln!(out_file, "    pub fn as_str(&self) -> &'static str {{")?;
    writeln!(out_file, "        match self {{")?;
    for v in &enum_variants {
        writeln!(out_file, "            Currency::{} => \"{}\",", v, v)?;
    }
    writeln!(out_file, "        }}")?;
    writeln!(out_file, "    }}\n")?;

    writeln!(out_file, "    pub fn info(&self) -> &'static [CurrencyInfo] {{")?;
    writeln!(out_file, "        CURRENCIES.get(self.as_str()).unwrap()")?;
    writeln!(out_file, "    }}")?;
    writeln!(out_file, "}}")?;

    println!("generated_currency.rs successfully created!");
    Ok(())
}