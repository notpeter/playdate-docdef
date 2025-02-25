use crate::stub::Stub;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fmt};

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

static TOML_STR_TYPO: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Typo.toml"));
static TOML_STR_INVALID: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Invalid.toml"));

lazy_static! {
    static ref TYPO: HashMap<String, TypoReplacement> = match toml::from_str(TOML_STR_TYPO) {
        Ok(v) => v,
        Err(e) => { panic!("ERROR: Loading Typo.toml failed. {:?}", e) }
    };
    static ref INVALID: HashMap<String, String> = match toml::from_str(TOML_STR_INVALID) {
        Ok(v) => v,
        Err(e) => { panic!("ERROR: Loading Invalid.toml failed. {:?}", e); }
    };
    static ref RE_CODE: Regex = Regex::new(r"</?code>").unwrap();
    static ref RE_EM: Regex = Regex::new(r"</?em>").unwrap();
    static ref RE_A: Regex = Regex::new(r"</?a[^>]*>").unwrap();
    static ref RE_STRONG: Regex = Regex::new(r"</?strong>").unwrap();
    static ref HTML_TAG: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref LUA_FUNC: Regex = Regex::new(// Lua function signatures: function(a,b,c)
        &format!(r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)", id=r"[\w_][\w\d_]*").to_string()
    ).unwrap();
}

// Given a function return a stub with overrides, parameter types and return types applied
pub fn annotate_function(anchor: &str, title: &String, text: &Vec<String>) -> Stub {
    let fname: String;
    let params: Vec<(String, String)>;

    // Apply overrides
    if TYPO.contains_key(anchor) {
        let fixed = TYPO.get(anchor).unwrap();
        params = fixed
            .parameters
            .iter()
            .map(|p| (p.clone(), "any".to_string()))
            .collect();
        // eprintln!("WARN: Found function override: {} -> {}", anchor, fixed);
        fname = fixed.fname.clone();
    } else {
        (fname, params) = params_from_title(title);
    }

    let returns: Vec<(String, String)> = Vec::new();

    Stub {
        title: fname,
        anchor: anchor.to_string(),
        text: text.clone(),
        params,
        returns,
    }
}

// Takes a valid function signature and returns a function name and a vector of parameters.
pub fn params_from_title(title: &String) -> (String, Vec<(String, String)>) {
    let mut params: Vec<(String, String)> = Vec::new();
    let caps = match LUA_FUNC.captures(title) {
        Some(c) => c,
        None => {
            panic!(
                "ERROR: Could not parse function signature (typo?): {}",
                title
            );
        }
    };

    let params_str = caps.name("params").unwrap().as_str();
    let fname = caps.name("fname").unwrap().as_str();
    let mut optional = false;
    if params_str.trim() != "" {
        for p in params_str.split(",") {
            let mut param_name = p.trim().to_string();
            // once with hit an open bracket every param afterwards is optional
            // TODO: technically there's like one func(a, [b], callback) but let's ignore that.
            if param_name.contains("[") {
                optional = true;
            }
            if optional {
                // strip the brackets
                param_name = param_name
                    .clone()
                    .replace("[", "")
                    .replace("]", "")
                    .trim()
                    .to_string();
                // add the optional ? to param name
                param_name = format!("{}?", param_name);
            }
            params.push((param_name.clone(), "any".to_string()));
            // Ignore parameters shown in docs after the ... like: fun(a1, a2, ..., aN)
            if param_name == "..." || param_name == "...?" {
                break;
            }
        }
    }
    (fname.to_string(), clean_parameters(&params))
}

pub fn clean_text(text: String) -> String {
    let t0 = text
        .replace("<code>", "`")
        .replace("</code>", "`")
        .replace("<pre>", "`")
        .replace("</pre>", "`")
        .replace("<em>", "*")
        .replace("</em>", "*")
        .replace("<strong>", "**")
        .replace("</strong>", "**")
        .replace("\n", " ")
        .replace("<br>", "\n---\n---")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("<bundleid>", "{bundleid}")
        .replace("<text>", "{text}")
        .replace("<message>", "{message}")
        .trim()
        .to_string();
    let tn = RE_A.replace_all(&t0, "");
    // The restuling Markdown should not have HTML tags.
    if HTML_TAG.is_match(&tn) {
        eprintln!(
            "WARN: Extra HTML tag in description text: {}",
            tn.to_string()
        );
    }
    tn.to_string()
}

pub fn clean_code(text: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.trim() != "" {
            // remove empty lines in middle of code blocks
            lines.push(line.to_string());
        }
    }
    lines
}

fn clean_parameters(params: &Vec<(String, String)>) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = Vec::new();
    for (p_name, lua_type) in params {
        let p_an = p_name.replace("?", ""); // without "?" at the the end for optional
        if INVALID.contains_key(p_an.as_str()) {
            let fixed_name = INVALID.get(p_an.as_str()).unwrap().to_string();
            // eprintln!("WARN: Fixed invalid parameter: {p_an} -> {fixed_name}");
            v.push((fixed_name, lua_type.to_string()));
        } else {
            v.push((p_name.to_string(), lua_type.to_string()));
        }
    }
    v
}
