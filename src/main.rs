mod luars;
mod args;
mod config;
mod fixes;
mod stub;
mod scrape;

use std::fs;
use crate::args::Action;


fn main() {
    let (args, response) = crate::args::setup();

    let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
    let statements: Vec<luars::LuarsStatement<'_>> = luars::parse_document(&unparsed_file);


    match args.action {
        Action::Annotate => {
            let stubs = scrape::scrape(response);
            for stub in stubs {
                println!("{}", stub.to_lua());
            }
        },
        Action::Stub => {
            println!("---@meta");
            println!("--- This file contains function stubs for autocompletion. DO NOT include it in your game.");
            println!("");
            for p in statements {
                println!("{}\n", p.generate_stub().join("\n"));
            }
            println!("--- End of LuaCATS stubs.");
        },
    }
}
