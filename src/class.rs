use indexmap::IndexMap;

pub fn print_class_defs() {
    for (class_def, fields) in CLASS.iter() {
        println!("{}", class_str(class_def, fields));
    }
}


fn class_str(class_name: &str, fields: &IndexMap<String, String>) -> String{
    let mut s: Vec<String> = Vec::new();
    s.push(format!("---@class {class_name}"));
    for (field, type_) in fields.iter() {
        s.push(format!("---@field {} {}", field, type_));
    }
    let cname: &str = if class_name.contains(":") {
        class_name.split(":").next().unwrap().trim()
    } else {
        class_name
    };

    s.push(format!("{cname} = {{}}")); // `{{}}` becomes `{}`
    s.push("".to_string());
    s.join("\n")
}
