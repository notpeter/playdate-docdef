//! Playdate DocDef - Generate LuaCATS type annotations for Playdate SDK
//!
//! This tool parses the .luars type definition file and optionally scrapes
//! the official Playdate SDK documentation to generate comprehensive
//! LuaCATS-compatible stub files for IDE autocompletion.

mod args;
mod luars;
mod multi;
mod output;
mod scraper;

use args::Action;
use multi::MultiStubOutput;
use output::StubOutput;
use std::fs;
use std::path::Path;

static PLAYDATE_LUARS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/playdate.luars"));

fn main() {
    let args = args::parse();

    // Parse the .luars type definitions
    let statements = luars::parse_document(PLAYDATE_LUARS).expect("Failed to parse playdate.luars");

    if args.llm {
        let output = StubOutput::from_statements(&statements, true, true);
        output.print();
        return;
    }

    match args.action {
        Action::Stub => {
            // Generate stubs without documentation
            let output = StubOutput::from_statements(&statements, args.compact, false);
            output.print();
        }
        Action::Annotate => {
            // Scrape documentation and generate annotated stubs
            let html = args::fetch_docs(&args);
            let scraped = scraper::scrape(&html, &statements);
            let output =
                StubOutput::from_statements_with_docs(&statements, &scraped, args.compact, false);
            output.print();
        }
        Action::Multi => {
            let output = MultiStubOutput::from_statements(&statements, args.compact);
            let out_dir = Path::new("out");
            if out_dir.exists() {
                fs::remove_dir_all(out_dir).expect("Failed to clear out/ directory");
            }
            fs::create_dir_all(out_dir).expect("Failed to create out/ directory");
            output
                .write_to_dir(out_dir)
                .expect("Failed to write multi-file output");
        }
    }
}
