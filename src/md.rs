//! Markdown documentation output

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::luars::{Param, Statement};
use crate::output::split_function_name;
use crate::scraper::ScrapedFunction;

fn namespace_to_path(ns: &str) -> PathBuf {
    if ns == "json" || ns.starts_with("json.") {
        PathBuf::from("json.md")
    } else if ns == "playdate.accelerometer" || ns.starts_with("playdate.accelerometer.") {
        PathBuf::from("playdate/accelerometer.md")
    } else if ns == "playdate.inputHandlers" || ns.starts_with("playdate.inputHandlers.") {
        PathBuf::from("playdate/inputhandlers.md")
    } else if ns == "playdate.file"
        || ns == "playdate.file.file"
        || ns.starts_with("playdate.file.")
    {
        PathBuf::from("playdate/file.md")
    } else if ns == "playdate.menu" || ns.starts_with("playdate.menu.") {
        PathBuf::from("playdate/menu.md")
    } else if ns == "playdate.network" || ns.starts_with("playdate.network.") {
        PathBuf::from("playdate/network.md")
    } else if ns == "playdate.pathfinder" || ns.starts_with("playdate.pathfinder.") {
        PathBuf::from("playdate/pathfinder.md")
    } else if ns == "playdate.ui" || ns.starts_with("playdate.ui.") {
        PathBuf::from("playdate/ui.md")
    } else if ns == "playdate.graphics.animation" || ns.starts_with("playdate.graphics.animation.")
    {
        PathBuf::from("playdate/graphics/animation.md")
    } else if ns == "playdate.graphics.font" || ns.starts_with("playdate.graphics.font.") {
        PathBuf::from("playdate/graphics/font.md")
    } else if ns == "playdate.math.logic" || ns.starts_with("playdate.math.logic.") {
        PathBuf::from("playdate/math.md")
    } else if ns == "playdate.scoreboards" || ns.starts_with("playdate.scoreboards.") {
        PathBuf::from("playdate/scoreboards.md")
    } else if ns == "playdate.system" || ns.starts_with("playdate.system.") {
        PathBuf::from("playdate/profiling.md")
    } else if ns == "playdate.time" || ns.starts_with("playdate.time.") {
        PathBuf::from("playdate/time.md")
    } else if ns == "playdate" {
        PathBuf::from("playdate.md")
    } else if ns == "Object" || ns.starts_with("Object.") {
        PathBuf::from("class.md")
    } else if ns.starts_with("playdate.") {
        PathBuf::from(format!("{}.md", ns.replace('.', "/")))
    } else if ns.is_empty() {
        PathBuf::from("other.md")
    } else {
        PathBuf::from(format!("{}.md", ns.replace('.', "/")))
    }
}

fn format_signature_parts(name: &str, params: &[Param], returns: &[Param]) -> String {
    let params = params
        .iter()
        .map(|p| format!("{}: {}", p.name.trim_end_matches('?'), p.typ))
        .collect::<Vec<_>>()
        .join(", ");
    let returns = if returns.is_empty() {
        "nil".to_string()
    } else if returns.len() == 1 {
        returns[0].typ.clone()
    } else {
        let types = returns
            .iter()
            .map(|r| r.typ.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", types)
    };
    format!("{}({}): {}", name, params, returns)
}

#[derive(Clone)]
struct MdFunction {
    name: String,
    params: Vec<Param>,
    returns: Vec<Param>,
    docs: Vec<String>,
}

pub fn write_markdown_docs(
    scraped: &BTreeMap<String, ScrapedFunction>,
    statements: &BTreeMap<String, Statement>,
    out_dir: &Path,
) -> io::Result<()> {
    let mut functions_by_file: BTreeMap<PathBuf, Vec<MdFunction>> = BTreeMap::new();
    let mut classes_by_file: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    let mut local_parent: HashMap<String, String> = HashMap::new();

    for stmt in statements.values() {
        match stmt {
            Statement::Global(name, parent, fields) => {
                if fields.is_empty() {
                    continue;
                }
                let file = if name == "kTextAlignment" {
                    PathBuf::from("playdate/graphics/font.md")
                } else {
                    namespace_to_path(name)
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
            Statement::Local(name, parent, fields) => {
                if fields.is_empty() {
                    if !parent.is_empty() {
                        local_parent.insert(name.clone(), parent.clone());
                    }
                    continue;
                }
                if !parent.is_empty() {
                    local_parent.insert(name.clone(), parent.clone());
                }
                let file = if name == "_InputHandler" {
                    PathBuf::from("playdate/inputhandlers.md")
                } else if name == "_DateTime" || name == "_ModTime" {
                    PathBuf::from("playdate/time.md")
                } else if name.starts_with("_SoundTrack") {
                    PathBuf::from("playdate/sound/track.md")
                } else if name.starts_with("_ScoreBoard") || name == "_ServerStatus" {
                    PathBuf::from("playdate/scoreboards.md")
                } else if name == "_Metadata" {
                    PathBuf::from("playdate.md")
                } else if name == "_PowerStatus" {
                    PathBuf::from("playdate/device.md")
                } else if name == "_SystemInfo" {
                    PathBuf::from("playdate.md")
                } else if name == "_SystemStats" {
                    PathBuf::from("playdate/profiling.md")
                } else if name == "_NewClass" {
                    PathBuf::from("class.md")
                } else if name == "_SoundControlEvent" {
                    PathBuf::from("playdate/sound/controlsignal.md")
                } else if name == "_SpriteCollisionData" || name == "_SpriteCollisionInfo" {
                    PathBuf::from("playdate/graphics/sprite.md")
                } else if !parent.is_empty() {
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
        let name_lower = func.name.to_lowercase();
        let file = if name_lower.contains("button") || name_lower.contains("crank") {
            PathBuf::from("playdate/input.md")
        } else if name_lower.contains("accelerometer") {
            PathBuf::from("playdate/accelerometer.md")
        } else if name_lower.starts_with("playdate.math.logic.") {
            PathBuf::from("playdate/math.md")
        } else if name_lower.starts_with("playdate.graphics.") && name_lower.contains("font")
            || name_lower.contains("playdate.graphics.imagewithtext")
            || name_lower.contains("playdate.graphics.gettextsizeformaxwidth")
            || name_lower.contains("playdate.graphics.gettextsize")
            || name_lower.contains("playdate.graphics.getsystemfont")
            || name_lower.contains("playdate.graphics.getlocalizedtext")
            || name_lower.contains("playdate.graphics.getfonttracking")
            || name_lower.contains("playdate.graphics.getfont")
            || name_lower.contains("playdate.graphics.drawtext")
            || name_lower.contains("playdate.graphics.drawlocalizedtext")
        {
            PathBuf::from("playdate/graphics/font.md")
        } else if name_lower.contains("playdate.graphics.stencil") {
            PathBuf::from("playdate/graphics/stencil.md")
        } else if name_lower.contains("playdate.graphics.generateqrcodesync")
            || name_lower.contains("playdate.graphics.generateqrcode")
        {
            PathBuf::from("playdate/graphics/qrcode.md")
        } else if name_lower.contains("playdate.graphics.imagesizeatpath")
            || name_lower.contains("playdate.graphics.checkalphacollision")
        {
            PathBuf::from("playdate/graphics/image.md")
        } else if name_lower.contains("playdate.graphics.perlin")
            || name_lower.contains("playdate.graphics.perlinarray")
        {
            PathBuf::from("playdate/graphics/perlin.md")
        } else if func.name == "print"
            || func.name == "printTable"
            || name_lower.contains("setnewlineprinted")
        {
            PathBuf::from("print.md")
        } else if func.name == "where"
            || name_lower.contains("setcollectsgarbage")
            || name_lower.contains("setgcscaling")
            || func.name == "sample"
        {
            PathBuf::from("profiling.md")
        } else if name_lower.contains("clearconsole")
            || name_lower.contains("debugdraw")
            || name_lower.contains("keypressed")
            || name_lower.contains("keyreleased")
        {
            PathBuf::from("playdate/simulator.md")
        } else if name_lower.contains("getstats") || name_lower.contains("setstatsinterval") {
            PathBuf::from("playdate/profiling.md")
        } else if name_lower.contains("getflipped")
            || name_lower.contains("getsystemlanguage")
            || name_lower.contains("getreduceflashing")
        {
            PathBuf::from("playdate/settings.md")
        } else if name_lower.contains("getpowerstatus")
            || name_lower.contains("getbatteryvoltage")
            || name_lower.contains("getbatterypercentage")
            || name_lower.contains("setautolockdisabled")
            || name_lower.contains("devicedidunlock")
            || name_lower.contains("devicewilllock")
            || name_lower.contains("devicewillsleep")
            || name_lower.contains("gamewillpause")
        {
            PathBuf::from("playdate/device.md")
        } else if name_lower.contains("mirrorended") || name_lower.contains("mirrorstarted") {
            PathBuf::from("playdate/mirror.md")
        } else if name_lower.contains("gamewill")
            || name_lower.contains("devicewill")
            || name_lower.contains("devicedid")
            || name_lower.contains("serialmessagereceived")
            || func.name.starts_with("playdate.update")
            || func.name.starts_with("playdate.stop")
            || func.name.starts_with("playdate.start")
            || func.name.starts_with("playdate.wait")
            || func.name.starts_with("playdate.restart")
        {
            PathBuf::from("playdate/lifecycle.md")
        } else if name_lower.contains("time") || name_lower.contains("epoch") {
            PathBuf::from("playdate/time.md")
        } else if name_lower.contains("menu") {
            PathBuf::from("playdate/menu.md")
        } else if let Some(parent) = local_parent.get(ns) {
            namespace_to_path(parent)
        } else {
            namespace_to_path(ns)
        };
        functions_by_file.entry(file).or_default().push(MdFunction {
            name: func.name.clone(),
            params: func.params.clone(),
            returns: func.returns.clone(),
            docs: func.docs.clone(),
        });
    }

    for stmt in statements.values() {
        let Statement::Function(name, params, returns) = stmt else {
            continue;
        };
        if scraped.contains_key(&stmt.lua_def()) {
            continue;
        }
        let (ns, _) = split_function_name(name);
        let name_lower = name.to_lowercase();
        let file = if name_lower.contains("button") || name_lower.contains("crank") {
            PathBuf::from("playdate/input.md")
        } else if name_lower.contains("accelerometer") {
            PathBuf::from("playdate/accelerometer.md")
        } else if name_lower.starts_with("playdate.math.logic.") {
            PathBuf::from("playdate/math.md")
        } else if name_lower.starts_with("playdate.graphics.") && name_lower.contains("font")
            || name_lower.contains("playdate.graphics.imagewithtext")
            || name_lower.contains("playdate.graphics.gettextsizeformaxwidth")
            || name_lower.contains("playdate.graphics.gettextsize")
            || name_lower.contains("playdate.graphics.getsystemfont")
            || name_lower.contains("playdate.graphics.getlocalizedtext")
            || name_lower.contains("playdate.graphics.getfonttracking")
            || name_lower.contains("playdate.graphics.getfont")
            || name_lower.contains("playdate.graphics.drawtext")
            || name_lower.contains("playdate.graphics.drawlocalizedtext")
        {
            PathBuf::from("playdate/graphics/font.md")
        } else if name_lower.contains("playdate.graphics.stencil") {
            PathBuf::from("playdate/graphics/stencil.md")
        } else if name_lower.contains("playdate.graphics.generateqrcodesync")
            || name_lower.contains("playdate.graphics.generateqrcode")
        {
            PathBuf::from("playdate/graphics/qrcode.md")
        } else if name_lower.contains("playdate.graphics.imagesizeatpath")
            || name_lower.contains("playdate.graphics.checkalphacollision")
        {
            PathBuf::from("playdate/graphics/image.md")
        } else if name_lower.contains("playdate.graphics.perlin")
            || name_lower.contains("playdate.graphics.perlinarray")
        {
            PathBuf::from("playdate/graphics/perlin.md")
        } else if name == "print"
            || name == "printTable"
            || name_lower.contains("setnewlineprinted")
        {
            PathBuf::from("print.md")
        } else if name == "class" {
            PathBuf::from("class.md")
        } else if name == "where"
            || name_lower.contains("setcollectsgarbage")
            || name_lower.contains("setgcscaling")
            || name == "sample"
        {
            PathBuf::from("profiling.md")
        } else if name_lower.contains("clearconsole")
            || name_lower.contains("debugdraw")
            || name_lower.contains("keypressed")
            || name_lower.contains("keyreleased")
        {
            PathBuf::from("playdate/simulator.md")
        } else if name_lower.contains("getstats") || name_lower.contains("setstatsinterval") {
            PathBuf::from("playdate/profiling.md")
        } else if name_lower.contains("getflipped")
            || name_lower.contains("getsystemlanguage")
            || name_lower.contains("getreduceflashing")
        {
            PathBuf::from("playdate/settings.md")
        } else if name_lower.contains("getpowerstatus")
            || name_lower.contains("getbatteryvoltage")
            || name_lower.contains("getbatterypercentage")
            || name_lower.contains("setautolockdisabled")
            || name_lower.contains("devicedidunlock")
            || name_lower.contains("devicewilllock")
            || name_lower.contains("devicewillsleep")
            || name_lower.contains("gamewillpause")
        {
            PathBuf::from("playdate/device.md")
        } else if name_lower.contains("mirrorended") || name_lower.contains("mirrorstarted") {
            PathBuf::from("playdate/mirror.md")
        } else if name_lower.contains("gamewill")
            || name_lower.contains("devicewill")
            || name_lower.contains("devicedid")
            || name_lower.contains("serialmessagereceived")
            || name.starts_with("playdate.update")
            || name.starts_with("playdate.stop")
            || name.starts_with("playdate.start")
            || name.starts_with("playdate.wait")
            || name.starts_with("playdate.restart")
        {
            PathBuf::from("playdate/lifecycle.md")
        } else if name_lower.contains("time") || name_lower.contains("epoch") {
            PathBuf::from("playdate/time.md")
        } else if name_lower.contains("menu") {
            PathBuf::from("playdate/menu.md")
        } else if let Some(parent) = local_parent.get(ns) {
            namespace_to_path(parent)
        } else {
            namespace_to_path(ns)
        };
        functions_by_file.entry(file).or_default().push(MdFunction {
            name: name.clone(),
            params: params.clone(),
            returns: returns.clone(),
            docs: Vec::new(),
        });
    }

    let mut all_files: BTreeMap<PathBuf, ()> = BTreeMap::new();
    for key in functions_by_file.keys() {
        all_files.insert(key.clone(), ());
    }
    for key in classes_by_file.keys() {
        all_files.insert(key.clone(), ());
    }

    let all_paths: Vec<PathBuf> = all_files.keys().cloned().collect();
    for rel_path in all_paths {
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
            let mut order: Vec<String> = Vec::new();
            let mut grouped: BTreeMap<String, (Vec<String>, Vec<String>)> = BTreeMap::new();

            for func in funcs {
                if !grouped.contains_key(&func.name) {
                    order.push(func.name.clone());
                }
                let sig = format_signature_parts(&func.name, &func.params, &func.returns);
                let entry = grouped
                    .entry(func.name.clone())
                    .or_insert_with(|| (Vec::new(), Vec::new()));
                if !entry.0.contains(&sig) {
                    entry.0.push(sig);
                }
                if entry.1.is_empty() && !func.docs.is_empty() {
                    entry.1 = func.docs.clone();
                }
            }

            for name in order {
                if let Some((sigs, docs)) = grouped.get(&name) {
                    lines.push(format!("### {}", name));
                    lines.push(String::new());
                    lines.push("```lua".to_string());
                    for sig in sigs {
                        lines.push(sig.clone());
                    }
                    lines.push("```".to_string());
                    if docs.is_empty() {
                        lines.push(String::new());
                    } else {
                        lines.push(String::new());
                        for doc in docs {
                            lines.push(doc.clone());
                        }
                        lines.push(String::new());
                    }
                }
            }
        }

        if let Some(classes) = classes_by_file.get(&rel_path) {
            if !classes.is_empty() {
                lines.push("## Classes".to_string());
                lines.push(String::new());
                for class_block in classes {
                    let class_name = class_block
                        .lines()
                        .find_map(|line| line.strip_prefix("---@class "))
                        .map(|s| s.split(':').next().unwrap_or(s).trim().to_string())
                        .unwrap_or_else(|| "Class".to_string());
                    lines.push(format!("### {}", class_name));
                    lines.push(String::new());
                    lines.push("```lua".to_string());
                    lines.push(class_block.clone());
                    lines.push("```".to_string());
                    lines.push(String::new());
                }
            }
        }

        let base_dir = rel_path.with_extension("");
        let mut sub_sections = Vec::new();
        for key in all_files.keys() {
            if key.starts_with(&base_dir) && key != &rel_path {
                if let Ok(stripped) = key.strip_prefix(&base_dir) {
                    let mut comps = stripped.components();
                    if matches!(comps.next(), Some(std::path::Component::Normal(_)))
                        && comps.next().is_none()
                    {
                        sub_sections.push(key.clone());
                    }
                }
            }
        }
        if !sub_sections.is_empty() {
            let current_dir = rel_path.parent().unwrap_or(Path::new(""));
            lines.push("## See Also:".to_string());
            lines.push(String::new());
            for sub in sub_sections {
                let link = if current_dir.as_os_str().is_empty() {
                    sub.to_string_lossy().to_string()
                } else if let Ok(stripped) = sub.strip_prefix(current_dir) {
                    stripped
                        .to_string_lossy()
                        .trim_start_matches('/')
                        .to_string()
                } else {
                    sub.to_string_lossy().to_string()
                };
                let label = sub
                    .with_extension("")
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| link.clone());
                lines.push(format!("- [{}]({})", label, link));
            }
            lines.push(String::new());
        }

        let full_path = out_dir.join(rel_path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, lines.join("\n"))?;
    }

    write_agents_index(&all_files, out_dir)?;

    Ok(())
}

fn write_agents_index(all_files: &BTreeMap<PathBuf, ()>, out_dir: &Path) -> io::Result<()> {
    let mut root_files: Vec<String> = Vec::new();
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for path in all_files.keys() {
        let comps: Vec<String> = path
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .collect();
        match comps.len() {
            1 => root_files.push(comps[0].clone()),
            2 => {
                let dir = comps[0].clone();
                let file = comps[1].clone();
                groups.entry(dir).or_default().push(file);
            }
            _ => {
                let dir = format!("{}/{}", comps[0], comps[1]);
                let file = comps[2..].join("/");
                groups.entry(dir).or_default().push(file);
            }
        }
    }

    root_files.sort();
    for files in groups.values_mut() {
        files.sort();
    }

    let mut lines = Vec::new();
    lines.push("[PlaydateSDK Docs Index]|root: ./.playdate-docs".to_string());
    lines.push(
        "|IMPORTANT: Prefer retrieval-led reasoning over pre-training-led reasoning".to_string(),
    );
    if !root_files.is_empty() {
        lines.push(format!("|{{{}}}", root_files.join(",")));
    }
    for (dir, files) in groups {
        lines.push(format!("|{}/{{{}}}", dir, files.join(",")));
    }

    let agents_path = out_dir.join("AGENTS.md");
    fs::write(agents_path, lines.join("\n"))?;
    Ok(())
}
