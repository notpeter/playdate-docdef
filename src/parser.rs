//! LUARS parser using nom
//!
//! This parser handles the .luars file format which defines Lua API types.
//! Format examples:
//!   global json;
//!   fun json.decode(str: string): table;
//!   global playdate = { argv: string[], isSimulator: boolean };
//!   local Timer: playdate.timer;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace1},
    combinator::{all_consuming, map, opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
};
use std::collections::BTreeMap;

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
    fn sort_key(&self) -> (usize, &str, &str, isize, isize) {
        match self {
            Statement::Global(name, _, _) => (1, namespace(name), name, 0, 0),
            Statement::Local(name, _, _) => (2, namespace(name), name, 0, 0),
            Statement::Function(name, params, _) => {
                let i_or_c = if name.contains(':') { 1 } else { 0 };
                (3, namespace(name), name, i_or_c, -(params.len() as isize))
            }
        }
    }
}

fn namespace(name: &str) -> &str {
    if let Some(pos) = name.rfind(':') {
        &name[..pos]
    } else if let Some(pos) = name.rfind('.') {
        &name[..pos]
    } else {
        ""
    }
}

// --- Nom Parser Combinators ---

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Parse a single Lua identifier (e.g., "foo", "_bar", "baz123")
fn lua_ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        take_while1(is_ident_start),
        take_while(is_ident_char),
    )).parse(input)
}

/// Parse a dotted identifier (e.g., "playdate.graphics.image")
fn dotted_ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        lua_ident,
        many0(pair(char('.'), lua_ident)),
    )).parse(input)
}

/// Parse optional whitespace
fn ws(input: &str) -> IResult<&str, &str> {
    take_while(|c| c == ' ' || c == '\t' || c == '\n' || c == '\r')(input)
}

/// Parse an optional '?' marker
fn optional_marker(input: &str) -> IResult<&str, bool> {
    map(opt(char('?')), |o| o.is_some()).parse(input)
}

/// Parse an integer value (possibly negative)
fn integer_value(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        opt(char('-')),
        take_while1(|c: char| c.is_ascii_digit()),
    )).parse(input)
}

/// Parse a quoted string value
fn quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        take_while(|c| c != '"'),
        char('"'),
    ).parse(input)
}

/// Parse a type expression (can be complex with unions, arrays, functions)
fn type_expr(input: &str) -> IResult<&str, String> {
    // Handle parenthesized union types: (type1|type2)
    let paren_union = map(
        delimited(
            char('('),
            separated_list1(
                (ws, char('|'), ws),
                single_type,
            ),
            char(')'),
        ),
        |types| format!("({})", types.join("|")),
    );

    // Handle simple type or union without parens
    let simple_or_union = map(
        separated_list1(
            (ws, char('|'), ws),
            single_type,
        ),
        |types| types.join("|"),
    );

    alt((paren_union, simple_or_union)).parse(input)
}

/// Parse a single type (not a union)
fn single_type(input: &str) -> IResult<&str, String> {
    alt((
        function_type,
        table_generic_type,
        basic_type,
    )).parse(input)
}

/// Parse function type: fun(params): return
fn function_type(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("fun(").parse(input)?;
    let (input, _) = ws(input)?;
    let (input, params) = opt(func_params_str).parse(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(')').parse(input)?;
    let (input, _) = ws(input)?;
    let (input, ret) = opt(preceded(
        (char(':'), ws),
        map(
            pair(type_expr, optional_marker),
            |(t, opt)| if opt { format!("{}?", t) } else { t },
        ),
    )).parse(input)?;

    let params_str = params.unwrap_or_default();
    let result = match ret {
        Some(r) => format!("fun({}): {}", params_str, r),
        None => format!("fun({})", params_str),
    };
    Ok((input, result))
}

/// Parse function params as a string (for embedding in type signatures)
fn func_params_str(input: &str) -> IResult<&str, String> {
    map(
        separated_list1(
            (ws, char(','), ws),
            func_param_str,
        ),
        |params| params.join(", "),
    ).parse(input)
}

fn func_param_str(input: &str) -> IResult<&str, String> {
    // Handle variadic: ...?: type
    let variadic = map(
        (
            tag("..."),
            optional_marker,
            ws,
            char(':'),
            ws,
            type_expr,
        ),
        |(_, opt, _, _, _, typ)| {
            if opt {
                format!("...?: {}", typ)
            } else {
                format!("...: {}", typ)
            }
        },
    );

    // Handle normal: name?: type
    let normal = map(
        (
            lua_ident,
            optional_marker,
            ws,
            char(':'),
            ws,
            type_expr,
        ),
        |(name, opt, _, _, _, typ)| {
            if opt {
                format!("{}?: {}", name, typ)
            } else {
                format!("{}: {}", name, typ)
            }
        },
    );

    alt((variadic, normal)).parse(input)
}

/// Parse table<K, V> type
fn table_generic_type(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("table<").parse(input)?;
    let (input, _) = ws(input)?;
    let (input, key) = dotted_ident(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(',').parse(input)?;
    let (input, _) = ws(input)?;
    let (input, val) = type_expr(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('>').parse(input)?;
    Ok((input, format!("table<{}, {}>", key, val)))
}

/// Parse a basic type with optional [] array suffix and ? optional marker
fn basic_type(input: &str) -> IResult<&str, String> {
    let (input, name) = dotted_ident(input)?;
    let (input, arrays) = recognize(many0(tag("[]"))).parse(input)?;
    let (input, optional) = optional_marker(input)?;

    let mut result = format!("{}{}", name, arrays);
    if optional {
        result.push('?');
    }
    Ok((input, result))
}

/// Parse a function parameter
fn func_param(input: &str) -> IResult<&str, Param> {
    // Handle variadic: ...?: type  or ...: type
    let variadic = map(
        (
            tag("..."),
            optional_marker,
            ws,
            char(':'),
            ws,
            type_expr,
        ),
        |(_, opt, _, _, _, typ)| Param {
            name: if opt { "...?".to_string() } else { "...".to_string() },
            typ,
        },
    );

    // Handle normal: name?: type  or name: type
    let normal = map(
        (
            lua_ident,
            optional_marker,
            ws,
            char(':'),
            ws,
            type_expr,
        ),
        |(name, opt, _, _, _, typ)| Param {
            name: if opt { format!("{}?", name) } else { name.to_string() },
            typ,
        },
    );

    alt((variadic, normal)).parse(input)
}

/// Parse function parameters list
fn func_params(input: &str) -> IResult<&str, Vec<Param>> {
    let (input, result) = separated_list0(
        (ws, char(','), ws),
        func_param,
    ).parse(input)?;
    let (input, _) = ws(input)?;
    // Handle trailing comma and/or ...
    let (input, _) = opt((char(','), ws, opt(tag("...")), ws, opt(char(',')))).parse(input)?;

    Ok((input, result))
}

/// Parse function return type(s)
fn func_return(input: &str) -> IResult<&str, Vec<Param>> {
    let (input, _) = char(':').parse(input)?;
    let (input, _) = ws(input)?;

    // Multiple named returns: (name: type, name2: type2)
    let multi_return = map(
        delimited(
            char('('),
            (
                ws,
                separated_list0(
                    (ws, char(','), ws),
                    func_param,
                ),
                ws,
            ),
            char(')'),
        ),
        |(_, params, _)| params,
    );

    // Single return type (no name)
    let single_return = map(
        pair(type_expr, optional_marker),
        |(typ, opt)| vec![Param {
            name: String::new(),
            typ: if opt { format!("{}?", typ) } else { typ },
        }],
    );

    alt((multi_return, single_return)).parse(input)
}

/// Parse a function name (may include : for methods)
fn func_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        dotted_ident,
        opt(pair(char(':'), lua_ident)),
    )).parse(input)
}

/// Parse a table field: name: type or name: type = value
fn table_field(input: &str) -> IResult<&str, Field> {
    // Handle quoted string names
    let quoted_name = map(quoted_string, |s| s.to_string());
    // Handle variadic ...
    let variadic_name = map(tag("..."), |s: &str| s.to_string());
    // Handle normal identifier with optional ?
    let ident_name = map(
        pair(lua_ident, optional_marker),
        |(name, opt)| if opt { format!("{}?", name) } else { name.to_string() },
    );

    let (input, name) = alt((quoted_name, variadic_name, ident_name)).parse(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(':').parse(input)?;
    let (input, _) = ws(input)?;
    let (input, typ) = type_expr(input)?;
    let (input, _) = ws(input)?;
    let (input, value) = opt(preceded(
        (char('='), ws),
        integer_value,
    )).parse(input)?;

    Ok((input, Field {
        name,
        typ,
        value: value.unwrap_or("").to_string(),
    }))
}

/// Parse table contents: { field1: type1, field2: type2 = value, ... }
fn table_contents(input: &str) -> IResult<&str, Vec<Field>> {
    delimited(
        (char('{'), ws),
        terminated(
            separated_list0(
                (ws, char(','), ws),
                table_field,
            ),
            (ws, opt(char(','))),
        ),
        (ws, char('}')),
    ).parse(input)
}

/// Parse a table declaration (for global/local)
fn table_decl(input: &str) -> IResult<&str, (String, String, Vec<Field>)> {
    let (input, name) = dotted_ident(input)?;
    let (input, _) = ws(input)?;

    // Optional parent type: `: ParentType`
    let (input, parent) = opt(preceded(
        (char(':'), ws),
        dotted_ident,
    )).parse(input)?;

    let (input, _) = ws(input)?;

    // Optional contents: `= { ... }`
    let (input, contents) = opt(preceded(
        (char('='), ws),
        table_contents,
    )).parse(input)?;

    Ok((input, (
        name.to_string(),
        parent.unwrap_or("").to_string(),
        contents.unwrap_or_default(),
    )))
}

/// Parse a global statement: global <table>;
fn global_stmt(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("global").parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (name, parent, contents)) = table_decl(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';').parse(input)?;

    Ok((input, Statement::Global(name, parent, contents)))
}

/// Parse a local statement: local <table>;
fn local_stmt(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("local").parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, (name, parent, contents)) = table_decl(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';').parse(input)?;

    Ok((input, Statement::Local(name, parent, contents)))
}

/// Parse a function statement: fun <name>(<params>): <return>;
fn func_stmt(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("fun").parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = func_name(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('(').parse(input)?;
    let (input, _) = ws(input)?;
    let (input, params) = func_params(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(')').parse(input)?;
    let (input, _) = ws(input)?;
    let (input, returns) = opt(func_return).parse(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(char('?')).parse(input)?; // trailing optional marker on return
    let (input, _) = ws(input)?;
    let (input, _) = char(';').parse(input)?;

    Ok((input, Statement::Function(
        name.to_string(),
        params,
        returns.unwrap_or_default(),
    )))
}

/// Parse a single statement
fn statement(input: &str) -> IResult<&str, Statement> {
    alt((global_stmt, local_stmt, func_stmt)).parse(input)
}

/// Parse entire document
fn document(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, _) = ws(input)?;
    let (input, stmts) = separated_list0(
        multispace1,
        statement,
    ).parse(input)?;
    let (input, _) = ws(input)?;
    Ok((input, stmts))
}

/// Parse a .luars document and return a sorted map of statements
pub fn parse_document(input: &str) -> Result<BTreeMap<String, Statement>, String> {
    match all_consuming(document).parse(input) {
        Ok((_, mut stmts)) => {
            // Sort by namespace, then type, then name
            stmts.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

            let mut result = BTreeMap::new();
            for stmt in stmts {
                let key = stmt.lua_def();
                if !result.contains_key(&key) {
                    result.insert(key, stmt);
                } else {
                    eprintln!("Duplicate definition: {}", key);
                }
            }
            Ok(result)
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_playdate_luars() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/playdate.luars"));
        let result = parse_document(input);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
        let stmts = result.unwrap();
        // Should have approximately the same count as semicolons in the file
        let expected = input.matches(';').count();
        assert_eq!(stmts.len(), expected, "Expected {} statements", expected);
    }

    #[test]
    fn test_simple_global() {
        let (_, stmt) = global_stmt("global json;").unwrap();
        assert_eq!(stmt, Statement::Global("json".into(), "".into(), vec![]));
    }

    #[test]
    fn test_global_with_parent() {
        let (_, stmt) = global_stmt("global playdate.sound.twopolefilter: SoundEffect;").unwrap();
        assert_eq!(stmt, Statement::Global(
            "playdate.sound.twopolefilter".into(),
            "SoundEffect".into(),
            vec![],
        ));
    }

    #[test]
    fn test_global_with_contents() {
        let input = "global playdate = { argv: string[], isSimulator: boolean };";
        let (_, stmt) = global_stmt(input).unwrap();
        match stmt {
            Statement::Global(name, parent, fields) => {
                assert_eq!(name, "playdate");
                assert_eq!(parent, "");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "argv");
                assert_eq!(fields[0].typ, "string[]");
            }
            _ => panic!("Expected Global"),
        }
    }

    #[test]
    fn test_local() {
        let (_, stmt) = local_stmt("local File: playdate.file.file;").unwrap();
        assert_eq!(stmt, Statement::Local(
            "File".into(),
            "playdate.file.file".into(),
            vec![],
        ));
    }

    #[test]
    fn test_simple_function() {
        let (_, stmt) = func_stmt("fun where(): nil;").unwrap();
        assert_eq!(stmt, Statement::Function(
            "where".into(),
            vec![],
            vec![Param { name: "".into(), typ: "nil".into() }],
        ));
    }

    #[test]
    fn test_function_with_params() {
        let input = "fun playdate.timer.new(duration: integer, callback: function, ...?: any): Timer;";
        let (_, stmt) = func_stmt(input).unwrap();
        match stmt {
            Statement::Function(name, params, returns) => {
                assert_eq!(name, "playdate.timer.new");
                assert_eq!(params.len(), 3);
                assert_eq!(params[0].name, "duration");
                assert_eq!(params[0].typ, "integer");
                assert_eq!(params[2].name, "...?");
                assert_eq!(returns.len(), 1);
                assert_eq!(returns[0].typ, "Timer");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_method_with_multi_return() {
        let input = "fun GridView:getScrollPosition(): (x: integer, y: integer);";
        let (_, stmt) = func_stmt(input).unwrap();
        match stmt {
            Statement::Function(name, params, returns) => {
                assert_eq!(name, "GridView:getScrollPosition");
                assert_eq!(params.len(), 0);
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
        let (_, stmt) = func_stmt(input).unwrap();
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
        let (_, stmt) = func_stmt(input).unwrap();
        match stmt {
            Statement::Function(_, params, _) => {
                assert_eq!(params[0].typ, "fun(time?: string, error?: string)");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_parse_document() {
        let input = r#"
global json;
fun json.decode(str: string): table;
fun json.encode(table: table): string;
"#;
        let result = parse_document(input).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.contains_key("json = {}"));
        assert!(result.contains_key("json.decode(str)"));
        assert!(result.contains_key("json.encode(table)"));
    }
}
