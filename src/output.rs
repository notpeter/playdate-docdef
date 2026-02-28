//! LuaCATS output generation
//!
//! This module generates LuaCATS-format stub files from parsed .luars
//! statements and optionally scraped documentation.

use std::collections::{BTreeMap, HashSet};
use textwrap;

use crate::luars::{Field, Param, Statement};
use crate::scraper::ScrapedFunction;

/// Maximum line length for documentation text (excluding "--- " prefix)
const MAX_LINE_LENGTH: usize = 96;

/// Notes/deprecations loaded from TOML
static NOTES: std::sync::LazyLock<BTreeMap<String, Vec<String>>> = std::sync::LazyLock::new(|| {
    let toml_str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Notes.toml"));
    toml::from_str(toml_str).unwrap_or_default()
});

pub(crate) fn format_stub(lines: &[Vec<String>]) -> String {
    let mut out = Vec::new();
    out.push("---@meta".to_string());
    out.push(
        "--- This file contains function stubs for autocompletion. DO NOT include it in your game."
            .to_string(),
    );
    out.push(String::new());

    for block in lines {
        if !block.is_empty() {
            out.push(block.join("\n"));
            out.push(String::new());
        }
    }

    out.push("--- End of LuaCATS stubs.".to_string());
    out.push(String::new());
    out.join("\n")
}

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
            out.push(format!(
                "---@field {} {} {}",
                field.name, field.typ, field.value
            ));
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
    let param_names: Vec<&str> = params
        .iter()
        .map(|p| p.name.trim_end_matches('?'))
        .collect();
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

pub(crate) fn split_function_name(name: &str) -> (&str, &str) {
    if let Some(pos) = name.rfind(':') {
        (&name[..pos], &name[pos + 1..])
    } else if let Some(pos) = name.rfind('.') {
        (&name[..pos], &name[pos + 1..])
    } else {
        ("", name)
    }
}

pub(crate) fn function_type(params: &[Param], returns: &[Param]) -> String {
    let params_str = params
        .iter()
        .map(|p| format!("{}: {}", p.name.trim_end_matches('?'), p.typ))
        .collect::<Vec<_>>()
        .join(", ");
    let returns_str = if returns.is_empty() {
        "nil".to_string()
    } else if returns.len() == 1 {
        returns[0].typ.clone()
    } else {
        let types = returns
            .iter()
            .map(|r| r.typ.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", types)
    };
    format!("fun({}): {}", params_str, returns_str)
}

pub(crate) fn append_compact_types(
    entries: &mut Vec<(String, Vec<String>)>,
    field: &str,
    typ: String,
) {
    if let Some((_, types)) = entries.iter_mut().find(|(name, _)| name == field) {
        types.push(typ);
    } else {
        entries.push((field.to_string(), vec![typ]));
    }
}

pub(crate) fn generate_compact_global(
    name: &str,
    parent: &str,
    fields: &[Field],
    functions: &[(String, Vec<String>)],
) -> Vec<String> {
    let mut out = Vec::new();

    if parent.is_empty() {
        out.push(format!("---@class {}", name));
    } else {
        out.push(format!("---@class {} : {}", name, parent));
    }

    for field in fields {
        if field.value.is_empty() {
            out.push(format!("---@field {} {}", field.name, field.typ));
        } else {
            out.push(format!(
                "---@field {} {} {}",
                field.name, field.typ, field.value
            ));
        }
    }

    let is_playdate = name == "playdate";
    if functions.is_empty() {
        if is_playdate {
            out.push(format!("{} = {} or {{}}", name, name));
        } else {
            out.push(format!("{} = {{}}", name));
        }
        return out;
    }

    if is_playdate {
        out.push(format!("{} = {} or {{", name, name));
    } else {
        out.push(format!("{} = {{", name));
    }
    for (field, types) in functions {
        for typ in types {
            out.push(format!("    ---@type {}", typ));
        }
        out.push(format!("    {} = nil,", field));
    }
    out.push("}".to_string());

    out
}

/// Check if a line is a list item (including nested/indented lists)
fn is_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("* ")
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
        let this_is_list = is_list_item(line);
        let next_is_list = docs.get(i + 1).map_or(false, |l| is_list_item(l));
        let no_break = in_code || line.starts_with("```") || (this_is_list && next_is_list);

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
        if line.starts_with("```") {
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
    compact: bool,
    compact_no_indent: bool,
}

impl StubOutput {
    /// Create stub output from parsed statements only (no docs)
    pub fn from_statements(
        statements: &BTreeMap<String, Statement>,
        compact: bool,
        compact_no_indent: bool,
    ) -> Self {
        let mut classes = Vec::new();
        let mut functions = Vec::new();

        if compact {
            let mut globals = HashSet::new();
            let mut functions_by_ns: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();
            let mut top_level: Vec<(String, Vec<String>)> = Vec::new();

            for stmt in statements.values() {
                if let Statement::Global(name, _, _) = stmt {
                    globals.insert(name.clone());
                }
            }

            for stmt in statements.values() {
                if let Statement::Function(name, params, returns) = stmt {
                    let (ns, field) = split_function_name(name);
                    if ns.is_empty() || !globals.contains(ns) {
                        let typ = function_type(params, returns);
                        append_compact_types(&mut top_level, name, typ);
                    } else {
                        let typ = function_type(params, returns);
                        let entry = functions_by_ns.entry(ns.to_string()).or_default();
                        append_compact_types(entry, field, typ);
                    }
                }
            }

            for stmt in statements.values() {
                match stmt {
                    Statement::Global(name, parent, fields) => {
                        let funcs = functions_by_ns
                            .get(name)
                            .map(|v| v.as_slice())
                            .unwrap_or(&[]);
                        classes.push(generate_compact_global(name, parent, fields, funcs));
                    }
                    Statement::Local(name, parent, fields) => {
                        classes.push(generate_class(name, parent, fields, "local "));
                    }
                    Statement::Function(_, _, _) => {}
                }
            }
            for (name, types) in top_level {
                let mut block = Vec::new();
                for typ in types {
                    block.push(format!("---@type {}", typ));
                }
                block.push(format!("{} = nil,", name));
                functions.push(block);
            }
        } else {
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
        }

        // Classes must come before functions
        let mut lines = Vec::new();
        lines.extend(classes);
        lines.extend(functions);

        StubOutput {
            lines,
            compact,
            compact_no_indent,
        }
    }

    /// Create stub output from statements with scraped documentation
    pub fn from_statements_with_docs(
        statements: &BTreeMap<String, Statement>,
        scraped: &BTreeMap<String, ScrapedFunction>,
        compact: bool,
        compact_no_indent: bool,
    ) -> Self {
        let mut classes = Vec::new();
        let mut functions = Vec::new();
        let mut processed: HashSet<String> = HashSet::new();

        if compact {
            let mut globals = HashSet::new();
            let mut functions_by_ns: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();
            let mut combined_functions = Vec::new();
            let mut top_level: Vec<(String, Vec<String>)> = Vec::new();

            for stmt in statements.values() {
                if let Statement::Global(name, _, _) = stmt {
                    globals.insert(name.clone());
                }
            }

            for func in scraped.values() {
                let key = func.lua_def();
                processed.insert(key.clone());

                let (params, returns) =
                    if let Some(Statement::Function(_, p, r)) = statements.get(&key) {
                        (p.clone(), r.clone())
                    } else {
                        (func.params.clone(), func.returns.clone())
                    };
                combined_functions.push((func.name.clone(), params, returns));
            }

            for stmt in statements.values() {
                if let Statement::Function(name, params, returns) = stmt {
                    let key = stmt.lua_def();
                    if !processed.contains(&key) {
                        combined_functions.push((name.clone(), params.clone(), returns.clone()));
                    }
                }
            }

            for (name, params, returns) in combined_functions {
                let (ns, field) = split_function_name(&name);
                if ns.is_empty() || !globals.contains(ns) {
                    let typ = function_type(&params, &returns);
                    append_compact_types(&mut top_level, &name, typ);
                } else {
                    let typ = function_type(&params, &returns);
                    let entry = functions_by_ns.entry(ns.to_string()).or_default();
                    append_compact_types(entry, field, typ);
                }
            }

            for stmt in statements.values() {
                match stmt {
                    Statement::Global(name, parent, fields) => {
                        let funcs = functions_by_ns
                            .get(name)
                            .map(|v| v.as_slice())
                            .unwrap_or(&[]);
                        classes.push(generate_compact_global(name, parent, fields, funcs));
                    }
                    Statement::Local(name, parent, fields) => {
                        classes.push(generate_class(name, parent, fields, "local "));
                    }
                    _ => {}
                }
            }
            for (name, types) in top_level {
                let mut block = Vec::new();
                for typ in types {
                    block.push(format!("---@type {}", typ));
                }
                block.push(format!("{} = nil,", name));
                functions.push(block);
            }
        } else {
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
                let (params, returns) =
                    if let Some(Statement::Function(_, p, r)) = statements.get(&key) {
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
        }

        let mut lines = Vec::new();
        lines.extend(classes);
        lines.extend(functions);

        StubOutput {
            lines,
            compact,
            compact_no_indent,
        }
    }

    /// Output to stdout
    pub fn print(&self) {
        println!("---@meta");
        if self.compact && self.compact_no_indent {
            println!();
        }
        if !self.compact {
            println!(
                "--- This file contains function stubs for autocompletion. DO NOT include it in your game."
            );
            println!();
        }

        for block in &self.lines {
            if !block.is_empty() {
                let rendered = render_block(block, self.compact, self.compact_no_indent);
                println!("{}", rendered);
                println!();
            }
        }

        if !self.compact {
            println!("--- End of LuaCATS stubs.");
        }
    }

    /// Get output as a single string
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        if self.compact {
            let mut out = Vec::new();
            out.push("---@meta".to_string());
            out.push(String::new());
            out.push(String::new());
            for block in &self.lines {
                if !block.is_empty() {
                    let rendered = render_block(block, self.compact, self.compact_no_indent);
                    out.push(rendered);
                    out.push(String::new());
                }
            }
            out.join("\n")
        } else {
            format_stub(&self.lines)
        }
    }
}

fn render_block(block: &[String], compact: bool, compact_no_indent: bool) -> String {
    if !(compact && compact_no_indent) {
        return block.join("\n");
    }

    let lines = block
        .iter()
        .map(|line| line.strip_prefix("    ").unwrap_or(line).to_string())
        .collect::<Vec<_>>();

    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if line.starts_with("---@type ") {
            let next_is_assignment = i + 1 < lines.len() && lines[i + 1].ends_with(" = nil,");
            let prev_is_type = i > 0 && lines[i - 1].starts_with("---@type ");
            if next_is_assignment && !prev_is_type {
                let assignment = &lines[i + 1];
                let typ = line.trim_start_matches("---@type ").trim();
                out.push(format!("{} ---@type {}", assignment, typ));
                i += 2;
                continue;
            }
        }
        out.push(line.clone());
        i += 1;
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_class() {
        let fields = vec![
            Field {
                name: "foo".into(),
                typ: "string".into(),
                value: "".into(),
            },
            Field {
                name: "bar".into(),
                typ: "integer".into(),
                value: "42".into(),
            },
        ];
        let result = generate_class("MyClass", "Parent", &fields, "");
        assert!(result.contains(&"---@class MyClass : Parent".to_string()));
        assert!(result.contains(&"---@field foo string".to_string()));
        assert!(result.contains(&"---@field bar integer 42".to_string()));
    }

    #[test]
    fn test_generate_function() {
        let params = vec![
            Param {
                name: "a".into(),
                typ: "string".into(),
            },
            Param {
                name: "b?".into(),
                typ: "integer".into(),
            },
        ];
        let returns = vec![Param {
            name: "".into(),
            typ: "boolean".into(),
        }];
        let result = generate_function("test.func", &params, &returns, None);
        assert!(result.contains(&"---@param a string".to_string()));
        assert!(result.contains(&"---@param b? integer".to_string()));
        assert!(result.contains(&"---@return boolean".to_string()));
        assert!(result.contains(&"function test.func(a, b) end".to_string()));
    }
}
