//! Multi-file LuaCATS stub output

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::luars::Statement;
use crate::output::{
    append_compact_types, format_stub, function_type, generate_class, generate_compact_global,
    generate_function, split_function_name,
};

fn namespace(name: &str) -> &str {
    if let Some(pos) = name.rfind(':') {
        &name[..pos]
    } else if let Some(pos) = name.rfind('.') {
        &name[..pos]
    } else {
        ""
    }
}

fn table_file(name: &str) -> PathBuf {
    if name == "json" || name.starts_with("json.") {
        PathBuf::from("json.lua")
    } else if name == "playdate" {
        PathBuf::from("playdate.lua")
    } else if name.starts_with("playdate.") {
        PathBuf::from(format!("{}.lua", name.replace('.', "/")))
    } else {
        PathBuf::from("other.lua")
    }
}

struct FileBlocks {
    classes: Vec<Vec<String>>,
    functions: Vec<Vec<String>>,
}

/// Multi-file stub generator output
pub struct MultiStubOutput {
    files: BTreeMap<PathBuf, FileBlocks>,
    compact: bool,
}

impl MultiStubOutput {
    pub fn from_statements(statements: &BTreeMap<String, Statement>, compact: bool) -> Self {
        let mut files: BTreeMap<PathBuf, FileBlocks> = BTreeMap::new();
        let mut class_names: HashSet<String> = HashSet::new();

        for stmt in statements.values() {
            match stmt {
                Statement::Global(name, _, _) | Statement::Local(name, _, _) => {
                    class_names.insert(name.clone());
                }
                _ => {}
            }
        }

        if compact {
            let mut globals = HashSet::new();
            let mut functions_by_ns: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();
            let mut top_level: Vec<(String, Vec<String>)> = Vec::new();

            for stmt in statements.values() {
                if let Statement::Global(name, _, _) = stmt {
                    globals.insert(name.clone());
                }
            }

            for stmt in statements.values() {
                if let Statement::Function(name, params, returns) = stmt {
                    let (ns, field) = split_function_name(name);
                    if ns.is_empty() || !globals.contains(ns) {
                        append_compact_types(&mut top_level, name, function_type(params, returns));
                    } else {
                        let entry = functions_by_ns.entry(ns.to_string()).or_default();
                        append_compact_types(entry, field, function_type(params, returns));
                    }
                }
            }

            for stmt in statements.values() {
                match stmt {
                    Statement::Global(name, parent, fields) => {
                        let file = table_file(name);
                        let funcs = functions_by_ns
                            .get(name)
                            .map(|v| v.as_slice())
                            .unwrap_or(&[]);
                        let block = generate_compact_global(name, parent, fields, funcs);
                        files
                            .entry(file)
                            .or_insert_with(|| FileBlocks {
                                classes: Vec::new(),
                                functions: Vec::new(),
                            })
                            .classes
                            .push(block);
                    }
                    Statement::Local(name, parent, fields) => {
                        let file = if !parent.is_empty() && class_names.contains(parent) {
                            table_file(parent)
                        } else {
                            PathBuf::from("other.lua")
                        };
                        let block = generate_class(name, parent, fields, "local ");
                        files
                            .entry(file)
                            .or_insert_with(|| FileBlocks {
                                classes: Vec::new(),
                                functions: Vec::new(),
                            })
                            .classes
                            .push(block);
                    }
                    Statement::Function(_, _, _) => {}
                }
            }
            for (name, types) in top_level {
                let mut block = Vec::new();
                for typ in types {
                    block.push(format!("---@type {}", typ));
                }
                block.push(format!("{} = nil,", name));
                files
                    .entry(PathBuf::from("other.lua"))
                    .or_insert_with(|| FileBlocks {
                        classes: Vec::new(),
                        functions: Vec::new(),
                    })
                    .functions
                    .push(block);
            }
        } else {
            for stmt in statements.values() {
                match stmt {
                    Statement::Global(name, parent, fields) => {
                        let file = table_file(name);
                        let block = generate_class(name, parent, fields, "");
                        files
                            .entry(file)
                            .or_insert_with(|| FileBlocks {
                                classes: Vec::new(),
                                functions: Vec::new(),
                            })
                            .classes
                            .push(block);
                    }
                    Statement::Local(name, parent, fields) => {
                        let file = if !parent.is_empty() && class_names.contains(parent) {
                            table_file(parent)
                        } else {
                            PathBuf::from("other.lua")
                        };
                        let block = generate_class(name, parent, fields, "local ");
                        files
                            .entry(file)
                            .or_insert_with(|| FileBlocks {
                                classes: Vec::new(),
                                functions: Vec::new(),
                            })
                            .classes
                            .push(block);
                    }
                    Statement::Function(name, params, returns) => {
                        let ns = namespace(name);
                        let file = if ns.is_empty() {
                            PathBuf::from("other.lua")
                        } else {
                            table_file(ns)
                        };
                        let block = generate_function(name, params, returns, None);
                        files
                            .entry(file)
                            .or_insert_with(|| FileBlocks {
                                classes: Vec::new(),
                                functions: Vec::new(),
                            })
                            .functions
                            .push(block);
                    }
                }
            }
        }

        MultiStubOutput { files, compact }
    }

    pub fn write_to_dir(&self, out_dir: &Path) -> io::Result<()> {
        for (rel_path, blocks) in &self.files {
            let mut lines = Vec::new();
            lines.extend(blocks.classes.clone());
            lines.extend(blocks.functions.clone());
            let content = if self.compact {
                format_stub_compact(&lines)
            } else {
                format_stub(&lines)
            };

            let full_path = out_dir.join(rel_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(full_path, content)?;
        }
        Ok(())
    }
}

fn format_stub_compact(lines: &[Vec<String>]) -> String {
    let mut out = Vec::new();
    out.push("---@meta".to_string());
    out.push(String::new());

    for block in lines {
        if !block.is_empty() {
            out.push(block.join("\n"));
            out.push(String::new());
        }
    }

    out.join("\n")
}
