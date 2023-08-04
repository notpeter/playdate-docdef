use std::{collections::HashMap, fmt};

use regex::Regex;
use lazy_static::lazy_static;
use toml::{Table, Value};
use serde::Deserialize;


#[derive(Deserialize)]
struct AnchorOverride {
    fname: String,
    parameters: Vec<String>, // You must include a parameters=[] if there are no params.
    ignore: Option<bool>
}

impl fmt::Display for AnchorOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.fname, self.parameters.join(", "))
    }
}


static FIXES_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Fixes.toml"));
static REPLACE_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Replace.toml"));

fn lua_function_regex() -> Regex {
    let id = r"[\w_][\w\d_]*";
    let f = format!(r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)");
    Regex::new(&f).unwrap()
}

lazy_static! {
    static ref RE_CODE: Regex = Regex::new(r"</?code>").unwrap();
    static ref RE_EM: Regex = Regex::new(r"</?em>").unwrap();
    static ref RE_A: Regex = Regex::new(r"</?a[^>]*>").unwrap();
    static ref RE_STRONG: Regex = Regex::new(r"</?strong>").unwrap();
    static ref HTML_TAG: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref LUA_FUNC: Regex = lua_function_regex();
    //TODO: Check to ensure "…" gets cleaned too.
    static ref FIXES: HashMap<String, AnchorOverride> = load_fixes(&FIXES_STR);
    static ref REPLACEMENTS: HashMap<String, String> = load_replacements(&REPLACE_STR);
}

pub fn get_overrides(anchor: &str) -> Option<(String, Vec<String>)> {
    if FIXES.contains_key(anchor) {
        let fixed = FIXES.get(anchor).unwrap();
        eprintln!("WARN: Found function override: {} -> {}", anchor, fixed);
        return Some((fixed.fname.clone(), fixed.parameters.clone()));
    }
    None
}

// Takes a valid function signature and returns a function name and a vector of parameters.
pub fn params_from_title(title: &String) -> (String, Vec<String>) {
    let mut params = Vec::new();
    let caps = LUA_FUNC.captures(title).unwrap();

    let params_str = caps.name("params").unwrap().as_str();
    let fname = caps.name("fname").unwrap().as_str();
    for p in params_str.split(",") {
        params.push(p.trim().to_string());
    }
    (fname.to_string(), clean_parameters(&title, &params))
}

pub fn clean_admonition(adm: String) -> String {
    let a1 = RE_CODE.replace_all(&adm, "`");
    let a2 = RE_EM.replace_all(&a1, "*");
    let a3 = RE_A.replace_all(&a2, "");
    let a4 = RE_STRONG.replace_all(&a3, "**");
    let an = a4.trim().to_string();
    if HTML_TAG.is_match(&an) {
        eprintln!("WARN: Extra HTML tag in admonition: {}", an.to_string());
    }
    an.to_string()
}


fn clean_parameters(title: &String, params: &Vec<String>) -> Vec<String> {
    let mut v = Vec::new();
    for p in params {
        if REPLACEMENTS.contains_key(p.as_str()) {
            let fixed_param = REPLACEMENTS.get(p.as_str()).unwrap().to_string();
            eprintln!("WARN: Fixed invalid parameter: {p} -> {fixed_param} (in `{title}`)");
            v.push(fixed_param);
        } else {
            v.push(p.to_string());
        }
    }
    v
}

fn load_fixes(fixes_toml: &str) -> HashMap<String, AnchorOverride> {
    let overrides: HashMap<String, AnchorOverride> = match toml::from_str(fixes_toml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            HashMap::new()
        }
    };
    overrides
}

fn load_replacements(replace_toml: &str) -> HashMap<String, String> {
    let replacements: HashMap<String, String> = match toml::from_str(replace_toml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
            HashMap::new()
        }
    };
    replacements
}
