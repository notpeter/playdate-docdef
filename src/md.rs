//! Markdown documentation output

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::luars::Statement;
use crate::output::split_function_name;
use crate::scraper::ScrapedFunction;

fn namespace_to_path(ns: &str) -> PathBuf {
    if ns == "json" || ns.starts_with("json.") {
        PathBuf::from("json.md")
    } else if ns == "playdate" {
        PathBuf::from("playdate.md")
    } else if ns.starts_with("playdate.") {
        PathBuf::from(format!("{}.md", ns.replace('.', "/")))
    } else if ns.is_empty() {
        PathBuf::from("other.md")
    } else {
        PathBuf::from(format!("{}.md", ns.replace('.', "/")))
    }
}

fn format_signature(func: &ScrapedFunction) -> String {
    let params = func
        .params
        .iter()
        .map(|p| format!("{}: {}", p.name.trim_end_matches('?'), p.typ))
        .collect::<Vec<_>>()
        .join(", ");
    let returns = if func.returns.is_empty() {
        "nil".to_string()
    } else if func.returns.len() == 1 {
        func.returns[0].typ.clone()
    } else {
        let types = func
            .returns
            .iter()
            .map(|r| r.typ.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", types)
    };
    format!("{}({}): {}", func.name, params, returns)
}

pub fn write_markdown_docs(
    scraped: &BTreeMap<String, ScrapedFunction>,
    statements: &BTreeMap<String, Statement>,
    out_dir: &Path,
) -> io::Result<()> {
    let mut functions_by_file: BTreeMap<PathBuf, Vec<&ScrapedFunction>> = BTreeMap::new();
    let mut classes_by_file: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    let mut local_parent: HashMap<String, String> = HashMap::new();

    for stmt in statements.values() {
        match stmt {
            Statement::Global(name, parent, fields) => {
                let (ns, _) = split_function_name(name);
                let file = namespace_to_path(ns);
                let mut lines = Vec::new();
                if parent.is_empty() {
                    lines.push(format!("---@class {}", name));
                } else {
                    lines.push(format!("---@class {} : {}", name, parent));
                }
                for field in fields {
                    if field.value.is_empty() {
                        lines.push(format!("---@field {} {}", field.name, field.typ));
                    } else {
                        lines.push(format!(
                            "---@field {} {} {}",
                            field.name, field.typ, field.value
                        ));
                    }
                }
                classes_by_file
                    .entry(file)
                    .or_default()
                    .push(lines.join("\n"));
            }
            Statement::Local(name, parent, fields) => {
                if !parent.is_empty() {
                    local_parent.insert(name.clone(), parent.clone());
                }
                let file = if !parent.is_empty() {
                    namespace_to_path(parent)
                } else {
                    PathBuf::from("other.md")
                };
                let mut lines = Vec::new();
                if parent.is_empty() {
                    lines.push(format!("---@class {}", name));
                } else {
                    lines.push(format!("---@class {} : {}", name, parent));
                }
                for field in fields {
                    if field.value.is_empty() {
                        lines.push(format!("---@field {} {}", field.name, field.typ));
                    } else {
                        lines.push(format!(
                            "---@field {} {} {}",
                            field.name, field.typ, field.value
                        ));
                    }
                }
                classes_by_file
                    .entry(file)
                    .or_default()
                    .push(lines.join("\n"));
            }
            Statement::Function(_, _, _) => {}
        }
    }

    for func in scraped.values() {
        let (ns, _) = split_function_name(&func.name);
        let file = if let Some(parent) = local_parent.get(ns) {
            namespace_to_path(parent)
        } else {
            namespace_to_path(ns)
        };
        functions_by_file.entry(file).or_default().push(func);
    }

    let mut all_files: BTreeMap<PathBuf, ()> = BTreeMap::new();
    for key in functions_by_file.keys() {
        all_files.insert(key.clone(), ());
    }
    for key in classes_by_file.keys() {
        all_files.insert(key.clone(), ());
    }

    for (rel_path, _) in all_files {
        let mut lines = Vec::new();
        let title = if rel_path == PathBuf::from("other.md") {
            "other".to_string()
        } else {
            rel_path
                .with_extension("")
                .to_string_lossy()
                .replace('/', ".")
        };
        lines.push(format!("# {}", title));
        lines.push(String::new());

        lines.push("## Functions".to_string());
        lines.push(String::new());
        if let Some(funcs) = functions_by_file.get(&rel_path) {
            for func in funcs {
                lines.push(format!("### {}", func.name));
                lines.push(String::new());
                lines.push("```lua".to_string());
                lines.push(format_signature(func));
                lines.push("```".to_string());
                lines.push(String::new());
                for doc in &func.docs {
                    lines.push(doc.clone());
                }
                lines.push(String::new());
            }
        }

        lines.push("## Classes".to_string());
        lines.push(String::new());
        if let Some(classes) = classes_by_file.get(&rel_path) {
            for class_block in classes {
                lines.push("```lua".to_string());
                lines.push(class_block.clone());
                lines.push("```".to_string());
                lines.push(String::new());
            }
        }

        let full_path = out_dir.join(rel_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, lines.join("\n"))?;
    }

    Ok(())
}
