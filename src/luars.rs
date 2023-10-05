use std::cmp::Ordering;

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "luars.pest"]
pub struct LuarsParser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LuarsStatement<'a> {
    Global(&'a str, &'a str, Vec<(&'a str, &'a str, &'a str)>),
    Local(&'a str, &'a str, Vec<(&'a str, &'a str, &'a str)>),
    Function(&'a str, Vec<(&'a str, &'a str)>, Vec<(&'a str, &'a str)>),
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct LuarsSortKey<'a> {
    id: usize,              // [1, 2, 3, 4] = [global, local, method, instance method
    namespace: &'a str,     // namespace: playdate.graphics.image
    name: &'a str,          // complete name: playdate.graphics.image:draw
    i_or_c: isize,          // [0, 1] = [class, instance]
    sub_id: isize,          // number of parameters (longer first)
}

impl LuarsStatement<'_> {
    fn id(&self) -> LuarsSortKey {

        let (id, name, i_or_c, sub_id) = match self {
            LuarsStatement::Global(name, _, _) => (1, name, 0, 0),
            LuarsStatement::Local(name, _, _) => (2, name, 0, 0),
            LuarsStatement::Function(name, params, _) => {
                if name.contains(":") {
                    (3, name, 1, -1 * params.len() as isize) // instance methods
                } else {
                    (3, name, 0, -1 * params.len() as isize) // class methods
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
        LuarsSortKey { namespace, id, name, i_or_c, sub_id }
    }
    pub fn func_sig(&self) -> String {
        match self {
            LuarsStatement::Function(name, params, _) => {
                let func_params: Vec<String> = params.iter().map(
                    |(fname, _)| fname.trim_matches('?').to_string()
                ).collect::<Vec<String>>();
                format!("{}({})", name, func_params.join(", "))
            }
            _ => {
                unreachable!("Only LuarsStatement::Function should not be called with func_sig()")
            }
        }
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

pub fn parse_tbl(pair: Pair<Rule>) -> LuarsStatement {
    let localglobal = match pair.as_rule() {
        Rule::Global => LuarsStatement::Global,
        Rule::Local => LuarsStatement::Local,
        _ => {
            eprintln!("Unexpected Rule: {:?}. Was expecting Local or Global.", pair.as_rule());
            unreachable!()
        }
    };
    let mut iterator = pair.into_inner();
    let mut obj_name: &str = "INVALID";
    let mut obj_type: &str = "";
    let mut obj_proto: Vec<(&str, &str, &str)> = Vec::new();
    while iterator.peek().is_some() {
        let chunk = iterator.next().unwrap();
        match chunk.as_rule() {
            Rule::Identifier => {
                obj_name = chunk.as_str();
            }
            Rule::CaptureType => {
                obj_type = chunk.as_str();
            }
            Rule::TableConstants => {
                let mut field = chunk.into_inner();
                let mut field_name: &str;
                let mut field_type: &str;
                let mut field_value: &str;
                while field.peek().is_some() {
                    let obj = field.next().unwrap();
                    if obj.as_rule() == Rule::TableKey {
                        field_name = obj.as_str();
                        if field.peek().is_some() && field.peek().unwrap().as_rule() == Rule::CaptureType {
                            field_type = field.next().unwrap().as_str();
                            if field.peek().is_some() && field.peek().unwrap().as_rule() == Rule::IntegerValue {
                                field_value = field.next().unwrap().as_str();
                                obj_proto.push((field_name, field_type, field_value));
                            } else {
                                obj_proto.push((field_name, field_type, ""));
                            }
                        } else {
                            eprint!("Unexpected parse: {:?} {:#?}", obj.as_rule(), obj);
                            unreachable!()
                        }
                    } else {
                        eprintln!("Unexpected parse: {:?} {:#?}", obj.as_rule(), obj);
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
    localglobal(obj_name, obj_type, obj_proto)
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
            Rule::Global => {
                parse_tbl(line)
            }
            Rule::Local => {
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
    fn global_simple() {
        let document = LuarsParser::parse(Rule::Global, "global json;\n")
            .expect("unsuccessful parse")
            .next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Global("json", "", Vec::new()));
    }
    #[test]
    fn global_table() {
        let document = LuarsParser::parse(Rule::Global, "global playdate.sound.twopolefilter: SoundEffect;")
            .expect("unsuccessful parse")
            .next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Global("playdate.sound.twopolefilter", "SoundEffect", Vec::new()));
    }
    #[test]
    fn local_type() {
        let document = LuarsParser::parse(Rule::Local, "local File: playdate.file.file;")
            .expect("unsuccessful parse")
            .next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Local("File", "playdate.file.file", Vec::new()));
    }
    #[test]
    fn local_literal() {
        let document = LuarsParser::parse(
            Rule::Local, "local Size: playdate.geometry.size = { width: number, height: number, };"
        ).expect("unsuccessful parse").next().unwrap();
        assert_eq!(parse_tbl(document), LuarsStatement::Local("Size", "playdate.geometry.size", vec![
            ("width", "number", ""),
            ("height", "number", ""),
        ]));
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
    #[test]
    fn bad_grammar() {
        let luars_pest_file = fs::read_to_string("src/luars.pest").expect("cannot read file");
        assert!(!luars_pest_file.contains("\t"), "tabs found in luars.pest");
        let lua_grammar_file = fs::read_to_string("playdate.luars").expect("cannot read file");
        assert!(!lua_grammar_file.contains("\t"), "tabs found in playdate.luars");
        let c_grammar_file = fs::read_to_string("playdate-c.luars").expect("cannot read file");
        assert!(!c_grammar_file.contains("\t"), "tabs found in playdate-c.luars");
    }
    #[test]
    fn playdate_grammar() {
        let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
        let playdate_luars = parse_document(&unparsed_file);
        assert_eq!(playdate_luars.len(), unparsed_file.matches(";").count());
    }
    #[test]
    fn playdate_c_grammar() {
        let unparsed_file = fs::read_to_string("playdate-c.luars").expect("cannot read file");
        let playdate_c_luars = parse_document(&unparsed_file);
        println!("{:?}", playdate_c_luars);
        assert_eq!(playdate_c_luars.len(), unparsed_file.matches(";").count());
    }
}
