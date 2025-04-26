mod args;
mod finstub;
mod fixes;
mod luars;
mod scrape;
mod stub;

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
        println!("{}\n", stub.generate_stub().join("\n"));
    }
    println!("--- End of LuaCATS stubs.");
}

fn main() {
    let (args, response) = crate::args::setup();

    let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
    let statements: Vec<luars::LuarsStatement<'_>> = luars::parse_document(&unparsed_file);
    let mut fin_stubs: Vec<FinStub> = Vec::new();
    let mut both: HashSet<String> = HashSet::new();
    for s in &statements {
        match s {
            luars::LuarsStatement::Global(_, _, _) | luars::LuarsStatement::Local(_, _, _) => {
                fin_stubs.push(FinStub::from_luars(s));
            }
            _ => {}
        }
    }
    match args.action {
        Action::Annotate => {
            let stubs = scrape::scrape(response, &statements);
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
                    luars::LuarsStatement::Function(_, _, _) => {
                        if !both.contains(s.func_sig().as_str()) {
                            fin_stubs.push(FinStub::from_luars(s));
                        }
                    }
                    luars::LuarsStatement::Local(_name, _parent, _) => {
                        // eprintln!("local {_name} {_parent}")
                    }
                    _ => {}
                }
            }
            go_out(fin_stubs);
        }
        Action::Stub => {
            for s in &statements {
                match s {
                    luars::LuarsStatement::Function(_, _, _) => {
                        fin_stubs.push(FinStub::from_luars(s));
                    }
                    _ => {}
                }
            }
            go_out(fin_stubs);
        }
    }
}
