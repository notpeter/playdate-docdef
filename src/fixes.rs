use regex::Regex;
use lazy_static::lazy_static;

use crate::config::{TYPO, INVALID, TYPO_C};
use crate::stub::Stub;

#[derive(Clone, Copy, PartialEq)]
pub enum FunctionType {
    Lua,
    C,
}

lazy_static! {
    static ref RE_CODE: Regex = Regex::new(r"</?code>").unwrap();
    static ref RE_EM: Regex = Regex::new(r"</?em>").unwrap();
    static ref RE_A: Regex = Regex::new(r"</?a[^>]*>").unwrap();
    static ref RE_STRONG: Regex = Regex::new(r"</?strong>").unwrap();
    static ref HTML_TAG: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref LUA_FUNC: Regex = Regex::new(// Lua function signatures: function(a,b,c)
        &format!(r"^(?P<fname>(?:{id}\.)*{id}[:\.]{id}|{id})\((?P<params>.*)\)", id=r"[\w_][\w\d_]*").to_string()
    ).unwrap();
    static ref C_FUNC: Regex = Regex::new(// C function signatures: void playdate->system->logToConsole(const char* format, ...)
        &format!(
            r"^(?P<ret_type>(?:{c_type})) (?P<fname>{fname}|{fname_top_level}|{missing_prefix})\({pstr}\);?",
            //todo: remove space from char class once playdate->graphics->video->getContext is fixed
            c_type=r"(?:(?:void|unsigned int|int|enum LuaType|char|const char|float|(?:size_|int8|int16|int32|uint8|uint16|uint32)_t|[A-Z][A-Za-z ]+)\**)",
            fname=r"playdate(?:\->[a-zA-Z_][a-zA-Z_0-9]*)+",
            fname_top_level=r"[a-zA-Z_][a-zA-Z_0-9]*",
            //todo: remove when fixed upstream
            missing_prefix=r"(?:\(\*[a-zA-Z_][a-zA-Z_]*)\)",
            pstr=r"(?P<params>.*)",
        ).to_string()
    ).unwrap();
}

// Given a function return a stub with overrides, parameter types and return types applied
pub fn annotate_function(anchor: &str, title: &String, text: &Vec<String>, f_type: FunctionType) -> Stub {
    let fname: String;
    let params: Vec<(String, String)>;

    // Apply overrides
    if f_type == FunctionType::Lua && TYPO.contains_key(anchor) {
        let fixed = TYPO.get(anchor).unwrap();
        params = fixed.parameters.iter().map(
            |p| (p.clone(), "any".to_string())
        ).collect();
        // eprintln!("WARN: Found function override: {} -> {}", anchor, fixed);
        fname = fixed.fname.clone();
    } else if f_type == FunctionType::C && TYPO_C.contains_key(anchor) {
        let fixed = TYPO_C.get(anchor).unwrap();
        // eprintln!("WARN: Found C_function override: {} -> {}", anchor, fixed);
        fname = fixed.fname.clone();
        params = Vec::new();
    } else {
        (fname, params) = params_from_title(title, f_type);
    }

    let returns: Vec<(String,String)> = Vec::new();

    Stub {
        title: fname,
        anchor: anchor.to_string(),
        text: text.clone(),
        params,
        returns,
    }
}

pub fn c_sig_from_title(title: &String) -> (String, String, Vec<(String, String)>) {
    let mut params: Vec<(String, String)> = Vec::new();
    let caps = match C_FUNC.captures(title) {
        Some(c) => c, None => { panic!("ERROR: Could not parse C function signature: \n{}\nusing this regex: {}\n", title, C_FUNC.as_str()) }
    };
    let ret_type = caps.name("ret_type").unwrap().as_str();
    let fname = caps.name("fname").unwrap().as_str();
    let params_str = caps.name("params").unwrap().as_str().trim();
    if params_str == "void" {
        params.push(("void".to_string(), "void".to_string()));
    } else if params_str != "" {
        for p in params_str.split(",") {
            let (p_name, p_type) = p.rsplit_once(" ").unwrap();
            params.push((p_name.trim().to_string(), p_type.trim().to_string()));
        }
    }
    (ret_type.to_string(), fname.to_string(), clean_c_parameters(&params))
}

pub fn recreate_c_sig(ret_type: &String, fname: &String, params: &Vec<(String, String)>) -> String {
    let mut sig = format!("{} {}(", ret_type, fname);
    if params.len() == 1 && params[0] == ("void".to_string(), "void".to_string()) {
        sig.push_str("void");
    } else {
        sig.push_str(
            &params.iter().map(
                |(p_name, p_type)|
                format!("{} {}", p_name, p_type)
            ).collect::<Vec<String>>().join(", ")
        );
    }
    sig.push_str(")");
    sig
}

pub fn clean_c_parameters(params: &Vec<(String, String)>) -> Vec<(String, String)> {
    params.clone()
}

// Takes a valid function signature and returns a function name and a vector of parameters.
pub fn params_from_title(title: &String, f_type: FunctionType) -> (String, Vec<(String, String)>) {
    let mut params: Vec<(String, String)> = Vec::new();
    let caps: regex::Captures;


    let caps = match f_type {
        FunctionType::Lua => match LUA_FUNC.captures(title) {
            Some(c) => c,
            None => {
                panic!("ERROR: Could not parse Lua function signature:\n{}\nwith: {}\n", title, LUA_FUNC.as_str());
            }
        },
        FunctionType::C => match C_FUNC.captures(title) {
            Some(c) => c,
            None => {
                panic!("ERROR: Could not parse C function signature:\n{}\nwith: {}\n", title, C_FUNC.as_str());
            }
        },
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
                param_name = param_name.clone().replace("[", "").replace("]", "").trim().to_string();
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
        .trim()
        .to_string();
    let tn = RE_A.replace_all(&t0, "");
    if HTML_TAG.is_match(&tn) && !&tn.contains("/Data/<bundleid>"){
        eprintln!("WARN: Extra HTML tag in description text: {}", tn.to_string());
    }
    tn.to_string()
}

pub fn clean_code(text: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.trim() != "" { // remove empty lines in middle of code blocks
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
            // eprintln!("WARN: Fixed invalid parameter: {p_an} -> {fixed_name} (in `{title}`)");
            v.push((fixed_name, lua_type.to_string()));
        } else {
            v.push((p_name.to_string(), lua_type.to_string()));
        }
    }
    v
}


#[cfg(test)]
mod tests {
    // use std::fs;
    use super::*;
    #[test]
    fn lua_func() {
        let title = "playdate.pathfinder.graph.new([nodeCount, [coordinates]])";
        let caps = LUA_FUNC.captures(title).unwrap();
        let params_str = caps.name("params").unwrap().as_str();
        let fname = caps.name("fname").unwrap().as_str();
        assert_eq!(fname, "playdate.pathfinder.graph.new");
        assert_eq!(params_str, "[nodeCount, [coordinates]]");
    }

    #[test]
    fn test_params_from_title() {
        let title = "playdate.graphics.image:draw(x, y, [flip, [sourceRect]])";
        let (fname, params) = params_from_title(&title.to_string(), FunctionType::Lua);
        assert_eq!(fname, "playdate.graphics.image:draw");
        assert_eq!(params, vec![
            ("x".to_string(), "any".to_string()),
            ("y".to_string(), "any".to_string()),
            ("flip?".to_string(), "any".to_string()),
            ("sourceRect?".to_string(), "any".to_string()),
        ]);
    }

    #[test]
    fn test_c_function_simple() {
        let title = "void playdate->system->removeAllMenuItems()";
        let caps = C_FUNC.captures(title).unwrap();

        let ret_type = caps.name("ret_type").unwrap().as_str();
        let fname = caps.name("fname").unwrap().as_str();
        let params_str = caps.name("params").unwrap().as_str();
        assert_eq!(ret_type, "void");
        assert_eq!(fname, "playdate->system->removeAllMenuItems");
        assert_eq!(params_str, "");

        let empty_vec: Vec<(String, String)> = Vec::new();
        let (ret_type, fname, params) = c_sig_from_title(&title.to_string());
        assert_eq!(
            (ret_type.as_str(), fname.as_str(), &params),
            ("void", "playdate->system->removeAllMenuItems", &empty_vec)
        );
        assert_eq!(title, recreate_c_sig(&ret_type, &fname, &params));

        let t2 = "void* playdate->system->realloc(void* ptr, size_t size)";
        let (ret_type, fname, params) = c_sig_from_title(&t2.to_string());
        assert_eq!(
            (ret_type.as_str(), fname.as_str(), &params),
            ("void*", "playdate->system->realloc", &vec![
                ("void*".to_string(), "ptr".to_string()),
                ("size_t".to_string(), "size".to_string()),
            ])
        );
    }

    #[test]
    fn test_c_function_complex() {
        let title = "PDMenuItem* playdate->system->addOptionsMenuItem(const char* title, const char** options, int optionsCount, PDMenuItemCallbackFunction* callback, void* userdata)";
        let (ret_type, fname, params) = c_sig_from_title(&title.to_string());
        assert_eq!(ret_type, "PDMenuItem*");
        assert_eq!(fname, "playdate->system->addOptionsMenuItem");
        assert_eq!(
            params,
            vec![
                ("const char*".to_string(), "title".to_string()),
                ("const char**".to_string(), "options".to_string()),
                ("int".to_string(), "optionsCount".to_string()),
                ("PDMenuItemCallbackFunction*".to_string(), "callback".to_string()),
                ("void*".to_string(), "userdata".to_string()),
            ]
        );
        assert_eq!(title, recreate_c_sig(&ret_type, &fname, &params));
    }

    #[test]
    fn test_c_struct_func() {
        let title = "void (*freeSignal)(PDSynthSignal* signal);";
        let (ret_type, fname, params) = c_sig_from_title(&title.to_string());
        assert_eq!(ret_type, "void");
        assert_eq!(fname, "(*freeSignal)");
        assert_eq!(
            params,
            vec![
                ("PDSynthSignal*".to_string(), "signal".to_string()),
            ]
        );
    }
}
