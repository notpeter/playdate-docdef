use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
use serde::Deserialize;

static TOML_STR_TYPO: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Typo.toml"));
static TOML_STR_INVALID: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Invalid.toml"));

lazy_static! {
    pub static ref TYPO: HashMap<String, TypoReplacement> = match toml::from_str(TOML_STR_TYPO) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Typo.toml failed. {:?}", e) } };

    pub static ref INVALID: HashMap<String, String> = match toml::from_str(TOML_STR_INVALID) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Invalid.toml failed. {:?}", e); } };
}

#[derive(Deserialize)]
pub struct TypoReplacement {
    pub fname: String,
    pub parameters: Vec<String>, // You must include a parameters=[] if there are no params.
}

impl fmt::Display for TypoReplacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.fname, self.parameters.join(", "))
    }
}
