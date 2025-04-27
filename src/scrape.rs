use std::collections::BTreeMap;

use regex::Regex;
use scraper::{CaseSensitivity, Selector};

use crate::fixes::{apply_fn_types, clean_code, clean_text};
use crate::luars::LuarsStatement;
use crate::stub::Stub;

enum AnchorType {
    Function,
    Method,
    Table,
    Callback,
    Variable,
    Attribute,
    Unkown,
}

impl AnchorType {
    pub fn new(anchor: &str) -> Self {
        match anchor {
            "lua-sample" => AnchorType::Function,
            "f-metadata" => AnchorType::Variable,
            _ => match anchor.get(0..2).unwrap_or("") {
                "v-" => AnchorType::Variable,
                "f-" => AnchorType::Function,
                "m-" => AnchorType::Method,
                "t-" => AnchorType::Table,
                "c-" => AnchorType::Callback,
                "a-" => AnchorType::Attribute,
                _ => AnchorType::Unkown,
            },
        }
    }
}

pub fn scrape(
    response: String,
    statements: &BTreeMap<String, LuarsStatement<'_>>,
) -> BTreeMap<String, Stub> {
    let document = scraper::Html::parse_document(&response);
    let outer = Selector::parse(concat!(
        "div.sect1>div.sectionbody>div.sect2>div.item",
        ",div.sect1>div.sectionbody>div.sect2>div.sect3>div.item",
        ",div.sect1>div.sectionbody>div.sect2>div.sect3>div.sect4>div.item",
        ",div.sect1>div.sectionbody>div.sect2>div.sect3>div.sect4>div.sect5>div.item",
    ))
    .unwrap();
    let sel_title = Selector::parse("div.title").unwrap();
    let sel_content = Selector::parse("div.content").unwrap();

    let sel_docs = Selector::parse(
        vec![
            "div.paragraph",
            "div.ulist",
            "div.admonitionblock",
            "div.literalblock",
            "div.listingblock",
        ]
        .join(",")
        .as_str(),
    )
    .unwrap();
    let sel_docs_text = Selector::parse("p").unwrap();
    let sel_docs_list = Selector::parse("ul>li").unwrap();
    let sel_docs_coderay = Selector::parse("code").unwrap();
    let sel_docs_admonition = Selector::parse("table>tbody>tr>td.content").unwrap();

    let re_function = Regex::new(r"^(?:[\w][\w\d]*\.)*(?:[\w][\w\d]*)(?:[:.][\w][\w\d]*)").unwrap();

    let mut _poop = 0;
    let mut _last_class: String = "".to_string();
    let mut stubs: BTreeMap<String, Stub> = BTreeMap::new();

    for element in document.select(&outer) {
        _poop = _poop + 1;

        let anchor: &str = element.value().attr("id").unwrap_or("");
        let title: String = element
            .select(&sel_title)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        if anchor == "" {
            // eprintln!("WARN: Docs missing anchor for: {}", title);
        }

        let mut text: Vec<String> = Vec::new();
        for c in element.select(&sel_content) {
            // Main Paragraphs of text documentation
            for div in c.select(&sel_docs) {
                let dv = div.value();
                if dv.has_class("paragraph", CaseSensitivity::CaseSensitive) {
                    div.select(&sel_docs_text).for_each(|p| {
                        let t = clean_text(p.inner_html());
                        text.push(t);
                    });
                } else if dv.has_class("listingblock", CaseSensitivity::CaseSensitive) {
                    text.push("```".to_string());
                    div.select(&sel_docs_coderay).for_each(|d| {
                        let lines = clean_code(d.text().collect::<String>());
                        for l in lines {
                            text.push(l);
                        }
                    });
                    text.push("```".to_string());
                } else if dv.has_class("ulist", CaseSensitivity::CaseSensitive) {
                    div.select(&sel_docs_list).for_each(|li| {
                        let mut t: String = clean_text(li.text().collect::<String>());
                        t.insert_str(0, "* ");
                        text.push(t);
                    });
                } else if dv.has_class("admonitionblock", CaseSensitivity::CaseSensitive) {
                    div.select(&sel_docs_admonition).for_each(|td| {
                        let adm = clean_text(td.inner_html());
                        text.push(adm);
                    });
                } else if dv.has_class("literalblock", CaseSensitivity::CaseSensitive) {
                    div.select(&sel_content).for_each(|d| {
                        // TODO: This clobbers line breaks. pre should get ``` fencing.
                        let litb = clean_text(d.inner_html());
                        text.push(litb);
                    });
                } else {
                    eprintln!("skipping {:?}", div.value())
                }
            }
        }

        match AnchorType::new(anchor) {
            AnchorType::Attribute => {
                // eprintln!("ATTRIBUTE {} {} {:?} ", anchor, title, text);
            }
            AnchorType::Variable => {
                if title.contains("  ") {
                    // eprintln!("MULTILINE_VARIABLE {} {} {:?} ", anchor, title, text);
                }
                // eprintln!("VARIABLE {} {} {:?} ", anchor, title, text);
            }
            AnchorType::Table
            | AnchorType::Method
            | AnchorType::Callback
            | AnchorType::Function => {
                // Functions with multiple (e.g. playdate.easingFunctions.*, )
                if title.contains("  ") {
                    if !(anchor.starts_with("m-") || anchor.starts_with("f-")) {}
                    for t in title.split("  ") {
                        let mut stub =
                            apply_fn_types(&anchor.to_string(), &t.trim().to_string(), &text);
                        stub = stub.annotate(statements);
                        let key = stub.lua_def();
                        if let Some(_val) = stubs.insert(key.clone(), Stub::Function(stub)) {
                            eprintln!("Duplicate stub {} (in multi-def)", key)
                        }
                    }
                // Other functions
                } else {
                    let stub =
                        apply_fn_types(&anchor.to_string(), &title, &text).annotate(statements);
                    let lua_def = stub.lua_def();
                    if let Some(_val) = stubs.insert(lua_def.clone(), Stub::Function(stub)) {
                        eprintln!("Duplicate stub {} (function)", lua_def)
                    }
                }
            }
            _ => {
                eprintln!("UNKNOWN: {anchor} {title}");
            }
        }

        // _last_class is context for the next loop. So if the title is missing a name (e.g. "p + p") we can infer it.
        if re_function.is_match(&title) && title.contains(":") {
            _last_class = title.split(":").next().unwrap_or("").to_string();
        }
    }
    stubs
}
