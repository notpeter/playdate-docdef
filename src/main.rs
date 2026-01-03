//! Playdate DocDef - Generate LuaCATS type annotations for Playdate SDK
//!
//! This tool parses the .luars type definition file and optionally scrapes
//! the official Playdate SDK documentation to generate comprehensive
//! LuaCATS-compatible stub files for IDE autocompletion.

mod args;
mod doc_scraper;
mod output;
mod parser;

use args::Action;
use output::StubOutput;

static PLAYDATE_LUARS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/playdate.luars"));

fn main() {
    let args = args::parse();

    // Parse the .luars type definitions using the new nom parser
    let statements = parser::parse_document(PLAYDATE_LUARS)
        .expect("Failed to parse playdate.luars");

    match args.action {
        Action::Stub => {
            // Generate stubs without documentation
            let output = StubOutput::from_statements(&statements);
            output.print();
        }
        Action::Annotate => {
            // Scrape documentation and generate annotated stubs
            let html = args::fetch_docs(&args);
            let scraped = doc_scraper::scrape(&html, &statements);
            let output = StubOutput::from_statements_with_docs(&statements, &scraped);
            output.print();
        }
    }
}
