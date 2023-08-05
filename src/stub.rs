use crate::args::Action;

// Stub Struct containing extracted signature, url anchor, list of parameters and description text
#[derive(Debug)]
pub struct Stub {
    pub title: String,
    pub anchor: String,
    pub params: Vec<String>,
    pub text: Vec<String>,
}

impl Stub {
    fn to_stub(&self) -> String {
        // TODO decide how to handle variables
        String::from(format!("function {}({}) end", self.title, self.params.join(", ")))
    }
    fn to_lua(&self) -> String {
        String::from(format!(
            "{}--- https://sdk.play.date/Inside%20Playdate.html#{}\n{}{}\n",
            self.text2comments(),
            self.anchor,
            self.params2comments(),
            self.to_stub()
        ))
    }
    fn text2comments (&self) -> String {
        let mut s = String::new();
        let mut i = 0;
        while i < self.text.len() {
            let this_line = self.text[i].clone();
            // Bulleted list get fewer newlines. Everything else needs empty lines for proper markdown rendering.
            if this_line.starts_with("*") && i < self.text.len() - 1 && self.text[i + 1].starts_with("*") {
                s.push_str(&format!("--- {}\n", this_line));
            } else {
                s.push_str(&format!("--- {}\n---\n", this_line));
            }
            i = i + 1;
        }
        s
    }
    fn params2comments (&self) -> String {
        let mut s = String::new();
        for p in &self.params {
            s.push_str(&format!("---@param {} any\n", p));
        }
        s
    }
}

pub fn generate(stubs: &Vec<Stub>, action: Action) {
    match action {
        Action::Stub => {
            println!("---@meta\n-- This file contains function stubs for autocompletion. DO NOT include it in your game.\n");
            for stub in stubs {
                println!("{}", stub.to_stub());
            }
        },
        Action::Annotate => {
            println!("---@meta\n-- This file contains function stubs for autocompletion. DO NOT include it in your game.\n");
            for stub in stubs {
                println!("{}", stub.to_lua());
            }
        },
    }
}
