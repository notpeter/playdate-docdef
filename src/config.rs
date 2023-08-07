use std::{collections::HashMap, fmt};

use indexmap::IndexMap;
use lazy_static::lazy_static;
use serde::Deserialize;

static TOML_STR_TYPO: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Typo.toml"));
static TOML_STR_INVALID: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Invalid.toml"));
static TOML_STR_TYPES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Type.toml"));
static TOML_STR_CLASS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Class.toml"));
static TOML_STR_RETURN: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Return.toml"));

lazy_static! {
    pub static ref TYPO: HashMap<String, TypoReplacement> = match toml::from_str(TOML_STR_TYPO) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Typo.toml failed. {:?}", e) } };

    pub static ref INVALID: HashMap<String, String> = match toml::from_str(TOML_STR_INVALID) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Invalid.toml failed. {:?}", e); } };

    pub static ref PARAM_TYPES: HashMap<String, String> = match toml::from_str(TOML_STR_TYPES) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Type.toml failed. {:?}", e); } };

    pub static ref CLASS: IndexMap<String, IndexMap<String, String>> = match toml::from_str(TOML_STR_CLASS) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Class.toml failed. {:?}", e); } };

    pub static ref RETURN: IndexMap<String, IndexMap<String, String>> = load_return(TOML_STR_RETURN);
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

fn load_return(tom_return: &str) -> IndexMap<String, IndexMap<String, String>> {
    let r : IndexMap<String, IndexMap<String, IndexMap<String, String>>> = match toml::from_str(tom_return) {
        Ok(v) => v, Err(e) => { panic!("ERROR: Loading Return.toml failed. {:?}", e); }
    };
    let mut out: IndexMap<String, IndexMap<String, String>> = IndexMap::new();
    for (namespace, funcs) in r {
        for (fname, returns) in funcs {
            let mut r: IndexMap<String, String> = IndexMap::new();
            if returns.is_empty() {
                r.insert("".to_string(), "nil".to_string());
            } else {
                for (ret_name, ret_type) in returns {
                    r.insert(ret_name, ret_type);
                }
            }
            out.insert(format!("{namespace}{fname}"), r);
        }
    }
    out
}
