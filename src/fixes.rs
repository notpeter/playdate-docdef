use crate::stub::StubFn;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

#[derive(Deserialize)]
pub struct FunctionReplacement {
    pub name: String,
    pub parameters: Vec<String>,
}

// TOML Files
static TOML_STR_NOTES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Notes.toml"));
static TOML_STR_FUNCTION: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/RenameFn.toml"));
static TOML_STR_INVALID: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Invalid.toml"));

// Static HashMaps from TOML files
static NOTES: LazyLock<HashMap<String, Vec<String>>> =
    LazyLock::new(|| toml::from_str(TOML_STR_NOTES).expect("Loading Notes.toml failed."));
static RENAME_FUNCTION: LazyLock<HashMap<String, FunctionReplacement>> =
    LazyLock::new(|| toml::from_str(TOML_STR_FUNCTION).expect("Loading RenameFn.toml failed."));
static INVALID: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| toml::from_str(TOML_STR_INVALID).expect("Loading Invalid.toml failed."));

/// HTML Link tags
static HTML_A: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"</?a[^>]*>").unwrap());
static HTML_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").unwrap());
/// Lua function signature: 'function(a,b,c)'
static LUA_FUNC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        &format!(
            r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)",
            id = r"[\w_][\w\d_]*"
        )
        .to_string(),
    )
    .unwrap()
});

/// Given an anchor, return an notes from Notes.toml (hard coded stuff)
pub fn apply_notes(name: &String) -> Vec<String> {
    if let Some(note) = NOTES.get(name.as_str()) {
        note.clone()
    } else {
        Vec::new()
    }
}

/// Given a function return a stub with overrides, parameter types and return types applied
pub fn apply_fn_types(anchor: &str, title: &String, text: &Vec<String>) -> StubFn {
    let name: String;
    let params: Vec<(String, String)>;

    // Apply overrides
    if let Some(fixed) = RENAME_FUNCTION.get(anchor) {
        params = fixed
            .parameters
            .iter()
            .map(|p| (p.clone(), "any".to_string()))
            .collect();
        // eprintln!("WARN: Found function override: {} -> {}", anchor, fixed);
        name = fixed.name.clone();
    } else {
        (name, params) = params_from_title(title);
    }

    let returns: Vec<(String, String)> = Vec::new();

    StubFn {
        title: name,
        anchor: anchor.to_string(),
        text: text.clone(),
        params,
        returns,
    }
}

// Takes a valid function signature and returns a function name and a vector of parameters.
fn params_from_title(title: &String) -> (String, Vec<(String, String)>) {
    let mut params: Vec<(String, String)> = Vec::new();
    let lua_func = LUA_FUNC
        .captures(title)
        .expect(format!("ERROR: Could not parse function signature (typo?): {title}").as_str());

    let params_str = lua_func.name("params").unwrap().as_str();
    let fname = lua_func.name("fname").unwrap().as_str();
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
    let tn = HTML_A.replace_all(&t0, "");
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
    let mut out: Vec<(String, String)> = Vec::new();
    for (p_name, lua_type) in params {
        let p_an = p_name.replace("?", ""); // without "?" at the the end for optional
        if let Some(fixed_name) = INVALID.get(&p_an) {
            // eprintln!("WARN: Fixed invalid parameter: {p_an} -> {fixed_name}");
            out.push((fixed_name.clone(), lua_type.to_string()));
        } else {
            out.push((p_name.to_string(), lua_type.to_string()));
        }
    }
    out
}
