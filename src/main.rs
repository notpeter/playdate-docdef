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
use std::collections::{BTreeMap, HashSet};

static PLAYDATE_LUARS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/playdate.luars"));

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

fn stubs_with_docs(
    playdate_luars: BTreeMap<String, LuarsStatement<'_>>,
    docs: String,
) -> Vec<FinStub> {
    let mut variables = Vec::new();
    let mut functions = Vec::new();
    let scraped_stubs = scrape::scrape(docs, &playdate_luars);

    let mut both: HashSet<String> = HashSet::new();

    // Then process scraped stubs, separating into variables and functions
    for stub in scraped_stubs.values() {
        match stub {
            Stub::Function(func_stub) => {
                both.insert(func_stub.lua_def());
                functions.push(FinStub::from_stub(&stub));
            }
            Stub::Variable(var_stub) => {
                eprintln!("WARN: Found scraped non-function {}", var_stub.lua_def());
                // dbg!(&var_stub);
            }
        }
    }

    // Finally collect remaining functions from luars that weren't in scraped docs
    for s in playdate_luars.values() {
        match s {
            LuarsStatement::Global(_, _, _) | LuarsStatement::Local(_, _, _) => {
                variables.push(FinStub::from_luars(s));
            }
            LuarsStatement::Function(_, _, _) => {
                if !both.contains(s.lua_def().as_str()) {
                    functions.push(FinStub::from_luars(s));
                }
            }
        }
    }

    // Combine variables and functions in the desired order
    let mut fin_stubs = Vec::new();
    fin_stubs.extend(variables);
    fin_stubs.extend(functions);
    fin_stubs
}

/// Outputs just the stubs as defined in the .luars source
fn stubs_without_docs(statements: BTreeMap<String, LuarsStatement<'_>>) -> Vec<FinStub> {
    let mut variables = Vec::new();
    let mut functions = Vec::new();
    for statement in statements.values() {
        match statement {
            LuarsStatement::Local(_, _, _) | LuarsStatement::Global(_, _, _) => {
                variables.push(FinStub::from_luars(statement));
            }
            LuarsStatement::Function(_, _, _) => {
                functions.push(FinStub::from_luars(statement));
            }
        }
    }
    let mut fin_stubs = Vec::new();
    fin_stubs.extend(variables);
    fin_stubs.extend(functions);
    fin_stubs
}

fn main() {
    let args = setup();
    let playdate_luars = parse_document(&PLAYDATE_LUARS);
    let fin_stubs = match args.action {
        Action::Annotate => stubs_with_docs(playdate_luars, fetch_docs(args)),
        Action::Stub => stubs_without_docs(playdate_luars),
    };
    go_out(fin_stubs);
}
