//! LUARS parser using Pest
//!
//! This module parses the .luars file format which defines Lua API types.

use pest::Parser;
use pest_derive::Parser;
use std::collections::BTreeMap;

#[derive(Parser)]
#[grammar = "luars.pest"]
pub struct LuarsParser;

/// Parsed statement from a .luars file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// Global table: (name, parent_type, fields)
    Global(String, String, Vec<Field>),
    /// Local type alias: (name, parent_type, fields)
    Local(String, String, Vec<Field>),
    /// Function: (name, params, returns)
    Function(String, Vec<Param>, Vec<Param>),
}

/// A field in a table definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub typ: String,
    pub value: String,
}

/// A function parameter or return value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub typ: String,
}

impl Statement {
    /// Generate the Lua function signature (used as map key)
    pub fn lua_def(&self) -> String {
        match self {
            Statement::Function(name, params, _) => {
                let param_names: Vec<&str> = params
                    .iter()
                    .map(|p| p.name.trim_end_matches('?'))
                    .collect();
                format!("{}({})", name, param_names.join(", "))
            }
            Statement::Local(name, _, _) => format!("local {} = {{}}", name),
            Statement::Global(name, _, _) => format!("{} = {{}}", name),
        }
    }

    /// Sort key for consistent ordering
    fn sort_key(&self) -> (usize, String, String, isize, isize) {
        match self {
            Statement::Global(name, _, _) => (1, namespace(name), name.clone(), 0, 0),
            Statement::Local(name, _, _) => (2, namespace(name), name.clone(), 0, 0),
            Statement::Function(name, params, _) => {
                let i_or_c = if name.contains(':') { 1 } else { 0 };
                (3, namespace(name), name.clone(), i_or_c, -(params.len() as isize))
            }
        }
    }
}

fn namespace(name: &str) -> String {
    if let Some(pos) = name.rfind(':') {
        name[..pos].to_string()
    } else if let Some(pos) = name.rfind('.') {
        name[..pos].to_string()
    } else {
        String::new()
    }
}

/// Parse a Global or Local table declaration
fn parse_table(pair: pest::iterators::Pair<Rule>) -> (String, String, Vec<Field>) {
    let mut name = String::new();
    let mut parent = String::new();
    let mut fields = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::Identifier => name = inner.as_str().to_string(),
            Rule::CaptureType => parent = inner.as_str().to_string(),
            Rule::TableConstants => fields = parse_table_constants(inner),
            _ => {}
        }
    }

    (name, parent, fields)
}

/// Parse table fields: { name: type, name2: type2 = value, ... }
fn parse_table_constants(pair: pest::iterators::Pair<Rule>) -> Vec<Field> {
    let mut fields = Vec::new();

    for field_pair in pair.into_inner() {
        if field_pair.as_rule() == Rule::TableField {
            let mut name = String::new();
            let mut typ = String::new();
            let mut value = String::new();

            for inner in field_pair.into_inner() {
                match inner.as_rule() {
                    Rule::FieldName => name = inner.as_str().to_string(),
                    Rule::CaptureType => typ = inner.as_str().to_string(),
                    Rule::IntegerValue => value = inner.as_str().to_string(),
                    _ => {}
                }
            }

            fields.push(Field { name, typ, value });
        }
    }

    fields
}

/// Parse function parameters
fn parse_parameters(pair: pest::iterators::Pair<Rule>) -> Vec<Param> {
    let mut params = Vec::new();

    let mut inner = pair.into_inner().peekable();
    while let Some(item) = inner.next() {
        match item.as_rule() {
            Rule::ParameterIdentifier | Rule::VariableParameter => {
                let name = item.as_str().to_string();
                // Next should be OptionalType
                if let Some(type_pair) = inner.next() {
                    if type_pair.as_rule() == Rule::OptionalType {
                        let typ = type_pair.as_str().to_string();
                        params.push(Param { name, typ });
                    }
                }
            }
            _ => {}
        }
    }

    params
}

/// Parse function return type(s)
fn parse_return(pair: pest::iterators::Pair<Rule>) -> Vec<Param> {
    let mut returns = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::FunctionalParameters => {
                // Multiple named returns: (x: int, y: int)
                returns = parse_parameters(inner);
            }
            Rule::OptionalType => {
                // Single unnamed return
                returns.push(Param {
                    name: String::new(),
                    typ: inner.as_str().to_string(),
                });
            }
            _ => {}
        }
    }

    returns
}

/// Parse a function declaration
fn parse_function(pair: pest::iterators::Pair<Rule>) -> Statement {
    let mut name = String::new();
    let mut params = Vec::new();
    let mut returns = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::FunctionName => name = inner.as_str().to_string(),
            Rule::FunctionalParameters => params = parse_parameters(inner),
            Rule::Return => returns = parse_return(inner),
            _ => {}
        }
    }

    Statement::Function(name, params, returns)
}

/// Parse a .luars document and return a sorted map of statements
pub fn parse_document(input: &str) -> Result<BTreeMap<String, Statement>, String> {
    let pairs = LuarsParser::parse(Rule::Document, input)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut statements = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            let stmt = match inner.as_rule() {
                Rule::Global => {
                    let (name, parent, fields) = parse_table(inner);
                    Statement::Global(name, parent, fields)
                }
                Rule::Local => {
                    let (name, parent, fields) = parse_table(inner);
                    Statement::Local(name, parent, fields)
                }
                Rule::Function => parse_function(inner),
                Rule::EOI => continue,
                _ => continue,
            };
            statements.push(stmt);
        }
    }

    // Sort by namespace, type, name for consistent output
    statements.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

    let mut result = BTreeMap::new();
    for stmt in statements {
        let key = stmt.lua_def();
        if !result.contains_key(&key) {
            result.insert(key, stmt);
        } else {
            eprintln!("Duplicate definition: {}", key);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_global() {
        let result = parse_document("global json;").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("json = {}"));
    }

    #[test]
    fn test_global_with_parent() {
        let result = parse_document("global playdate.sound.twopolefilter: SoundEffect;").unwrap();
        let stmt = result.get("playdate.sound.twopolefilter = {}").unwrap();
        match stmt {
            Statement::Global(name, parent, _) => {
                assert_eq!(name, "playdate.sound.twopolefilter");
                assert_eq!(parent, "SoundEffect");
            }
            _ => panic!("Expected Global"),
        }
    }

    #[test]
    fn test_global_with_contents() {
        let input = "global playdate = { argv: string[], isSimulator: boolean };";
        let result = parse_document(input).unwrap();
        let stmt = result.get("playdate = {}").unwrap();
        match stmt {
            Statement::Global(name, _, fields) => {
                assert_eq!(name, "playdate");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "argv");
                assert_eq!(fields[0].typ, "string[]");
            }
            _ => panic!("Expected Global"),
        }
    }

    #[test]
    fn test_local() {
        let result = parse_document("local File: playdate.file.file;").unwrap();
        let stmt = result.get("local File = {}").unwrap();
        match stmt {
            Statement::Local(name, parent, _) => {
                assert_eq!(name, "File");
                assert_eq!(parent, "playdate.file.file");
            }
            _ => panic!("Expected Local"),
        }
    }

    #[test]
    fn test_simple_function() {
        let result = parse_document("fun where(): nil;").unwrap();
        let stmt = result.get("where()").unwrap();
        match stmt {
            Statement::Function(name, params, returns) => {
                assert_eq!(name, "where");
                assert!(params.is_empty());
                assert_eq!(returns.len(), 1);
                assert_eq!(returns[0].typ, "nil");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_function_with_params() {
        let input = "fun playdate.timer.new(duration: integer, callback: function, ...?: any): Timer;";
        let result = parse_document(input).unwrap();
        let stmt = result.get("playdate.timer.new(duration, callback, ...)").unwrap();
        match stmt {
            Statement::Function(name, params, returns) => {
                assert_eq!(name, "playdate.timer.new");
                assert_eq!(params.len(), 3);
                assert_eq!(params[0].name, "duration");
                assert_eq!(params[0].typ, "integer");
                assert_eq!(params[2].name, "...?");
                assert_eq!(returns[0].typ, "Timer");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_method_with_multi_return() {
        let input = "fun GridView:getScrollPosition(): (x: integer, y: integer);";
        let result = parse_document(input).unwrap();
        let stmt = result.get("GridView:getScrollPosition()").unwrap();
        match stmt {
            Statement::Function(name, params, returns) => {
                assert_eq!(name, "GridView:getScrollPosition");
                assert!(params.is_empty());
                assert_eq!(returns.len(), 2);
                assert_eq!(returns[0].name, "x");
                assert_eq!(returns[1].name, "y");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_union_type() {
        let input = "fun playdate.buttonIsPressed(button: (integer|string)): boolean;";
        let result = parse_document(input).unwrap();
        let stmt = result.get("playdate.buttonIsPressed(button)").unwrap();
        match stmt {
            Statement::Function(_, params, _) => {
                assert_eq!(params[0].typ, "(integer|string)");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_function_type_param() {
        let input = "fun playdate.getServerTime(callback: fun(time?: string, error?: string));";
        let result = parse_document(input).unwrap();
        let stmt = result.get("playdate.getServerTime(callback)").unwrap();
        match stmt {
            Statement::Function(_, params, _) => {
                assert_eq!(params[0].typ, "fun(time?: string, error?: string)");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_full_playdate_luars() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/playdate.luars"));
        let result = parse_document(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
        let stmts = result.unwrap();
        let expected = input.matches(';').count();
        assert_eq!(stmts.len(), expected, "Expected {} statements", expected);
    }
}
