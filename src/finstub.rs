use crate::luars::LuarsStatement;
use crate::stub::Stub;

pub struct Attribute {
    pub key_name: String,
    pub key_type: String,
    pub key_value: String,
}

pub struct Variable {
    pub prefix: String,
    pub var_name: String,
    pub var_type: String,
    pub var_attr: Vec<Attribute>,
}

pub enum FinStub {
    Stub(Stub),
    Variable(Variable),
}

impl FinStub {
    pub fn from_stub(stub: &Stub) -> FinStub {
        FinStub::Stub(stub.clone())
    }
    pub fn from_luars(luars: &LuarsStatement) -> FinStub {
        let prefix: String = match luars {
            LuarsStatement::Local(_, _, _) => "local ".to_string(),
            _ => "".to_string(),
        };
        match luars {
            LuarsStatement::Global(var_name, var_type, attr_in) |
            LuarsStatement::Local(var_name, var_type, attr_in) => {
                let var_name = var_name.to_string();
                let var_type = var_type.to_string();
                let mut var_attr: Vec<Attribute> = Vec::new();
                for attr in attr_in {
                    let (key_name, key_type, key_value) = attr;
                    let key_name = key_name.to_string();
                    let key_type = key_type.to_string();
                    let key_value = key_value.to_string();
                    var_attr.push(Attribute { key_name, key_type, key_value } );
                }
                FinStub::Variable(Variable {prefix, var_name, var_type, var_attr } )
            },
            LuarsStatement::Function(fun_name, fun_params, fun_returns) => {
                let title = fun_name.to_string();
                let anchor = "".to_string();
                let params: Vec<(String, String)> = fun_params.iter().map(|(fname, ftype)| (fname.to_string(), ftype.to_string())).collect();
                let returns: Vec<(String, String)> = fun_returns.iter().map(|(fname, ftype)| (fname.to_string(), ftype.to_string())).collect();
                let text: Vec<String> = Vec::new();
                FinStub::Stub(Stub { title, anchor, params, returns, text })
            },
        }
    }
    pub fn lua_statement(&self) -> String {
        // Return a valid lua statement for the class or function.
        match self {
            FinStub::Variable(var) => { format!("{}{} = {{}}", var.prefix, var.var_name) },
            FinStub::Stub(stub) => { stub.to_stub() },
        }
    }
    fn luacats_params(&self) -> Vec<String> {
        // Returns '---@param name type' for functions
        match self {
            FinStub::Stub(stub) => {
                stub.params.iter().map(
                    |(name, _type)|
                        format!("---@param {} {}", name, _type)
                ).collect::<Vec<String>>()
            }
            FinStub::Variable(_) => {
                unreachable!("LuarsStatement::Global and LuarsStatement::Local should not be called with luacats_params()")
            }
        }
    }
    fn luacats_returns(&self) -> Vec<String> {
        // Returns '---@return type [name]' for functions
        match self {
            FinStub::Stub(stub) => {
                stub.returns.iter().map(
                    |(_name, _type)|
                        if _name.to_string() == "" {
                            format!("---@return {}", _type)
                        } else {
                            format!("---@return {_type} {_name}", _type=_type, _name=_name)
                        }
                ).collect::<Vec<String>>()
            }
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
            FinStub::Variable(var) => {
                var.var_attr.iter().map(
                    |a|
                        if a.key_value.to_string() != "" {
                            format!("---@field {} {} {}", a.key_name, a.key_type, a.key_value)
                        } else {
                            format!("---@field {} {}", a.key_name, a.key_type)
                        }
                ).collect::<Vec<String>>()
            }
            FinStub::Stub(_) => {
                unreachable!("luacats_fields() should not be called with FinStub::Stub")
            }
        }
    }
    pub fn generate_stub(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        match self {
            FinStub::Variable(_) => {
                out.push(self.luacats_class());
                out.extend(self.luacats_fields());
                out.push(self.lua_statement());
            },
            FinStub::Stub(stub) => {
                out.extend(stub.text_comments());
                out.extend(self.luacats_params());
                out.extend(self.luacats_returns());
                out.push(self.lua_statement());
            }
        }
        out
    }
}
