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
    let param_names : Vec<String> = params.iter().map(|(name, _)| name.clone()).collect::<Vec<String>>();
    String::from(format!("{}({})", name, param_names.join(", ")))
}

impl Stub {
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
            if p_name != "..." {
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
