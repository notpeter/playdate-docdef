use indexmap::IndexMap;

// Stub Struct containing extracted signature, url anchor, list of parameters and description text
#[derive(Debug)]
pub struct Stub {
    pub title: String,
    pub anchor: String,
    pub params: Vec<(String, String)>, // parameter_name=type_name
    pub returns: IndexMap<String, String>, // return_name=type_name
    pub text: Vec<String>,
}

pub fn func_signature(name: &String, params: &Vec<(String, String)>) -> String {
    let param_names : Vec<String> = params.iter().map(|(name, _)| name.clone().replace("?", "")).collect::<Vec<String>>();
    String::from(format!("{}({})", name, param_names.join(", ")))
}

impl Stub {
    pub fn to_luars(&self) -> String {
        let mut s = String::new();
        // s.push_str(&format!("# https://sdk.play.date/Inside%20Playdate.html#{}\n", self.anchor));
        s.push_str(&format!("fn {}(", &self.title));
        s.push_str(&format!(
            "{}",
            self.params
                .iter()
                .map(|(p_name, p_type)| format!("{}: {}", p_name, p_type))
                .collect::<Vec<String>>()
                .join(", ")
        ));
        s.push_str(") -> ");
        if self.returns.len() == 1 {
            s.push_str(&self.returns.values().next().unwrap().to_string());
        } else {
            s.push_str(&format!(
                "({})",
                self.returns
                    .iter()
                    .map(|(r_name, r_type)| format!("{}: {}", r_name, r_type))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
        s
    }
    pub fn to_toml(&self) -> String {
        let mut s = String::new();
        s.push_str(
            &format!(
                "[[function]]\n\
                name = \"{}\"\n\
                anchor = \"{}\"\n",
                self.title,
                self.anchor
            )
        );
        if self.params.is_empty() {
            s.push_str(
                "args = []\n"
            );
        } else if self.params.len() == 1 {
            for (p_name, p_type) in &self.params {
                s.push_str(&format!("args = [ {{ name = \"{}\", type = \"{}\" }}, ]\n", p_name, p_type));
            }
        } else {
            s.push_str("args = [\n");
            for (p_name, p_type) in &self.params {
                s.push_str(&format!("    {{ name = \"{}\", type = \"{}\" }},\n", p_name, p_type));
            }
            s.push_str("]\n");
        }

        if self.returns.len() == 0 {
            s.push_str("returns = []\n");
        } else if self.returns.len() == 1 {
            for (r_name, r_type) in &self.returns {
                s.push_str(&format!("returns = [ {{ name = \"{}\", type = \"{}\" }}, ]\n", r_name, r_type));
            }
        } else {
            s.push_str("returns = [\n");
            for (r_name, r_type) in &self.returns {
                s.push_str(&format!("    {{ name = \"{}\", type = \"{}\" }},\n", r_name, r_type));
            }
            s.push_str("]\n");
        }
        s.push_str("\n");
        s
    }
    pub fn to_stub(&self) -> String {
        // TODO decide how to handle variables
        String::from(format!("function {} end", func_signature(&self.title, &self.params)))
    }
    pub fn to_lua(&self) -> String {
        String::from(format!(
            "{}--- https://sdk.play.date/Inside%20Playdate.html#{}\n{}{}{}\n",
            self.text2comments(),
            self.anchor,
            self.params2comments(),
            self.returns2comments(),
            self.to_stub()
        ))
    }
    fn text2comments(&self) -> String {
        let mut s = String::new();
        let mut i = 0;
        let mut in_code = false;
        while i < self.text.len() {
            let line = self.text[i].clone();
            // Bulleted list and code get fewer newlines. Everything else needs empty lines for proper markdown rendering.
            if in_code || line.starts_with("```") || (line.starts_with("*") && i < self.text.len() - 1 && self.text[i + 1].starts_with("*")) {
                s.push_str(&format!("--- {}\n", line));
            } else {
                s.push_str(&format!("--- {}\n---\n", line));
            }
            // this is hacky as hell
            if line == "```" {
                in_code = !in_code;
            }
            i = i + 1;
        }
        s
    }
    fn params2comments(&self) -> String {
        let mut s = String::new();
        for (p_name, p_type) in &self.params {
            if p_name != "..." && p_name != "...?" {
                s.push_str(&format!("---@param {} {}\n", p_name, p_type));
            }
        }
        s
    }
    fn returns2comments(&self) -> String {
        let mut s = String::new();
        for (r_name, r_type) in &self.returns {
            if r_name == "" {
                s.push_str(&format!("---@return {}\n", r_type));
            } else {
                s.push_str(&format!("---@return {} {}\n", r_type, r_name));
            }
        }
        s
    }
}
