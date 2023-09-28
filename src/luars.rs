use std::{fmt::Display, cmp::Ordering};

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "luars.pest"]
pub struct LuarsParser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LuarsStatement<'a> {
    Table(&'a str, &'a str, Vec<(&'a str, &'a str)>),
    Function(&'a str, Vec<(&'a str, &'a str)>, Vec<(&'a str, &'a str)>),
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct LuarsSortKey<'a> {
    namespace: &'a str,
    id: usize,
    name: &'a str,
}

impl LuarsStatement<'_> {

    fn id(&self) -> LuarsSortKey {
        let (name, id) = match self {
            LuarsStatement::Table(name, _, _) => (name, 4),
            LuarsStatement::Function(name, _, _) => {
                if name.contains(":") {
                    (name, 5) // instance methods
                } else {
                    (name, 3) // class methods
                }
            },
        };
        let (namespace, name) = if name.contains(":") {
            name.split_at(name.rfind(":").unwrap())
        } else if name.contains(".") {
            name.split_at(name.rfind(".").unwrap())
        } else {
            ("", *name)
        };
        LuarsSortKey { namespace, id, name }
    }
}

impl PartialOrd for LuarsStatement<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for LuarsStatement<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl Display for LuarsStatement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuarsStatement::Table(name, parent, tablekeys) => {
                let fields = &tablekeys.iter().map(|(k, v)| format!("---@field {} {}", k, v)).collect::<Vec<String>>().join("\n");
                if parent.to_string() == "" {
                    write!(f, "---@class {name}\n{fields}\n{name} = {{}}\n", name=name, fields=fields)
                } else {
                    write!(f, "---@class {name} : {parent}\n{fields}\n{name} = {{}}\n", name=name, parent=parent, fields=fields)
                }
            }
            LuarsStatement::Function(name, params, returns) => {
                let returns = &returns.iter().map(
                    |(_name, _type)|
                        if _name.to_string() == "" {
                            format!("---@return {}", _type)
                        } else {
                            format!("---@return {_type} {_name}", _type=_type, _name=_name)
                        }
                ).collect::<Vec<String>>().join("\n").replace("  ", " ");
                if params.len() == 0 {
                    write!(f, "{}\nfunction {}() end\n", returns, name)
                } else {
                    let mut params_: Vec<&str> = Vec::new();
                    let mut params_out: Vec<String> = Vec::new();
                    for  (k, v) in params.iter() {
                        let k_noq = k.trim_matches('?');
                        params_.push(k_noq);
                        let p = format!("---@param {} {}", k, v);
                        params_out.push(p);
                    }
                    write!(f, "{}\n{}\nfunction {}({}) end\n", params_out.join("\n"), returns, name, params_.join(", "))
                }

            }
            // _ => { println!("{}", self); unreachable!() }
        }
    }
}

pub fn parse_tbl(pair: Pair<Rule>) -> LuarsStatement {
    let iterator = pair.into_inner();
    let mut obj_name: &str = "INVALID";
    let mut obj_type: &str = "";
    let mut obj_proto: Vec<(&str, &str)> = Vec::new();
    for field in iterator {
        match field.as_rule() {
            Rule::Identifier => {
                obj_name = field.as_str();
            }
            Rule::CaptureType => {
                obj_type = field.as_str();
            }
            Rule::TableConstants => {
                let mut field = field.into_inner();
                while field.peek().is_some() && field.peek().unwrap().as_rule() == Rule::TableKey {
                    let field_name: &str = field.next().unwrap().as_str();
                    let field_type: &str = field.next().unwrap().as_str();
                    obj_proto.push((field_name, field_type));
                }
            }
            _ => {
                eprintln!("Rule: {:?}", field.as_rule());
                unreachable!()
            }
        }
    }
    LuarsStatement::Table(obj_name, obj_type, obj_proto)
}

pub fn parse_function(pair: Pair<Rule>) -> LuarsStatement {
    let iterator = pair.into_inner();
    let mut name: &str = "INVALID";
    let mut params: Vec<(&str, &str)> = Vec::new();
    let mut returns: Vec<(&str, &str)> = Vec::new();
    for chunk in iterator {
        match chunk.as_rule() {
            Rule::FunctionName => {
                name = chunk.as_str();
            }
            Rule::FunctionalParameters => {
                let mut field = chunk.into_inner();
                while field.peek().is_some() {
                    let field_name: &str = field.next().unwrap().as_str();
                    let field_type: &str = field.next().unwrap().as_str();
                    params.push((field_name, field_type));
                }
            }
            Rule::Return => {
                let mut field = chunk.into_inner();
                match field.peek().unwrap().as_rule() {
                    Rule::OptionalType => {
                        returns.push(("", field.next().unwrap().as_str()));
                    }
                    Rule::FunctionalParameters => {
                        let mut field = field.next().unwrap().into_inner();
                        while field.peek().is_some() {
                            let field_name: &str = field.next().unwrap().as_str();
                            let field_type: &str = field.next().unwrap().as_str();
                            returns.push((field_name, field_type));
                        }
                    }
                    _ => {
                        eprintln!("Rule: {:?}", field.peek());
                        unreachable!()
                    }
                }
            }
            _ => {
                eprintln!("Rule: {:?}", chunk.as_rule());
                unreachable!()
            }
        }
    }
    LuarsStatement::Function(name, params, returns)
}
pub fn parse_document(unparsed_file: &str) -> Vec<LuarsStatement> {
    let document = LuarsParser::parse(Rule::Document, &unparsed_file)
    .expect("unsuccessful parse")
    .next().unwrap();

    let mut statements: Vec<LuarsStatement> = Vec::new();

    for line in document.into_inner() {
        let f = match line.as_rule() {
            Rule::Table => {
                parse_tbl(line)
            }
            Rule::Function => {
                parse_function(line)
            }
            _ => {
                eprintln!("Rule: {:?}", line.as_rule());
                unreachable!()
            }
        };
        //println!("{:?}", f);
        statements.push(f);
    }
    statements.sort();
    statements
}

#[cfg(test)]
mod tests {
    use std::fs;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_tbl_simple() {
        let document = LuarsParser::parse(Rule::Table, "tbl json;\n")
            .expect("unsuccessful parse")
            .next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Table("json", "", Vec::new()));
    }
    #[test]
    fn tbl_type() {
        let document = LuarsParser::parse(Rule::Table, "tbl File: playdate.file.file;")
            .expect("unsuccessful parse")
            .next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Table("File", "playdate.file.file", Vec::new()));
    }
    #[test]
    fn tbl_literal() {
        let document = LuarsParser::parse(
            Rule::Table, "tbl Size: playdate.geometry.size = { width: number, height: number, };"
        ).expect("unsuccessful parse").next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Table("Size", "playdate.geometry.size", vec![
            ("width", "number"),
            ("height", "number"),
        ]));
    }
    #[test]
    fn playdate_grammar() {
        let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
        let playdate_luars = parse_document(&unparsed_file);
        assert_eq!(playdate_luars.len(), unparsed_file.matches(";").count());
    }
    #[test]
    fn funcs() {
        let document = LuarsParser::parse(Rule::Function, "fun where(): nil;")
            .expect("unsuccessful parse").next().unwrap();
        assert_eq!(
            parse_function(document),
            LuarsStatement::Function("where", Vec::new(), vec![("", "nil"),]),
        )
    }
    #[test]
    fn funcs2() {
        let document = LuarsParser::parse(Rule::Function, "fun playdate.timer.new(duration: integer, callback: function, ...?: any): Timer;")
            .expect("unsuccessful parse").next().unwrap();
        assert_eq!(
            parse_function(document),
            LuarsStatement::Function(
                "playdate.timer.new",
                vec![("duration", "integer"), ("callback", "function"), ("...?", "any"),],
                vec![("", "Timer"),]
            ),
        )
    }
    #[test]
    fn funcs3() {
        let document = LuarsParser::parse(
            Rule::Function,
            "fun GridView:getScrollPosition(): (x: integer, y: integer);"
        ).expect("bad parse").next().unwrap();
        assert_eq!(
            parse_function(document),
            LuarsStatement::Function(
                "GridView:getScrollPosition",
                vec![],
                vec![("x", "integer"), ("y", "integer"),]
            ),
        )
    }
}
