use std::collections::HashMap;
use regex::Regex;

use lazy_static::lazy_static;

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

    static ref CLEAN: HashMap<&'static str,&'static str> = HashMap::from([
        ("function", "func"),
        ("repeat", "_repeat"),
        ("end", "_end"),
        ("path.mid", "midi_path"),
        ("…", "..."), // TODO: This doesn't work
    ]);
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
        if CLEAN.contains_key(p.as_str()) {
            eprintln!("WARN: Fixed invalid parameter name (reserved word): `{title}`");
            v.push(CLEAN.get(p.as_str()).unwrap_or(&p.as_str()).to_string());
        } else if p.contains("-") {
            eprintln!("WARN: Fixed invalid parameter name (hyphen / dash): `{title}`");
            v.push(p.replace("-", "_"));
        } else {
            v.push(p.to_string());
        }
    }
    v
}
