use indexmap::IndexMap;

use crate::config::CLASS;


pub fn print_class_defs() {
    for (class_name, fields) in CLASS.iter() {
        println!("{}", class_str(class_name, fields));
    }
}


fn class_str(class_name: &str, fields: &IndexMap<String, String>) -> String{
    let mut s: Vec<String> = Vec::new();
    s.push(format!("---@class {class_name}"));
    for (field, type_) in fields.iter() {
        s.push(format!("---@field {} {}", field, type_));
    }
    // Use of "local" means that if you load this file in a Lua interpretter
    // you aren't polluting the global namespace.
    s.push(format!("local {class_name} = {{}}")); // `{{}}` becomes `{}`
    s.push("".to_string());
    s.join("\n")
}
