use std::{fmt::Display, cmp::Ordering};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "luars.pest"]
pub struct LuarsParser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LuarsStatement<'a> {
    Module(&'a str),
    Constant(&'a str, &'a str, isize),
    Object(&'a str, &'a str, Vec<(&'a str, &'a str)>),
    Variable(&'a str, &'a str),
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
            LuarsStatement::Module(name) => (name, 1),
            LuarsStatement::Constant(name, _, _) => (name, 2),
            LuarsStatement::Variable(name, _) => (name, 2),
            LuarsStatement::Object(name, _, _) => (name, 4),
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
            LuarsStatement::Module(name) => {
                write!(f, "---@class {name}\n{name} = {{}}\n", name=name)
            }
            LuarsStatement::Constant(name, cons_type, cons_integer) => {
                write!(f, "---@type {}\n{} <const> = {}\n", cons_type, name, cons_integer)
            }
            LuarsStatement::Object(name, parent, tablekeys) => {
                if String::from(*name) == "playdate._PowerStatus".to_string() {
                    println!("---------------------------");
                    println!("{} {} {:?}", name, parent, tablekeys);
                }
                let fields = &tablekeys.iter().map(|(k, v)| format!("---@field {} {}", k, v)).collect::<Vec<String>>().join("\n");
                write!(f, "---@class {} : {}\n{}\n{} = {{}}\n", name, parent, fields, name)
            }
            LuarsStatement::Variable(name, var_type) => {
                write!(f, "---@type {}\n{} = {{}}\n", var_type, name)
            }
            LuarsStatement::Function(name, params, returns) => {
                if params.len() == 0 {
                    let returns = &returns.iter().map(|(k, v)| format!("---@return {} {}", k, v)).collect::<Vec<String>>().join("\n").replace("  ", " ");
                    write!(f, "{}\nfunction {}()\n", returns, name)
                } else {
                    let params_ = &params.iter().map(|(k, _)| format!("{}", k)).collect::<Vec<String>>().join(", ");
                    let params = &params.iter().map(|(k, v)| format!("---@param {} {}", k, v)).collect::<Vec<String>>().join("\n");
                    let returns = &returns.iter().map(|(k, v)| format!("---@return {} {}", k, v)).collect::<Vec<String>>().join("\n").replace("  ", " ");
                    write!(f, "{}\n{}\nfunction {}({})\n", params, returns, name, params_)
                }

            }
            // _ => { println!("{}", self); unreachable!() }
        }
    }
}

pub fn parse_document(unparsed_file: &str) -> Vec<LuarsStatement> {
    let document = LuarsParser::parse(Rule::Document, &unparsed_file)
    .expect("unsuccessful parse")
    .next().unwrap();

    let mut statements: Vec<LuarsStatement> = Vec::new();

    for line in document.into_inner() {
        let f = match line.as_rule() {
            Rule::Module => {
                let name = line.into_inner().next().unwrap().as_str();
                LuarsStatement::Module(name)
            }
            Rule::Constant => {
                let mut iterator = line.into_inner();
                let cons_name: &str = iterator.next().unwrap().as_str();
                let cons_type: &str = iterator.next().unwrap().as_str();
                let cons_value: isize = iterator.next().unwrap().as_str().parse::<isize>().unwrap();
                LuarsStatement::Constant(cons_name, cons_type, cons_value)
            }
            Rule::Object => {
                let mut iterator = line.into_inner();
                let obj_name: &str = iterator.next().unwrap().as_str();
                let obj_type: &str = iterator.next().unwrap().as_str();
                let mut obj_proto: Vec<(&str, &str)> = Vec::new();
                if iterator.peek().is_some() && iterator.peek().unwrap().as_rule() == Rule::TablePrototype {
                    let mut field = iterator.next().unwrap().into_inner();
                    while field.peek().is_some() && field.peek().unwrap().as_rule() == Rule::TableKey {
                        let field_name: &str = field.next().unwrap().as_str();
                        let field_type: &str = field.next().unwrap().as_str();
                        //     println!("{:#?}", field);
                        obj_proto.push((field_name, field_type));
                    }
                }
                // println!("{} : {} = {:?}", obj_name, obj_type, obj_proto);
                LuarsStatement::Object(obj_name, obj_type, obj_proto)
            }
            Rule::Variable => {
                let mut iterator = line.into_inner();
                let var_name: &str = iterator.next().unwrap().as_str();
                let var_type: &str = iterator.next().unwrap().as_str();
                LuarsStatement::Variable(var_name, var_type)
            }
            Rule::Function => {
                let mut iterator = line.into_inner();
                let func_name: &str = iterator.next().unwrap().as_str();
                let mut func_params: Vec<(&str, &str)> = Vec::new();
                if iterator.peek().unwrap().as_rule() != Rule::Return {
                    let func_params_iterator = iterator.next().unwrap().into_inner();
                    for param in func_params_iterator {
                        let mut param_iterator = param.into_inner();
                        let param_name: &str = param_iterator.next().unwrap().as_str();
                        let param_type: &str = param_iterator.next().unwrap().as_str();
                        func_params.push((param_name, param_type));
                    }
                }
                let mut func_returns: Vec<(&str, &str)> = Vec::new();
                let func_returns_iterator = iterator.next().unwrap().into_inner();
                for ret in func_returns_iterator {
                    if ret.as_rule() == Rule::TypeLua {
                        func_returns.push(("", ret.as_str()));
                    }
                }
                LuarsStatement::Function(func_name, func_params, func_returns)
            }
            _ => {
                println!("Rule: {:?}", line.as_rule());
                unreachable!()
            }
        };
        //println!("{:?}", f);
        statements.push(f);
    }
    statements.sort();
    statements
}
