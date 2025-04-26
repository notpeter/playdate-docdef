mod args;
mod finstub;
mod fixes;
mod luars;
mod scrape;
mod stub;

use args::{fetch_docs, setup};
use luars::{LuarsStatement, parse_document};
use stub::Stub;

use crate::{args::Action, finstub::FinStub};
use std::{collections::HashSet, fs};

fn go_out(fin_stubs: Vec<FinStub>) {
    let header = [
        "---@meta",
        "--- This file contains function stubs for autocompletion. DO NOT include it in your game.",
    ];
    println!("{}\n", header.join("\n"));

    for stub in fin_stubs {
        let output = stub.generate_stub();
        if !output.is_empty() {
            println!("{}\n", output.join("\n"));
        }
    }
    println!("--- End of LuaCATS stubs.");
}

fn annotated_stubs(playdate_luars: Vec<LuarsStatement<'_>>, docs: String) -> Vec<FinStub> {
    let mut fin_stubs = Vec::new();
    let scraped_stubs = scrape::scrape(docs, &playdate_luars);

    let mut both: HashSet<String> = HashSet::new();
    for s in &playdate_luars {
        match s {
            LuarsStatement::Global(_, _, _) | LuarsStatement::Local(_, _, _) => {
                fin_stubs.push(FinStub::from_luars(s));
            }
            _ => {}
        }
    }
    for stub in scraped_stubs {
        fin_stubs.push(FinStub::from_stub(&stub));
        match stub {
            Stub::Function(stub) => {
                both.insert(stub.lua_def());
            }
            Stub::Variable(stub) => {
                both.insert(stub.title);
            }
        }
    }
    for s in &playdate_luars {
        match s {
            LuarsStatement::Function(_, _, _) => {
                if !both.contains(s.lua_def().as_str()) {
                    fin_stubs.push(FinStub::from_luars(s));
                }
            }
            _ => {}
        }
    }
    fin_stubs
}

/// Outputs just the stubs as defined in the .luars source
fn just_stubs(statements: Vec<LuarsStatement<'_>>) -> Vec<FinStub> {
    let mut fin_stubs = Vec::new();
    for s in &statements {
        fin_stubs.push(FinStub::from_luars(s));
    }
    fin_stubs
}

fn main() {
    let args = setup();
    let playdate_luars_file = fs::read_to_string("playdate.luars").expect("cannot read file");
    let playdate_luars = parse_document(&playdate_luars_file);
    let fin_stubs = match args.action {
        Action::Annotate => annotated_stubs(playdate_luars, fetch_docs(args)),
        Action::Stub => just_stubs(playdate_luars),
    };
    go_out(fin_stubs);
}
