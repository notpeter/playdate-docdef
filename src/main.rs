mod args;
mod finstub;
mod fixes;
mod luars;
mod scrape;
mod stub;

use args::{fetch_docs, setup};
use luars::{parse_document, LuarsStatement};
use stub::Stub;

use crate::{args::Action, finstub::FinStub};
use std::{collections::HashSet, fs};

fn go_out(fin_stubs: Vec<FinStub>) {
    println!("---@meta");
    println!(
        "--- This file contains function stubs for autocompletion. DO NOT include it in your game."
    );
    println!("");

    for stub in fin_stubs {
        let output = stub.generate_stub();
        if !output.is_empty() {
            println!("{}\n", output.join("\n"));
        }
    }
    println!("--- End of LuaCATS stubs.");
}

fn annotated_stubs(
    statements: Vec<LuarsStatement<'_>>,
    mut fin_stubs: Vec<FinStub>,
    docs: String,
) -> Vec<FinStub> {
    let stubs = scrape::scrape(docs, &statements);
    let mut both: HashSet<String> = HashSet::new();
    for s in &statements {
        match s {
            LuarsStatement::Global(_, _, _) | LuarsStatement::Local(_, _, _) => {
                fin_stubs.push(FinStub::from_luars(s));
            }
            _ => {}
        }
    }
    for stub in stubs {
        fin_stubs.push(FinStub::from_stub(&stub));
        match stub {
            Stub::Function(stub) => {
                both.insert(stub.func_signature());
            }
            Stub::Variable(stub) => {
                both.insert(stub.title);
            }
        }
    }
    for s in &statements {
        match s {
            LuarsStatement::Function(_, _, _) => {
                if !both.contains(s.func_sig().as_str()) {
                    fin_stubs.push(FinStub::from_luars(s));
                }
            }
            _ => {}
        }
    }
    fin_stubs
}

/// Outputs just the stubs as defined in the .luars source
fn just_stubs(statements: Vec<LuarsStatement<'_>>, mut fin_stubs: Vec<FinStub>) -> Vec<FinStub> {
    for s in &statements {
        fin_stubs.push(FinStub::from_luars(s));
    }
    fin_stubs
}

fn main() {
    let args = setup();

    let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
    let statements: Vec<LuarsStatement<'_>> = parse_document(&unparsed_file);
    let mut fin_stubs: Vec<FinStub> = Vec::new();
    fin_stubs = match args.action {
        Action::Annotate => {
            let docs = fetch_docs(args);
            annotated_stubs(statements, fin_stubs, docs)
        }
        Action::Stub => just_stubs(statements, fin_stubs),
    };
    go_out(fin_stubs);
}
