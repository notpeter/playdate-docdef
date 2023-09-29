use serde_derive::Deserialize;
use toml;

pub struct LuaThing {
    pub name: String,
    pub r#type: String,
    pub note: String,
    pub docs: String,
}

#[derive(Deserialize, Default)]
struct LuaOptional {
    lt: LuaThing,
    #[serde(default = false)]
    optional: bool,
}

pub struct LuaFunction {
    pub lt: LuaThing,
    pub args: Vec<LuaOptional>,
    pub rets: Vec<LuaOptional>,
}

pub struct LuaTOML {
    pub table: Vec<LuaThing>,
    pub method: Vec<LuaFunction>,
    pub property: Vec<LuaThing>,
    pub function: Vec<LuaFunction>,
}
