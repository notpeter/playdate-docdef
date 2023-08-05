use std::{collections::HashMap, fmt};

use regex::Regex;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::stub::Stub;


static TOML_FIXES_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Fixes.toml"));
static TOML_REPLACE_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Replace.toml"));
static TOML_TYPES_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Types.toml"));

lazy_static! {
    static ref RE_CODE: Regex = Regex::new(r"</?code>").unwrap();
    static ref RE_EM: Regex = Regex::new(r"</?em>").unwrap();
    static ref RE_A: Regex = Regex::new(r"</?a[^>]*>").unwrap();
    static ref RE_STRONG: Regex = Regex::new(r"</?strong>").unwrap();
    static ref HTML_TAG: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref LUA_FUNC: Regex = Regex::new(
        &format!(r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)", id=r"[\w_][\w\d_]*").to_string()
    ).unwrap();
    static ref FIXES: HashMap<String, AnchorOverride> = load_fixes(&TOML_FIXES_STR);
    static ref REPLACEMENTS: HashMap<String, String> = load_replacements(&TOML_REPLACE_STR);
    static ref TYPES: TypesTOML = load_types(&TOML_TYPES_STR);
}

#[derive(Deserialize)]
struct AnchorOverride {
    fname: String,
    parameters: Vec<String>, // You must include a parameters=[] if there are no params.
}

impl fmt::Display for AnchorOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.fname, self.parameters.join(", "))
    }
}

#[derive(Deserialize)]
struct TypesTOML {
    types: HashMap<String, String>, // Array of valid types
    by_name: HashMap<String, String>, // parameter_name=type_name
}

// Given a function return a stub with overrides and paramer types applied
pub fn annotate_function(anchor: &str, title: &String, text: &Vec<String>) -> Stub {
    let fname: String;
    let mut params: Vec<(String, String)>;

    // Apply overrides
    if FIXES.contains_key(anchor) {
        let fixed = FIXES.get(anchor).unwrap();
        params = fixed.parameters.iter().map(|p| (p.clone(), "any".to_string())).collect();
        eprintln!("WARN: Found function override: {} -> {}", anchor, fixed);
        fname = fixed.fname.clone();
    } else {
        (fname, params) = params_from_title(title);
    }
    // Apply types from Types.toml
    for (p, t) in params.iter_mut() {
        if TYPES.by_name.contains_key(p) {
            *t = TYPES.by_name.get(p).unwrap().to_string();
        }
    }

    Stub {
        title: fname,
        anchor: anchor.to_string(),
        text: text.clone(),
        params: params,
    }
}

// Takes a valid function signature and returns a function name and a vector of parameters.
pub fn params_from_title(title: &String) -> (String, Vec<(String, String)>) {
    let mut params: Vec<(String, String)> = Vec::new();
    let caps = LUA_FUNC.captures(title).unwrap();

    let params_str = caps.name("params").unwrap().as_str();
    let fname = caps.name("fname").unwrap().as_str();
    if params_str.trim() != "" {
        for p in params_str.split(",") {
            let param_name = p.trim().to_string();

            params.push((p.trim().to_string(), "any".to_string()));
            // Ignore parameters shown in docs after the ...
            if param_name == "..." {
                break;
            }
        }
    }
    (fname.to_string(), clean_parameters(&title, &params))
}

pub fn clean_text(text: String) -> String {
    let t0 = text
        .replace("<code>", "`")
        .replace("</code>", "`")
        .replace("<em>", "*")
        .replace("</em>", "*")
        .replace("<strong>", "**")
        .replace("</strong>", "**")
        .replace("\n", " ")
        .trim()
        .to_string();
    let tn = RE_A.replace_all(&t0, "");
    if HTML_TAG.is_match(&tn) {
        eprintln!("WARN: Extra HTML tag in description text: {}", tn.to_string());
    }
    tn.to_string()
}


fn clean_parameters(title: &String, params: &Vec<(String, String)>) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = Vec::new();
    for (p_name, lua_type) in params {
        if REPLACEMENTS.contains_key(p_name.as_str()) {
            let fixed_name = REPLACEMENTS.get(p_name.as_str()).unwrap().to_string();
            eprintln!("WARN: Fixed invalid parameter: {p_name} -> {fixed_name} (in `{title}`)");
            v.push((fixed_name, lua_type.to_string()));
        } else {
            v.push((p_name.to_string(), lua_type.to_string()));
        }
    }
    v
}

// Initialization functions run by lazy_static

fn load_fixes(fixes_toml: &str) -> HashMap<String, AnchorOverride> {
    let overrides: HashMap<String, AnchorOverride> = match toml::from_str(fixes_toml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: Loading Fixes.toml failed.");
            panic!("ERROR: {:?}", e);
        }
    };
    overrides
}

fn load_replacements(replace_toml: &str) -> HashMap<String, String> {
    let replacements: HashMap<String, String> = match toml::from_str(replace_toml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: Loading Replace.toml failed.");
            panic!("ERROR: {:?}", e);
        }
    };
    replacements
}

fn load_types(types_toml: &str) -> TypesTOML {
    let tt: TypesTOML = match toml::from_str(types_toml) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("ERROR: Loading Types.toml failed.");
            panic!("ERROR: {:?}", e);
        }
    };

    // validate all types in `by_name` are in `types` (catches typos)
    for (type_name, type_type) in &tt.by_name {
        if !tt.types.contains_key(type_type) {
            panic!("Error: Found type name `{}` whose type `{}` is missing from type map Types.toml", type_name, type_type);
        }
    }
    tt
}
