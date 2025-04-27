use crate::fixes::apply_notes;
use crate::luars::LuarsStatement;
use std::collections::BTreeMap;
use textwrap;

// Wrap long lines of documentation at this length
// Note: Function signatures are not wrapped (a couple are >100 chars)
// Note: This also includes the leading "--- " (4 chars).
static MAX_LINE_LENGTH: usize = 100 - 4;

pub enum Stub {
    Function(StubFn),
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
        if let Some(statement) = statements.get(&our_lua) {
            match statement {
                LuarsStatement::Function(_name, params, returns) => {
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
                _ => eprintln!("eek, found non-function for {our_lua}"),
            }
        } else {
            eprintln!("WARN: Function {our_lua} not untyped {}", self.anchor);
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

    fn generate_description(&self) -> Vec<String> {
        if self.anchor == "" {
            Vec::new()
        } else {
            let mut lines = Vec::new();
            let mut i = 0;
            let mut in_code = false;
            while i < self.text.len() {
                let line = &self.text[i];
                // Bulleted list and code get fewer newlines.
                // Everything else needs extra empty lines for proper markdown rendering.
                let no_break = in_code
                    || line.starts_with("```")
                    || (line.starts_with("* ")
                        && i < self.text.len() - 1
                        && self.text[i + 1].starts_with("* "));
                if no_break {
                    lines.push(format!("--- {}", line));
                } else {
                    for wrapped_line in textwrap::wrap(line.as_str(), MAX_LINE_LENGTH) {
                        lines.push(format!("--- {}", wrapped_line));
                    }
                    lines.push("---".to_string());
                }
                // this is hacky as hell
                if line == "```" {
                    in_code = !in_code;
                }
                i = i + 1;
            }
            lines.push(format!(
                "--- [Inside Playdate: {}](https://sdk.play.date/Inside%20Playdate.html#{})",
                self.title, self.anchor
            ));
            lines
        }
    }

    /// Generate complete LuaCATS for function
    pub fn get_luacats(&self) -> Vec<String> {
        let mut out = Vec::new();
        out.extend(apply_notes(&self.lua_def()));
        out.extend(self.generate_description());
        out.extend(self.luacats_params());
        out.extend(self.luacats_returns());
        out.push(self.lua_statement());
        out
    }

    /// Generatte '---@param name type' for each parameter to the function
    fn luacats_params(&self) -> Vec<String> {
        self.params
            .iter()
            .map(|(name, _type)| format!("---@param {} {}", name, _type))
            .collect::<Vec<String>>()
    }

    /// Generate '---@return type [name]' for the function (multiple lines for multival returns)
    fn luacats_returns(&self) -> Vec<String> {
        self.returns
            .iter()
            .map(|(_name, _type)| {
                if _name.to_string() == "" {
                    format!("---@return {}", _type)
                } else {
                    format!("---@return {_type} {_name}", _type = _type, _name = _name)
                }
            })
            .collect::<Vec<String>>()
    }

    /// Return a valid lua statement for the function.
    fn lua_statement(&self) -> String {
        format!("function {} end", self.lua_def())
    }
}

pub struct TableContents {
    pub name: String,
    pub r#type: String,
    pub value: String, // Some Enums have documented values
}

impl TableContents {
    pub fn to_string(&self) -> String {
        let mut line = Vec::new();
        line.push("---@field");
        line.push(&self.name);
        line.push(&self.r#type);
        if !self.value.is_empty() {
            line.push(&self.value);
        }
        line.join(" ")
    }
}

/// Global and Local Variables
pub struct Table {
    pub prefix: String,
    pub name: String,
    pub r#type: String,
    pub contents: Vec<TableContents>,
}

impl Table {
    pub fn get_luacats(&self) -> Vec<String> {
        let mut out = Vec::new();
        out.push(self.luacats_class());
        out.extend(self.luacats_fields());
        out.push(self.lua_statement());
        out
    }

    /// Return a valid lua statement for the class.
    pub fn lua_statement(&self) -> String {
        format!("{}{} = {{}}", self.prefix, self.name)
    }
    /// Returns '---@field name type [value]' for class/instance attributes
    pub fn luacats_fields(&self) -> Vec<String> {
        self.contents
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
    }

    /// Returns '---@class name : parent' for classes
    pub fn luacats_class(&self) -> String {
        if self.r#type.to_string() == "" {
            format!("---@class {}", self.name)
        } else {
            format!("---@class {} : {}", self.name, self.r#type)
        }
    }
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
        assert_eq!(stub.lua_statement(), "function test_func(param1) end");
    }
}
