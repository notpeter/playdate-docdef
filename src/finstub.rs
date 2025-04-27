use crate::luars::LuarsStatement;
use crate::stub::{Stub, StubFn, Table, TableContents};

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
    pub fn generate_stub(&self) -> Vec<String> {
        // Returns the complete stub for a class or function.
        let mut out: Vec<String> = Vec::new();
        match self {
            FinStub::VariableStub(stub) => {
                out.extend(stub.get_luacats());
            }
            FinStub::FunctionStub(stub) => {
                out.extend(stub.get_luacats());
            }
        }
        out
    }
}
