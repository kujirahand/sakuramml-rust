//! song: ユーザー定義関数・システム関数の定義
use super::*;

#[derive(Debug)]
pub enum SFunctionType {
    System,
    User,
}

#[derive(Debug)]
pub struct SFunction {
    pub name: String,
    pub tokens: Tokens,
    pub lineno: isize,
    pub func_id: usize,
    pub arg_names: Vec<String>,
    pub arg_types: Vec<char>, // S: string, I: int, A: array
    pub arg_def_values: Vec<SValue>,
    pub function_type: SFunctionType,
}

impl SFunction {
    pub fn new(name: &str, tokens: Tokens, func_id: usize, lineno: isize) -> Self {
        Self {
            name: name.to_string(),
            tokens,
            lineno,
            func_id,
            arg_names: vec![],
            arg_types: vec![],
            arg_def_values: vec![],
            function_type: SFunctionType::User,
        }
    }
    pub fn new_system(name: &str, func_id: usize, arg_types: &'static str) -> Self {
        Self {
            name: name.to_string(),
            tokens: vec![],
            lineno: 0,
            func_id,
            arg_names: vec![],
            arg_types: arg_types.chars().collect(),
            arg_def_values: vec![],
            function_type: SFunctionType::System,
        }
    }
}
