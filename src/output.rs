//! LuaCATS output generation
//!
//! This module generates LuaCATS-format stub files from parsed .luars
//! statements and optionally scraped documentation.

use std::collections::{BTreeMap, HashSet};
use textwrap;

use crate::parser::{Statement, Param, Field};
use crate::doc_scraper::ScrapedFunction;

/// Maximum line length for documentation text (excluding "--- " prefix)
const MAX_LINE_LENGTH: usize = 96;

/// Notes/deprecations loaded from TOML
static NOTES: std::sync::LazyLock<BTreeMap<String, Vec<String>>> = std::sync::LazyLock::new(|| {
    let toml_str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Notes.toml"));
    toml::from_str(toml_str).unwrap_or_default()
});

/// Generate LuaCATS output for a class/table
pub fn generate_class(name: &str, parent: &str, fields: &[Field], prefix: &str) -> Vec<String> {
    let mut out = Vec::new();

    // Class annotation
    if parent.is_empty() {
        out.push(format!("---@class {}", name));
    } else {
        out.push(format!("---@class {} : {}", name, parent));
    }

    // Field annotations
    for field in fields {
        if field.value.is_empty() {
            out.push(format!("---@field {} {}", field.name, field.typ));
        } else {
            out.push(format!("---@field {} {} {}", field.name, field.typ, field.value));
        }
    }

    // Lua assignment
    out.push(format!("{}{} = {{}}", prefix, name));

    out
}

/// Generate LuaCATS output for a function
pub fn generate_function(
    name: &str,
    params: &[Param],
    returns: &[Param],
    docs: Option<&ScrapedFunction>,
) -> Vec<String> {
    let mut out = Vec::new();

    // Apply any notes (deprecations, etc.)
    let param_names: Vec<&str> = params.iter().map(|p| p.name.trim_end_matches('?')).collect();
    let lua_def = format!("{}({})", name, param_names.join(", "));
    if let Some(notes) = NOTES.get(&lua_def) {
        out.extend(notes.clone());
    }

    // Documentation from scraped HTML
    if let Some(func) = docs {
        out.extend(generate_docs(&func.docs, &func.anchor, name));
    }

    // Parameter annotations
    for param in params {
        out.push(format!("---@param {} {}", param.name, param.typ));
    }

    // Return annotations
    for ret in returns {
        if ret.name.is_empty() {
            out.push(format!("---@return {}", ret.typ));
        } else {
            out.push(format!("---@return {} {}", ret.typ, ret.name));
        }
    }

    // Function stub
    out.push(format!("function {} end", lua_def));

    out
}

/// Generate documentation comment lines
fn generate_docs(docs: &[String], anchor: &str, title: &str) -> Vec<String> {
    if anchor.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut in_code = false;

    for (i, line) in docs.iter().enumerate() {
        // Code blocks and bullet lists get fewer line breaks
        let is_list_item = line.starts_with("* ");
        let next_is_list = docs.get(i + 1).map_or(false, |l| l.starts_with("* "));
        let no_break = in_code || line == "```" || (is_list_item && next_is_list);

        if no_break {
            out.push(format!("--- {}", line));
        } else {
            // Wrap long lines
            for wrapped in textwrap::wrap(line, MAX_LINE_LENGTH) {
                out.push(format!("--- {}", wrapped));
            }
            out.push("---".to_string());
        }

        // Track code block state
        if line == "```" {
            in_code = !in_code;
        }
    }

    // Link to official docs
    out.push(format!(
        "--- [Inside Playdate: {}](https://sdk.play.date/Inside%20Playdate.html#{})",
        title, anchor
    ));

    out
}

/// Full stub generator output
pub struct StubOutput {
    lines: Vec<Vec<String>>,
}

impl StubOutput {
    /// Create stub output from parsed statements only (no docs)
    pub fn from_statements(statements: &BTreeMap<String, Statement>) -> Self {
        let mut classes = Vec::new();
        let mut functions = Vec::new();

        for stmt in statements.values() {
            match stmt {
                Statement::Global(name, parent, fields) => {
                    classes.push(generate_class(name, parent, fields, ""));
                }
                Statement::Local(name, parent, fields) => {
                    classes.push(generate_class(name, parent, fields, "local "));
                }
                Statement::Function(name, params, returns) => {
                    functions.push(generate_function(name, params, returns, None));
                }
            }
        }

        // Classes must come before functions
        let mut lines = Vec::new();
        lines.extend(classes);
        lines.extend(functions);

        StubOutput { lines }
    }

    /// Create stub output from statements with scraped documentation
    pub fn from_statements_with_docs(
        statements: &BTreeMap<String, Statement>,
        scraped: &BTreeMap<String, ScrapedFunction>,
    ) -> Self {
        let mut classes = Vec::new();
        let mut functions = Vec::new();
        let mut processed: HashSet<String> = HashSet::new();

        // First, output all classes/tables from statements
        for stmt in statements.values() {
            match stmt {
                Statement::Global(name, parent, fields) => {
                    classes.push(generate_class(name, parent, fields, ""));
                }
                Statement::Local(name, parent, fields) => {
                    classes.push(generate_class(name, parent, fields, "local "));
                }
                _ => {}
            }
        }

        // Process scraped functions (they have docs)
        for func in scraped.values() {
            let key = func.lua_def();
            processed.insert(key.clone());

            // Get types from statements if available
            let (params, returns) = if let Some(Statement::Function(_, p, r)) = statements.get(&key) {
                (p.as_slice(), r.as_slice())
            } else {
                (func.params.as_slice(), func.returns.as_slice())
            };

            functions.push(generate_function(&func.name, params, returns, Some(func)));
        }

        // Add remaining functions from statements (those not in scraped docs)
        for stmt in statements.values() {
            if let Statement::Function(name, params, returns) = stmt {
                let key = stmt.lua_def();
                if !processed.contains(&key) {
                    functions.push(generate_function(name, params, returns, None));
                }
            }
        }

        let mut lines = Vec::new();
        lines.extend(classes);
        lines.extend(functions);

        StubOutput { lines }
    }

    /// Output to stdout
    pub fn print(&self) {
        println!("---@meta");
        println!("--- This file contains function stubs for autocompletion. DO NOT include it in your game.");
        println!();

        for block in &self.lines {
            if !block.is_empty() {
                println!("{}", block.join("\n"));
                println!();
            }
        }

        println!("--- End of LuaCATS stubs.");
    }

    /// Get output as a single string
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("---@meta\n");
        result.push_str("--- This file contains function stubs for autocompletion. DO NOT include it in your game.\n");
        result.push('\n');

        for block in &self.lines {
            if !block.is_empty() {
                result.push_str(&block.join("\n"));
                result.push_str("\n\n");
            }
        }

        result.push_str("--- End of LuaCATS stubs.\n");
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_class() {
        let fields = vec![
            Field { name: "foo".into(), typ: "string".into(), value: "".into() },
            Field { name: "bar".into(), typ: "integer".into(), value: "42".into() },
        ];
        let result = generate_class("MyClass", "Parent", &fields, "");
        assert!(result.contains(&"---@class MyClass : Parent".to_string()));
        assert!(result.contains(&"---@field foo string".to_string()));
        assert!(result.contains(&"---@field bar integer 42".to_string()));
    }

    #[test]
    fn test_generate_function() {
        let params = vec![
            Param { name: "a".into(), typ: "string".into() },
            Param { name: "b?".into(), typ: "integer".into() },
        ];
        let returns = vec![
            Param { name: "".into(), typ: "boolean".into() },
        ];
        let result = generate_function("test.func", &params, &returns, None);
        assert!(result.contains(&"---@param a string".to_string()));
        assert!(result.contains(&"---@param b? integer".to_string()));
        assert!(result.contains(&"---@return boolean".to_string()));
        assert!(result.contains(&"function test.func(a, b) end".to_string()));
    }
}
