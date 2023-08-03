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
            "\n{}--- https://sdk.play.date/2.0.1/Inside%20Playdate.html#{}\n{}\n",
            self.text2comments(), self.anchor, self.to_stub()
        ))
    }
    fn text2comments (&self) -> String {
        let mut s = String::new();
        for t in &self.text {
            s.push_str(&format!("--- {}\n---\n", t));
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
