mod args;
mod docs;
mod stub;
use stub::Stub;

use regex::Regex;
use scraper::Selector;
// use toml::Table;
// use serde_json::{Result, Value};


fn main() {
    let (args, response) = crate::args::setup();

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

    let re_operator = Regex::new(r"(?:[\#\-\+\*\/]| \.\. )").unwrap();
    let re_optional = Regex::new(r"\(.*\[.*\)").unwrap();  // function signature with brackets (optional params)
    let re_brackets = Regex::new(r"[\[\]]").unwrap();  // any brackets (used with replace_all)
    let re_function = Regex::new(r"^(?:[\w][\w\d]*\.)*(?:[\w][\w\d]*)(?:[:.][\w][\w\d]*)").unwrap();

    let mut _poop = 0;
    let mut _last_class: String = "".to_string();
    let mut stubs : Vec<Stub> = Vec::new();


    for element in document.select(&outer) {
        // for d2 in element.select(&selector1).chain(element.select(&selector2)) {
        for d2 in element.select(&selector2) {
            _poop = _poop + 1;

            let anchor: &str = d2.value().attr("id").unwrap_or("");
            let mut title: String = d2.select(&sel_title).next().unwrap().text().collect::<String>();

            if anchor == "" {
                eprintln!("WARN: No id for: {}", title);
            } else if title == "print(string)" {
                continue;
            }

            let mut text: Vec<String> = Vec::new();

            for c in d2.select(&sel_content) {
                for div_p in c.select(&sel_paragraph) { // Paragraphs of the documentation
                    for p in div_p.select(&sel_p) {
                        text.push(p.text().collect::<String>());
                    }
                }
                for td in c.select(&sel_admonition) { // Add admonitions parapgraphs
                    let adm = td.inner_html();
                    text.push(docs::clean_admonition(adm));
                }
            }
            // This gets rid of the brackets (optional functional parameters) in the title
            if re_optional.is_match(&title) {
                title = re_brackets.replace_all(&title, "").to_string();
            }
            //TODO: Fix this to correctly capture params and duplicated type params.
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
                } else if title.contains("[") {
                    title = format!("{_last_class}:__index(n)").to_string();
                } else if title.ends_with("({ row1, row2, row3, row4, row5, row6, row7, row8 })") {
                    title = format!("{}(row_tbl)", title.split("(").next().unwrap().to_string());
                } else {
                    panic!("Unhandled operator: {}", title);
                }
            }
            if title.contains("  ") { // Functions with multiple
                for t in title.split("  ") {
                    let fname: String;
                    let params: Vec<String>;
                    (fname, params) = docs::params_from_title(&t.trim().to_string());
                    let stub = Stub { title: fname, anchor: anchor.to_string(), params: params.clone(), text: text.clone() };
                    stubs.push(stub)
                }
            } else if title.contains("(") {
                let fname: String;
                let params: Vec<String>;
                (fname, params) = docs::params_from_title(&title);
                let stub = Stub { title: fname, anchor: anchor.to_string(), params: params.clone(), text: text.clone() };
                stubs.push(stub)
            } else {
                // TODO: Add this as a stub!
                eprintln!("VARIABLE: {title}");
            }

            // _last_class is context for the next loop. So if the title is missing a name (e.g. "p + p") we can infer it.
            if re_function.is_match(&title) && title.contains(":") {
                _last_class = title.split(":").next().unwrap_or("").to_string();
            }
        }
    }
    stub::generate(&stubs, args.action);
    // titles
    //     .zip(1..101)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));
}
