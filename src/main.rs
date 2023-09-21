use std::fs;

mod luars;

fn main() {
    let unparsed_file = fs::read_to_string("playdate.luars").expect("cannot read file");
    let poop = luars::parse_document(&unparsed_file);
    for p in poop {
        println!("{}", p);
    }
}
