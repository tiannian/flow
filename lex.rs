use std::String;

pub enum Token {
    ModuleBegin,
    ModuleEnd,
    ImportPackage(String),
    PackageAlias(String),
    VariableName(String),
    VariableType(String),
    VariableWire(String,String),
}

pub struct Lex {

}

impl Lex {
    fn next_symbol() -> Token {
        
    }
}
