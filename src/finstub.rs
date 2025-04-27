use crate::luars::LuarsStatement;
use crate::stub::{Stub, StubFn};
use std::collections::HashMap;

pub struct TableContents {
    pub name: String,
    pub r#type: String,
    pub value: String, // Some Enums have documented values
}

impl TableContents {
    pub fn to_string(&self) -> String {
        let mut line = Vec::new();
        line.push("---@field");
        line.push(&self.name);
        line.push(&self.r#type);
        if !self.value.is_empty() {
            line.push(&self.value);
        }
        line.join(" ")
    }
}

/// Global and Local Variables
pub struct Table {
    pub prefix: String,
    pub name: String,
    pub r#type: String,
    pub contents: Vec<TableContents>,
}

/// Final Stubs, ready for outputting
pub enum FinStub {
    FunctionStub(StubFn),
    VariableStub(Table),
}

impl FinStub {
    pub fn from_stub(stub: &Stub) -> FinStub {
        match stub {
            Stub::Function(fn_stub) => FinStub::FunctionStub(fn_stub.clone()),
        }
    }
    pub fn from_luars(luars: &LuarsStatement) -> FinStub {
        let prefix: String = match luars {
            LuarsStatement::Local(_, _, _) => "local ".to_string(),
            _ => "".to_string(),
        };
        match luars {
            LuarsStatement::Global(table_name, table_type, table_contents)
            | LuarsStatement::Local(table_name, table_type, table_contents) => {
                let var_name = table_name.to_string();
                let var_type = table_type.to_string();
                let mut var_contents: Vec<TableContents> = Vec::new();
                for tc in table_contents {
                    let (key_name, key_type, key_value) = tc;
                    var_contents.push(TableContents {
                        name: key_name.to_string(),
                        r#type: key_type.to_string(),
                        value: key_value.to_string(),
                    });
                }
                FinStub::VariableStub(Table {
                    prefix,
                    name: var_name,
                    r#type: var_type,
                    contents: var_contents,
                })
            }
            LuarsStatement::Function(fun_name, fun_params, fun_returns) => {
                let title = fun_name.to_string();
                let anchor = "".to_string();
                let params: Vec<(String, String)> = fun_params
                    .iter()
                    .map(|(fname, ftype)| (fname.to_string(), ftype.to_string()))
                    .collect();
                let returns: Vec<(String, String)> = fun_returns
                    .iter()
                    .map(|(fname, ftype)| (fname.to_string(), ftype.to_string()))
                    .collect();
                let text: Vec<String> = Vec::new();
                FinStub::FunctionStub(StubFn {
                    title,
                    anchor,
                    params,
                    returns,
                    text,
                })
            }
        }
    }
    pub fn lua_statement(&self) -> String {
        // Return a valid lua statement for the class or function.
        match self {
            FinStub::VariableStub(var) => {
                format!("{}{} = {{}}", var.prefix, var.name)
            }
            FinStub::FunctionStub(stub) => stub.lua_statement(),
        }
    }
    fn luacats_class(&self) -> String {
        // Returns '---@class name : parent' for classes
        match self {
            FinStub::VariableStub(var) => {
                if var.r#type.to_string() == "" {
                    format!("---@class {}", var.name)
                } else {
                    format!("---@class {} : {}", var.name, var.r#type)
                }
            }
            _ => {
                unreachable!("luacats_class() should not be called with FinStub::Stub")
            }
        }
    }
    fn luacats_fields(&self) -> Vec<String> {
        // Returns '---@field name type [value]' for class/instance attributes
        match self {
            FinStub::VariableStub(var) => var
                .contents
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>(),
            _ => {
                unreachable!("luacats_fields() should not be called with FinStub::Stub")
            }
        }
    }
    pub fn generate_stub(&self) -> Vec<String> {
        // Returns the complete stub for a class or function.
        let notes = HashMap::from([
            // TODO: maybe make this lazy_static (how?)
            (
                "playdate.ui.crankIndicator:start()",
                "---@deprecated since 2.1.0-beta1",
            ),
            (
                "playdate.ui.crankIndicator:update()",
                "---@deprecated since 2.1.0-beta1",
            ),
        ]);
        let mut out: Vec<String> = Vec::new();
        match self {
            FinStub::VariableStub(_) => {
                out.push(self.luacats_class());
                out.extend(self.luacats_fields());
                out.push(self.lua_statement());
            }
            FinStub::FunctionStub(stub) => {
                if notes.contains_key(stub.lua_def().as_str()) {
                    out.push(notes.get(stub.lua_def().as_str()).unwrap().to_string());
                }
                out.extend(stub.generate_description());
                out.extend(stub.luacats_params());
                out.extend(stub.luacats_returns());
                out.push(stub.lua_statement());
            }
        }
        out
    }
}
