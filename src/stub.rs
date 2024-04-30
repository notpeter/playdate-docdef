use crate::luars::LuarsStatement;
use textwrap;

// Wrap long lines of documentation at this length
// Note: Function signatures are not wrapped (a couple are >100 chars)
// Note: This also includes the leading "--- " (4 chars).
static MAX_LINE_LENGTH: usize = 100 - 4;

// Stub Struct containing extracted signature, url anchor, list of parameters and description text
#[derive(Debug, Clone)]
pub struct Stub {
    pub title: String,
    pub anchor: String,
    pub params: Vec<(String, String)>,  // parameter_name=type_name
    pub returns: Vec<(String, String)>, // return_name=type_name
    pub text: Vec<String>,
}

impl Stub {
    pub fn apply_types(mut self, statements: &Vec<LuarsStatement>) -> Stub {
        let func_sig = self.func_signature();
        let mut found: bool = false;
        for s in statements {
            //TODO: This is hella inefficient
            match s {
                LuarsStatement::Function(_, _, _) => {}
                _ => {
                    continue;
                }
            }
            let s_sig = s.func_sig();
            // eprintln!("INFO: Comparing {} to {}", func_sig, s_sig);

            match s {
                LuarsStatement::Function(_, params, returns) => {
                    if func_sig == s_sig {
                        self.params = params
                            .iter()
                            .map(|(fname, ftype)| (fname.to_string(), ftype.to_string()))
                            .collect();
                        self.returns = returns
                            .iter()
                            .map(|(fname, ftype)| (fname.to_string(), ftype.to_string()))
                            .collect();
                        found = true;
                    }
                }
                _ => {}
            }
        }
        if found {
            // eprintln!("INFO: Found types for {}", func_sig);
        } else {
            eprintln!("WARN: Could not find types for {}", func_sig);
        }
        self
    }
    pub fn func_signature(&self) -> String {
        let name = self.title.clone();
        let params = self.params.clone();
        let param_names: Vec<String> = params
            .iter()
            .map(|(name, _)| name.clone().replace("?", ""))
            .collect::<Vec<String>>();
        String::from(format!("{}({})", name, param_names.join(", ")))
    }
    pub fn text_comments(&self) -> Vec<String> {
        if self.anchor == "" {
            Vec::new()
        } else {
            self.text2comments()
        }
    }
    pub fn to_stub(&self) -> String {
        String::from(format!("function {} end", self.func_signature()))
    }
    fn text2comments(&self) -> Vec<String> {
        let mut s = Vec::new();
        let mut i = 0;
        let mut in_code = false;
        while i < self.text.len() {
            let line = self.text[i].clone();
            // Bulleted list and code get fewer newlines.
            // Everything else needs extra empty lines for proper markdown rendering.
            let no_break = in_code
                || line.starts_with("```")
                || (line.starts_with("* ")
                    && i < self.text.len() - 1
                    && self.text[i + 1].starts_with("* "));
            if no_break {
                s.push(format!("--- {}", line));
            } else {
                for wrapped_line in textwrap::wrap(line.as_str(), MAX_LINE_LENGTH) {
                    s.push(format!("--- {}", wrapped_line));
                }
                s.push("---".to_string());
            }
            // this is hacky as hell
            if line == "```" {
                in_code = !in_code;
            }
            i = i + 1;
        }
        s.push(format!(
            "--- [Inside Playdate: {}](https://sdk.play.date/Inside%20Playdate.html#{})",
            self.title.clone(),
            self.anchor
        ));
        s
    }
}
