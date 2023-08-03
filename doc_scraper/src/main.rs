#![allow(dead_code)]

use std::{fs::File, io::Read};
use regex::Regex;
use scraper::Selector;
use clap::{Parser, ValueEnum};
// use serde_json::{Result, Value};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value="stub")]
    action: Action,

    /// Filename to load from
    #[arg(short, long, value_hint = clap::ValueHint::FilePath, default_value = "inside.html")]
    path: Option<std::path::PathBuf>,

    #[arg(short, long, value_hint = clap::ValueHint::Url, conflicts_with("path"))]
    url: Option<std::string::String>,

    /// Verbose logging (-v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
    Stub,
    Annotate,
}

#[derive(Debug)]
struct Stub {
    title: String,
    anchor: String,
    text: Vec<String>,
}

impl Stub {
    fn to_stub(&self) -> String {
        let a;
        if self.title.ends_with(")") {
            a = String::from(format!("function {} end", self.title))
        }else {
            a = String::from(format!("{} = nil", self.title))
        }
        a
    }
    fn to_lua(&self) -> String {
        String::from(
            format!("\n{}--- https://sdk.play.date/2.0.1/Inside%20Playdate.html#{}\n{}\n", self.text2comments(), self.anchor, self.to_stub())
        )
    }
    fn text2comments (&self) -> String {
        let mut s = String::new();
        for t in &self.text {
            s.push_str(&format!("--- {}\n---\n", t));
        }
        s
    }
}

fn fetch_or_file(args: &Args) -> String {
    let mut response = String::new();
    if args.url.is_some() {
        eprintln!("Fetching from {}", args.url.as_ref().unwrap());
        response = reqwest::blocking::get(
            args.url.as_ref().unwrap()
        )
        .unwrap()
        .text()
        .unwrap();
    }else {
        eprintln!("Reading from {}", args.path.as_ref().unwrap().display());
        let mut file = match File::open(args.path.as_ref().unwrap()) {
            Err(why) => panic!("couldn't open file: {}",  why),
            Ok(file) => file,
        };
        file.read_to_string(&mut response).unwrap();
    }
    response
}

fn main() {
    let args = Args::parse();
    let response = fetch_or_file(&args);

    let document = scraper::Html::parse_document(&response);
    let outer = Selector::parse("div.sect1>div.sectionbody>div.sect2").unwrap();

    // let selector1 = Selector::parse("div.sect1>div.sectionbody>div.sect2>div.sect3>div.item").unwrap();
    // let selector1 = Selector::parse("div.sect3>div.item").unwrap();
    let selector2 = Selector::parse("div.item").unwrap();

    let sel_title = Selector::parse("div.title").unwrap();
    let sel_content = Selector::parse("div.content").unwrap();
    let sel_paragraph = Selector::parse("div.paragraph").unwrap();
    let sel_p = Selector::parse("p").unwrap();

    let sel_admonition = Selector::parse("div.admonitionblock>table>tbody>tr>td.content").unwrap();
    let re_code = Regex::new(r"</?code>").unwrap();
    let re_em = Regex::new(r"</?em>").unwrap();
    let re_a = Regex::new(r"</?a[^>]*>").unwrap();
    let re_strong = Regex::new(r"</?strong>").unwrap();
    let html_tag = Regex::new(r"<[^>]*>").unwrap();

    let re_operator = Regex::new(r"(?:[\#\-\+\*\/]| \.\. )").unwrap();
    let re_brackets = Regex::new(r"[\[\]]").unwrap();
    let re_function = Regex::new(r"^(?:[\w][\w\d]*\.)*(?:[\w][\w\d]*)(?:[:.][\w][\w\d]*)").unwrap();

    let mut _poop = 0;
    let mut _last_class: String = "".to_string();
    let mut stubs : Vec<Stub> = Vec::new();


    for element in document.select(&outer) {
        // for d2 in element.select(&selector1).chain(element.select(&selector2)) {
        for d2 in element.select(&selector2) {
            _poop = _poop + 1;

            let anchor = d2.value().attr("id").unwrap_or("");
            let mut title = d2.select(&sel_title).next().unwrap().text().collect::<String>();

            let mut text: Vec<String> = Vec::new();
            for c in d2.select(&sel_content) {
                for div_p in c.select(&sel_paragraph) { // Paragraphs of the documentation
                    for p in div_p.select(&sel_p) {
                        text.push(p.text().collect::<String>());
                    }
                }
                for td in c.select(&sel_admonition) { // Add admonitions parapgraphs
                    let adm = td.inner_html();
                    let a1 = re_code.replace_all(&adm, "`");
                    let a2 = re_em.replace_all(&a1, "*");
                    let a3 = re_a.replace_all(&a2, "");
                    let a4 = re_strong.replace_all(&a3, "**");
                    let an = a4.trim().to_string();
                    if html_tag.is_match(&an) {
                        print!("WARN: Extra HTML tag in admonition: {}", an.to_string());
                    }
                    text.push(an.to_string());
                }
            }
            // This gets rid of the brackets (optional functional parameters) in the title
            title = re_brackets.replace_all(&title, "").to_string();

            if re_function.is_match(&title) {
                if title.contains("-") {
                    // println!("WARN: Function with dash in title: {}. Fixed as {}", title, title.replace("-", "_"));
                    title = title.replace("-", "_");
                }
                if title.contains(":") {
                    _last_class = title.split(":").next().unwrap_or("").to_string();
                }
            }
            // TODO: This does not show multiple overloaded functions with different type params
            if re_operator.is_match(&title) {
                text.push("# {title}".to_string());
                if title.starts_with("-") {
                    title = format!("{_last_class}:__unm()").to_string();
                } else if title.starts_with("#") {
                    title = format!("{_last_class}:__len()").to_string();
                } else if title.contains("-") {
                    title = format!("{_last_class}:__sub(other)").to_string();
                } else if title.contains("*") {
                    title = format!("{_last_class}:__mul(other)").to_string();
                } else if title.contains("/") {
                    title = format!("{_last_class}:__div(other)").to_string();
                } else if title.contains("+") {
                    title = format!("{_last_class}:__add(other)").to_string();
                } else if title.contains("..") {
                    title = format!("{_last_class}:__concat(other)").to_string();
                } else {
                    panic!("Unknown operator: {}", title);
                }
            }
            if anchor == "" {
                // println!("WARN: No id for: {}", title);
            }
            if title.contains("  ") { // Functions with multiple
                for t in title.split("  ") {
                    let stub = Stub { title: t.to_string().trim().to_owned(), anchor: anchor.to_string(), text: text.clone() };
                    stubs.push(stub)
                }
            }
            else {
                let stub = Stub { title: title, anchor: anchor.to_string(), text: text };
                stubs.push(stub)
            }
        }
    }
    if args.action == Action::Stub {
        println!("-- This file contains function stubs for autocompletion. DO NOT include it in your game.\n");
        for stub in &stubs {
            println!("{}", stub.to_stub());
        }
    } else if args.action == Action::Annotate {
        println!("-- This file contains function stubs for autocompletion. DO NOT include it in your game.\n");
        for stub in &stubs {
            println!("{}", stub.to_lua());
        }
    }
    // titles
    //     .zip(1..101)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));
}
