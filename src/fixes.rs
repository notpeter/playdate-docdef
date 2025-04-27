use crate::stub::{StubFn, StubVar};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fmt, sync::LazyLock};

#[derive(Deserialize)]
pub struct FunctionReplacement {
    pub name: String,
    pub parameters: Vec<String>,
}

impl fmt::Display for FunctionReplacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.parameters.join(", "))
    }
}

#[derive(Deserialize)]
pub struct VariableReplacement {
    pub name: String,
}

impl fmt::Display for VariableReplacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

static TOML_STR_FUNCTION: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/RenameFn.toml"));
static TOML_STR_PROPERTY: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/RenameVar.toml"));
static TOML_STR_INVALID: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Invalid.toml"));

static RENAME_FUNCTION: LazyLock<HashMap<String, FunctionReplacement>> =
    LazyLock::new(|| toml::from_str(TOML_STR_FUNCTION).expect("Loading RenameFn.toml failed."));
static RENAME_PROPERTY: LazyLock<HashMap<String, VariableReplacement>> =
    LazyLock::new(|| toml::from_str(TOML_STR_PROPERTY).expect("Loading RenameVar.toml failed."));
static INVALID: LazyLock<HashMap<String, String>> =
    LazyLock::new(|| toml::from_str(TOML_STR_INVALID).expect("Loading Invalid.toml failed."));
static RE_A: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"</?a[^>]*>").unwrap());
static HTML_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<[^>]*>").unwrap());
static LUA_FUNC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        // Lua function signatures: function(a,b,c)
        &format!(
            r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)",
            id = r"[\w_][\w\d_]*"
        )
        .to_string(),
    )
    .unwrap()
});

pub fn apply_variable_types(anchor: &str, name: &String, text: &Vec<String>) -> StubVar {
    let title = RENAME_PROPERTY
        .get(anchor)
        .map(|prop| prop.name.clone())
        .unwrap_or_else(|| name.clone());
    eprintln!("apply {title}");
    StubVar {
        anchor: anchor.to_string(),
        title,
        parent: "".to_string(),
        _attrs: Vec::new(),
    }
}

// Given a function return a stub with overrides, parameter types and return types applied
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
