use std::fmt::Display;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "luars.pest"]
pub struct LuarsParser;

#[derive(Debug)]
pub enum LuarsStatement<'a> {
    Module(&'a str),
    Constant(&'a str, &'a str, isize),
    Object(&'a str, &'a str, Vec<(&'a str, &'a str)>),
    Variable(&'a str, &'a str),
    Function(&'a str, Vec<(&'a str, &'a str)>, Vec<(&'a str, &'a str)>),
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
    statements
}
