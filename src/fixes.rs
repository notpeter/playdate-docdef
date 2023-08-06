use regex::Regex;
use lazy_static::lazy_static;

use crate::config::{TYPO, INVALID, PARAM_TYPES};
use crate::stub::Stub;

lazy_static! {
    static ref RE_CODE: Regex = Regex::new(r"</?code>").unwrap();
    static ref RE_EM: Regex = Regex::new(r"</?em>").unwrap();
    static ref RE_A: Regex = Regex::new(r"</?a[^>]*>").unwrap();
    static ref RE_STRONG: Regex = Regex::new(r"</?strong>").unwrap();
    static ref HTML_TAG: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref LUA_FUNC: Regex = Regex::new(
        &format!(r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)", id=r"[\w_][\w\d_]*").to_string()
    ).unwrap();
}

// Given a function return a stub with overrides and paramer types applied
pub fn annotate_function(anchor: &str, title: &String, text: &Vec<String>) -> Stub {
    let fname: String;
    let mut params: Vec<(String, String)>;

    // Apply overrides
    if TYPO.contains_key(anchor) {
        let fixed = TYPO.get(anchor).unwrap();
        params = fixed.parameters.iter().map(|p| (p.clone(), "any".to_string())).collect();
        eprintln!("WARN: Found function override: {} -> {}", anchor, fixed);
        fname = fixed.fname.clone();
    } else {
        (fname, params) = params_from_title(title);
    }
    // Apply types from Types.toml
    for (p, t) in params.iter_mut() {
        if PARAM_TYPES.contains_key(p) {
            *t = PARAM_TYPES.get(p).unwrap().to_string();
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
        if INVALID.contains_key(p_name.as_str()) {
            let fixed_name = INVALID.get(p_name.as_str()).unwrap().to_string();
            eprintln!("WARN: Fixed invalid parameter: {p_name} -> {fixed_name} (in `{title}`)");
            v.push((fixed_name, lua_type.to_string()));
        } else {
            v.push((p_name.to_string(), lua_type.to_string()));
        }
    }
    v
}
