use std::collections::BTreeMap;

use crate::luars::LuarsStatement;
use textwrap;

// Wrap long lines of documentation at this length
// Note: Function signatures are not wrapped (a couple are >100 chars)
// Note: This also includes the leading "--- " (4 chars).
static MAX_LINE_LENGTH: usize = 100 - 4;

pub enum Stub {
    Variable(StubVar),
    Function(StubFn),
}

#[derive(Debug, Clone)]
pub struct StubAttr {
    pub name: String,
    pub anchor: String,
    pub _type: String,
    pub value: String,
    pub text: String,
}

impl StubAttr {
    pub fn to_stub(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct StubVar {
    pub title: String,
    pub anchor: String,
    pub parent: String,
    pub attrs: Vec<StubAttr>,
    pub _text: Vec<String>,
}

impl StubVar {
    pub fn to_stub(&self) -> String {
        format!("local {}", self.title)
    }
    pub fn annotate(mut self, statements: &BTreeMap<String, LuarsStatement>) -> Self {
        let our_lua = self.lua_def();
        if let Some(statement) = statements.get(&our_lua) {
            match statement {
                LuarsStatement::Global(_name, parent, attrs) => {
                    self.parent = parent.to_string();
                    self.attrs = attrs
                        .iter()
                        .map(|(aname, atype, avalue)| StubAttr {
                            name: aname.to_string(),
                            anchor: String::new(),
                            _type: atype.to_string(),
                            value: avalue.to_string(),
                            text: String::new(),
                        })
                        .collect();
                }
                LuarsStatement::Function(_, _, _) => {
                    eprintln!("eek, found function not global for {our_lua}")
                }
                LuarsStatement::Local(_, _, _) => {
                    eprintln!("eek, found local not global for {our_lua}")
                }
            }
        }
        // Docs have variable we haven't typed in playdate.luars yet
        else {
            eprintln!("WARN: Function {our_lua} not found/untyped {}", self.anchor);
        }
        self
    }
    /// Lua function signature (no types)
    pub fn lua_def(&self) -> String {
        self.title.clone()
    }
}

// Stub Struct containing extracted signature, url anchor, list of parameters and description text
#[derive(Debug, Clone)]
pub struct StubFn {
    pub title: String,
    pub anchor: String,
    /// Function Parameters (name, type)
    pub params: Vec<(String, String)>,
    /// Return (name, type)
    pub returns: Vec<(String, String)>,
    pub text: Vec<String>,
}

impl StubFn {
    pub fn annotate(mut self, statements: &BTreeMap<String, LuarsStatement>) -> Self {
        let our_lua = self.lua_def();
        // TODO: This is hella inefficient (N^2; where N=1000+)
        if let Some(statement) = statements.get(&our_lua) {
            match statement {
                LuarsStatement::Function(_name, params, returns) => {
                    // TODO: Calculating .lua every time is also inefficient
                    if our_lua == statement.lua_def() {
                        self.params = params
                            .iter()
                            .map(|(fname, ftype)| (fname.to_string(), ftype.to_string()))
                            .collect();
                        self.returns = returns
                            .iter()
                            .map(|(fname, ftype)| (fname.to_string(), ftype.to_string()))
                            .collect();
                    }
                }
                LuarsStatement::Global(_, _, _) => {
                    eprintln!("eek, found global not function for {our_lua}")
                }
                LuarsStatement::Local(_, _, _) => {
                    eprintln!("eek, found loca not function for {our_lua}")
                }
            }
        } else {
            eprintln!("WARN: Function {our_lua} not found/untyped {}", self.anchor);
        }
        self
    }

    /// Lua function signature (no types)
    pub fn lua_def(&self) -> String {
        let name = self.title.clone();
        let params = self.params.clone();
        let param_names: Vec<String> = params
            .iter()
            .map(|(name, _)| name.clone().replace("?", ""))
            .collect::<Vec<String>>();
        format!("{}({})", name, param_names.join(", "))
    }

    pub fn text_comments(&self) -> Vec<String> {
        if self.anchor == "" {
            Vec::new()
        } else {
            self.text2comments()
        }
    }

    pub fn to_stub(&self) -> String {
        format!("function {} end", self.lua_def())
    }

    fn text2comments(&self) -> Vec<String> {
        text_to_comments(&self.text, &self.title, &self.anchor)
    }
}

pub fn text_to_comments(text: &[String], title: &str, anchor: &str) -> Vec<String> {
    let mut s = Vec::new();
    let mut i = 0;
    let mut in_code = false;
    while i < text.len() {
        let line = &text[i];
        // Bulleted list and code get fewer newlines.
        // Everything else needs extra empty lines for proper markdown rendering.
        let no_break = in_code
            || line.starts_with("```")
            || (line.starts_with("* ") && i < text.len() - 1 && text[i + 1].starts_with("* "));
        if no_break {
            s.push(format!("--- {}", line));
        } else {
            for wrapped_line in textwrap::wrap(line.as_str(), MAX_LINE_LENGTH) {
                s.push(format!("--- {}", wrapped_line));
            }
            s.push("---".to_string());
        }
        // this is hacky as hell
        if line == "```" {
            in_code = !in_code;
        }
        i = i + 1;
    }
    s.push(format!(
        "--- [Inside Playdate: {}](https://sdk.play.date/Inside%20Playdate.html#{})",
        title, anchor
    ));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stub() {
        let stub = StubFn {
            title: "test_func".to_string(),
            anchor: "test_anchor".to_string(),
            params: vec![("param1".to_string(), "string".to_string())],
            returns: vec![("ret1".to_string(), "number".to_string())],
            text: vec!["Test description".to_string()],
        };

        assert_eq!(stub.lua_def(), "test_func(param1)");
        assert_eq!(stub.to_stub(), "function test_func(param1) end");
    }
}
