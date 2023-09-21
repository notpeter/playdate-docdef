use std::fs;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "luars.pest"]
pub struct LuarsParser;


#[derive(Debug)]
enum LuarsStatement<'a> {
    Module(&'a str),
    Variable(&'a str, &'a str),
    Constant(&'a str, &'a str, isize),
    Function(&'a str, Vec<(&'a str, &'a str)>),
    Object(&'a str, &'a str, Vec<(&'a str, &'a str)>),
}



fn main() {
    let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");

    let document = LuarsParser::parse(Rule::Document, &unparsed_file)
    .expect("unsuccessful parse")
    .next().unwrap();


    for line in document.into_inner() {
        let f = match line.as_rule() {
            Rule::Module => {
                let name = line.into_inner().next().unwrap().as_str();
                LuarsStatement::Module(name)
            }
            Rule::Constant => {
                let mut iterator = line.into_inner();
                // println!("{:#?}", line.into_inner());
                let cons_name: &str = iterator.next().unwrap().as_str();
                let cons_type: &str = iterator.next().unwrap().as_str();
                let cons_value: isize = iterator.next().unwrap().as_str().parse::<isize>().unwrap();
                LuarsStatement::Constant(cons_name, cons_type, cons_value)
            }
            Rule::Object => {
                let mut iterator = line.into_inner();
                let obj_name: &str = iterator.next().unwrap().as_str();
                let obj_type: &str = iterator.next().unwrap().as_str();
                let mut obj_fields: Vec<(&str, &str)> = Vec::new();
                LuarsStatement::Object(obj_name, obj_type, obj_fields)
            }
            // Rule::Variable => {
            //     println!("Variable!");
            // }
            // Rule::Function => {
            //     println!("Function!");
            // }
            // _ => {
            //     println!("Rule: {:?}", record.as_rule(), record.);
            // }
            _ => unreachable!(),
        };
        println!("{:?}", f);
    }
    // println!("Number of records: {}", statement_count);

    // let successful_parse = LuarsParser::parse(Rule::Statement, "mod json;");
    // println!("{:?}", successful_parse);
}
