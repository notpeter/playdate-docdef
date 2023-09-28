use std::fs;

mod luars;

fn main() {
    let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
    let poop = luars::parse_document(&unparsed_file);
    println!("---@meta");
    println!("--- This file contains function stubs for autocompletion. DO NOT include it in your game.");
    for p in poop {
        println!("{}", p);
    }
    println!("--- End of LuaCATS stubs.")
}
