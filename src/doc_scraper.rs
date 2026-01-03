//! HTML documentation scraper for Playdate SDK
//!
//! This module extracts API documentation from the official Playdate SDK
//! HTML documentation and converts it to structured data.

use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeMap;
use std::sync::LazyLock;

use crate::parser::{Param, Statement};

/// A scraped function stub with documentation
#[derive(Debug, Clone)]
pub struct ScrapedFunction {
    pub name: String,
    pub anchor: String,
    pub params: Vec<Param>,
    pub returns: Vec<Param>,
    pub docs: Vec<String>,
}

impl ScrapedFunction {
    /// Generate the lua function definition string (used as map key)
    pub fn lua_def(&self) -> String {
        let param_names: Vec<&str> = self
            .params
            .iter()
            .map(|p| p.name.trim_end_matches('?'))
            .collect();
        format!("{}({})", self.name, param_names.join(", "))
    }

    /// Apply type information from parsed .luars statements
    pub fn apply_types(&mut self, statements: &BTreeMap<String, Statement>) {
        let key = self.lua_def();
        if let Some(Statement::Function(_, params, returns)) = statements.get(&key) {
            self.params = params.clone();
            self.returns = returns.clone();
        }
    }
}

/// Type of documentation item based on anchor prefix
#[derive(Debug, Clone, Copy, PartialEq)]
enum ItemType {
    Function,  // f-*
    Method,    // m-*
    Callback,  // c-*
    Table,     // t-*
    Variable,  // v-*
    Attribute, // a-*
    Unknown,
}

impl ItemType {
    fn from_anchor(anchor: &str) -> Self {
        match anchor.get(..2) {
            Some("f-") => ItemType::Function,
            Some("m-") => ItemType::Method,
            Some("c-") => ItemType::Callback,
            Some("t-") => ItemType::Table,
            Some("v-") => ItemType::Variable,
            Some("a-") => ItemType::Attribute,
            _ => ItemType::Unknown,
        }
    }

    fn from_class(class: &str) -> Self {
        if class.contains("function") {
            ItemType::Function
        } else if class.contains("method") {
            ItemType::Method
        } else if class.contains("callback") {
            ItemType::Callback
        } else if class.contains("table") {
            ItemType::Table
        } else if class.contains("variable") {
            ItemType::Variable
        } else if class.contains("attribute") {
            ItemType::Attribute
        } else {
            ItemType::Unknown
        }
    }

    fn is_function_like(&self) -> bool {
        matches!(
            self,
            ItemType::Function | ItemType::Method | ItemType::Callback | ItemType::Table
        )
    }
}

// Lazy static selectors and regexes
static SEL_ITEM: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse(concat!(
        "div.sect1>div.sectionbody>div.sect2>div.item",
        ",div.sect1>div.sectionbody>div.sect2>div.sect3>div.item",
        ",div.sect1>div.sectionbody>div.sect2>div.sect3>div.sect4>div.item",
        ",div.sect1>div.sectionbody>div.sect2>div.sect3>div.sect4>div.sect5>div.item",
    ))
    .unwrap()
});

static SEL_TITLE: LazyLock<Selector> = LazyLock::new(|| Selector::parse("div.title").unwrap());

static SEL_CONTENT: LazyLock<Selector> = LazyLock::new(|| Selector::parse("div.content").unwrap());

static SEL_P_TAG: LazyLock<Selector> = LazyLock::new(|| Selector::parse("p").unwrap());

static SEL_CODE_TAG: LazyLock<Selector> = LazyLock::new(|| Selector::parse("code").unwrap());

static SEL_PRE_TAG: LazyLock<Selector> = LazyLock::new(|| Selector::parse("pre").unwrap());

static SEL_ADMONITION: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table>tbody>tr>td.content").unwrap());

static RE_FUNC_SIG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^((?:[\w_][\w\d_]*\.)*[\w_][\w\d_]*[:.:][\w_][\w\d_]*|[\w_][\w\d_]*(?:\.[\w_][\w\d_]*)*)\s*\(([^)]*)\)").unwrap()
});

static RE_HTML_LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"</?a[^>]*>").unwrap());

static RE_EM_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<em[^>]*>").unwrap());

/// Scrape the Playdate SDK documentation HTML
pub fn scrape(
    html: &str,
    statements: &BTreeMap<String, Statement>,
) -> BTreeMap<String, ScrapedFunction> {
    let document = Html::parse_document(html);
    let mut result = BTreeMap::new();
    let overrides = load_overrides();
    let invalid_params = load_invalid_params();

    for element in document.select(&SEL_ITEM) {
        let anchor = element.value().attr("id").unwrap_or("");
        let class = element.value().attr("class").unwrap_or("");

        // Try anchor prefix first, fall back to class attribute
        let item_type = match ItemType::from_anchor(anchor) {
            ItemType::Unknown => ItemType::from_class(class),
            t => t,
        };

        if !item_type.is_function_like() {
            continue;
        }

        let title = extract_title(&element);
        let docs = extract_docs(&element);

        // Handle multi-function definitions (separated by double spaces)
        let titles: Vec<&str> = if title.contains("  ") {
            title.split("  ").map(|s| s.trim()).collect()
        } else {
            vec![title.as_str()]
        };

        for title in titles {
            if let Some(mut func) = parse_function_title(anchor, title, &overrides, &invalid_params)
            {
                func.docs = docs.clone();
                func.apply_types(statements);
                let key = func.lua_def();
                result.insert(key, func);
            }
        }
    }

    result
}

/// Extract the title text from an item element
fn extract_title(element: &ElementRef) -> String {
    element
        .select(&SEL_TITLE)
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default()
}

/// Recursively extract list items with proper indentation for nested lists
fn extract_list_items(list_el: &ElementRef, docs: &mut Vec<String>, depth: usize) {
    let indent = "  ".repeat(depth);

    // Find the direct ul child, then iterate its direct li children
    for child in list_el.children() {
        if let Some(ul) = ElementRef::wrap(child) {
            if ul.value().name() == "ul" {
                for li_node in ul.children() {
                    if let Some(li) = ElementRef::wrap(li_node) {
                        if li.value().name() != "li" {
                            continue;
                        }

                        // Get direct text content from <p> tag if present
                        let text = if let Some(p) = li
                            .children()
                            .filter_map(ElementRef::wrap)
                            .find(|el| el.value().name() == "p")
                        {
                            clean_html_text(&p.inner_html())
                        } else {
                            // Get only direct text nodes
                            li.children()
                                .filter_map(|c| c.value().as_text())
                                .map(|t| t.text.as_ref())
                                .collect::<String>()
                                .trim()
                                .to_string()
                        };

                        if !text.is_empty() {
                            docs.push(format!("{}* {}", indent, text));
                        }

                        // Check for nested ulist inside this li
                        for li_child in li.children() {
                            if let Some(nested) = ElementRef::wrap(li_child) {
                                let classes = nested.value().attr("class").unwrap_or("");
                                if classes.contains("ulist") {
                                    extract_list_items(&nested, docs, depth + 1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Extract documentation text from an item element, preserving document order
fn extract_docs(element: &ElementRef) -> Vec<String> {
    let mut docs = Vec::new();

    for content in element.select(&SEL_CONTENT) {
        // Process children in document order
        for child in content.children() {
            if let Some(child_el) = ElementRef::wrap(child) {
                let classes = child_el.value().attr("class").unwrap_or("");

                if classes.contains("paragraph") {
                    // Paragraph - extract inner p tag
                    if let Some(p) = child_el.select(&SEL_P_TAG).next() {
                        docs.push(clean_html_text(&p.inner_html()));
                    }
                } else if classes.contains("ulist") {
                    // Unordered list - extract list items with nested list support
                    extract_list_items(&child_el, &mut docs, 0);
                } else if classes.contains("listingblock") {
                    // Code block (with <code> tag)
                    for code in child_el.select(&SEL_CODE_TAG) {
                        docs.push("```".to_string());
                        for line in code.text().collect::<String>().lines() {
                            if !line.trim().is_empty() {
                                docs.push(line.to_string());
                            }
                        }
                        docs.push("```".to_string());
                    }
                } else if classes.contains("literalblock") {
                    // Literal block (with <pre> tag)
                    for pre in child_el.select(&SEL_PRE_TAG) {
                        docs.push("```".to_string());
                        for line in pre.text().collect::<String>().lines() {
                            if !line.trim().is_empty() {
                                docs.push(line.to_string());
                            }
                        }
                        docs.push("```".to_string());
                    }
                } else if classes.contains("admonitionblock") {
                    // Admonition blocks (caution, note, warning, etc.)
                    let prefix = "";
                    // let prefix = if classes.contains("caution") { "CAUTION: " } else { "" };
                    for adm in child_el.select(&SEL_ADMONITION) {
                        docs.push(format!("{}{}", prefix, clean_html_text(&adm.inner_html())));
                    }
                }
            }
        }
    }

    docs
}

/// Clean HTML text by converting tags to markdown
fn clean_html_text(text: &str) -> String {
    // Handle em tags with attributes (like <em class="constant">)
    let result = RE_EM_TAG.replace_all(text, "*");
    let result = result
        .replace("</em>", "*")
        .replace("<code>", "`")
        .replace("</code>", "`")
        .replace("<pre>", "`")
        .replace("</pre>", "`")
        .replace("<strong>", "**")
        .replace("</strong>", "**")
        .replace("\n", " ")
        .replace("<br>", "\n---\n---")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("<bundleid>", "{bundleid}")
        .replace("<text>", "{text}")
        .replace("<message>", "{message}");

    // Remove HTML links
    let result = RE_HTML_LINK.replace_all(&result, "");
    result.trim().to_string()
}

/// Parse a function title into a ScrapedFunction
fn parse_function_title(
    anchor: &str,
    title: &str,
    overrides: &BTreeMap<String, FunctionOverride>,
    invalid_params: &BTreeMap<String, String>,
) -> Option<ScrapedFunction> {
    // Check for overrides first
    if let Some(override_) = overrides.get(anchor) {
        let params: Vec<Param> = override_
            .parameters
            .iter()
            .map(|p| Param {
                name: p.clone(),
                typ: "any".to_string(),
            })
            .collect();
        return Some(ScrapedFunction {
            name: override_.name.clone(),
            anchor: anchor.to_string(),
            params,
            returns: Vec::new(),
            docs: Vec::new(),
        });
    }

    // Parse the function signature
    let caps = RE_FUNC_SIG.captures(title)?;
    let name = caps.get(1)?.as_str().to_string();
    let params_str = caps.get(2)?.as_str();

    let mut params = Vec::new();
    let mut is_optional = false;

    if !params_str.trim().is_empty() {
        for p in params_str.split(',') {
            let mut param_name = p.trim().to_string();

            // Track optional params (denoted by [])
            if param_name.contains('[') {
                is_optional = true;
            }

            // Clean brackets
            param_name = param_name.replace(['[', ']'], "").trim().to_string();

            // Mark as optional
            if is_optional && !param_name.ends_with('?') {
                param_name.push('?');
            }

            // Apply invalid param fixes
            let clean_name = param_name.trim_end_matches('?');
            if let Some(fixed) = invalid_params.get(clean_name) {
                param_name = if param_name.ends_with('?') {
                    format!("{}?", fixed)
                } else {
                    fixed.clone()
                };
            }

            params.push(Param {
                name: param_name.clone(),
                typ: "any".to_string(),
            });

            // Stop after variadic
            if param_name.starts_with("...") {
                break;
            }
        }
    }

    Some(ScrapedFunction {
        name,
        anchor: anchor.to_string(),
        params,
        returns: Vec::new(),
        docs: Vec::new(),
    })
}

/// Function override from TOML config
#[derive(Debug, Clone, serde::Deserialize)]
struct FunctionOverride {
    name: String,
    parameters: Vec<String>,
}

/// Load function overrides from TOML
fn load_overrides() -> BTreeMap<String, FunctionOverride> {
    static TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/RenameFn.toml"));
    toml::from_str(TOML).unwrap_or_default()
}

/// Load invalid parameter name fixes from TOML
fn load_invalid_params() -> BTreeMap<String, String> {
    static TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/Invalid.toml"));
    toml::from_str(TOML).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_html_text() {
        assert_eq!(
            clean_html_text("<code>foo</code> and <em>bar</em>"),
            "`foo` and *bar*"
        );
    }

    #[test]
    fn test_parse_function_title() {
        let overrides = BTreeMap::new();
        let invalid = BTreeMap::new();

        let func =
            parse_function_title("f-test", "playdate.foo(a, b, c)", &overrides, &invalid).unwrap();
        assert_eq!(func.name, "playdate.foo");
        assert_eq!(func.params.len(), 3);
        assert_eq!(func.params[0].name, "a");
    }

    #[test]
    fn test_parse_function_with_optional_params() {
        let overrides = BTreeMap::new();
        let invalid = BTreeMap::new();

        let func = parse_function_title("f-test", "foo(a, [b, c])", &overrides, &invalid).unwrap();
        assert_eq!(func.params.len(), 3);
        assert_eq!(func.params[0].name, "a");
        assert_eq!(func.params[1].name, "b?");
        assert_eq!(func.params[2].name, "c?");
    }

    #[test]
    fn test_parse_method() {
        let overrides = BTreeMap::new();
        let invalid = BTreeMap::new();

        let func =
            parse_function_title("m-test", "Sprite:draw(x, y)", &overrides, &invalid).unwrap();
        assert_eq!(func.name, "Sprite:draw");
        assert_eq!(func.params.len(), 2);
    }
}
