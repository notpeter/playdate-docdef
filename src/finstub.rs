use crate::luars::LuarsStatement;
use crate::stub::StubFn;
use std::collections::HashMap;

pub struct Attribute {
    pub key_name: String,
    pub key_type: String,
    pub key_value: String,
    pub comment: String,
}

impl Attribute {
    pub fn to_string(&self) -> String {
        let mut line = Vec::new();
        line.push("---@field");
        line.push(&self.key_name);
        line.push(&self.key_type);
        if !self.key_value.is_empty() {
            line.push(&self.key_value);
        }
        if !self.comment.is_empty() {
            line.push(&self.comment);
        }
        line.join(" ")
    }
}

pub struct Variable {
    pub prefix: String,
    pub var_name: String,
    pub var_type: String,
    pub var_attr: Vec<Attribute>,
}

pub enum FinStub {
    Stub(StubFn),
    Variable(Variable),
}

impl FinStub {
    pub fn from_stub(stub: &StubFn) -> FinStub {
        FinStub::Stub(stub.clone())
    }
    pub fn from_luars(luars: &LuarsStatement) -> FinStub {
        let prefix: String = match luars {
            LuarsStatement::Local(_, _, _) => "local ".to_string(),
            _ => "".to_string(),
        };
        match luars {
            LuarsStatement::Global(var_name, var_type, attr_in)
            | LuarsStatement::Local(var_name, var_type, attr_in) => {
                let var_name = var_name.to_string();
                let var_type = var_type.to_string();
                let mut var_attr: Vec<Attribute> = Vec::new();
                for attr in attr_in {
                    let (key_name, key_type, key_value) = attr;
                    let key_name = key_name.to_string();
                    let key_type = key_type.to_string();
                    let key_value = key_value.to_string();
                    var_attr.push(Attribute {
                        key_name,
                        key_type,
                        key_value,
                        comment: String::from("foo"),
                    });
                }
                FinStub::Variable(Variable {
                    prefix,
                    var_name,
                    var_type,
                    var_attr,
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
                FinStub::Stub(StubFn {
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
            FinStub::Variable(var) => {
                format!("{}{} = {{}}", var.prefix, var.var_name)
            }
            FinStub::Stub(stub) => stub.to_stub(),
        }
    }
    fn luacats_params(&self) -> Vec<String> {
        // Returns '---@param name type' for functions
        match self {
            FinStub::Stub(stub) => stub
                .params
                .iter()
                .map(|(name, _type)| format!("---@param {} {}", name, _type))
                .collect::<Vec<String>>(),
            FinStub::Variable(_) => {
                unreachable!("LuarsStatement::Global and LuarsStatement::Local don't support luacats_params()")
            }
        }
    }
    fn luacats_returns(&self) -> Vec<String> {
        // Returns '---@return type [name]' for functions
        match self {
            FinStub::Stub(stub) => stub
                .returns
                .iter()
                .map(|(_name, _type)| {
                    if _name.to_string() == "" {
                        format!("---@return {}", _type)
                    } else {
                        format!("---@return {_type} {_name}", _type = _type, _name = _name)
                    }
                })
                .collect::<Vec<String>>(),
            FinStub::Variable(_) => {
                unreachable!("luacats_returns() should not be called with FinStub::Variable")
            }
        }
    }
    fn luacats_class(&self) -> String {
        // Returns '---@class name : parent' for classes
        match self {
            FinStub::Variable(var) => {
                if var.var_type.to_string() == "" {
                    format!("---@class {}", var.var_name)
                } else {
                    format!("---@class {} : {}", var.var_name, var.var_type)
                }
            }
            FinStub::Stub(_) => {
                unreachable!("luacats_class() should not be called with FinStub::Stub")
            }
        }
    }
    fn luacats_fields(&self) -> Vec<String> {
        // Returns '---@field name type [value]' for class/instance attributes
        match self {
            FinStub::Variable(var) => var
                .var_attr
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>(),
            FinStub::Stub(_) => {
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
            FinStub::Variable(_) => {
                out.push(self.luacats_class());
                out.extend(self.luacats_fields());
                out.push(self.lua_statement());
            }
            FinStub::Stub(stub) => {
                if notes.contains_key(stub.func_signature().as_str()) {
                    out.push(
                        notes
                            .get(stub.func_signature().as_str())
                            .unwrap()
                            .to_string(),
                    );
                }
                out.extend(stub.text_comments());
                out.extend(self.luacats_params());
                out.extend(self.luacats_returns());
                out.push(self.lua_statement());
            }
        }
        out
    }
}
