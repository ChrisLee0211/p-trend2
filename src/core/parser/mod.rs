use std::vec;

use regex::{Regex, Error};
mod js_plugin;
mod ts_plugin;
mod vue_plugin;
mod less_plugin;
pub enum CodeType {
    VUE,
    JS,
    TS,
    DTS,
    LESS
}

pub trait ParserPlugin {
    fn new() -> Self;

    fn match_code_type(&self, name:&String) -> Result<CodeType, Error>;

    fn import_parser(&self,file_name:&String) -> Vec<String>;
}

pub fn get_code_type(name:&String) -> Result<CodeType, String> {
    let js_reg = Regex::new(r"\.(js|jsx)$").expect("fail to init regex for .js");
    if js_reg.is_match(name) {
       return Ok(CodeType::JS)
    }
    let dts_reg = Regex::new(r"\.\d\.ts").expect("fail to init regex for .d.ts");
    if dts_reg.is_match(name) {
       return Ok(CodeType::DTS)
    }
    let ts_reg = Regex::new(r"\.(ts|tsx)$").expect("fail to init regex for .ts");
    if ts_reg.is_match(name) {
       return Ok(CodeType::TS)
    }
    let vue_reg = Regex::new(r"\.(vue)$").expect("fail to init regex for .vue");
    if vue_reg.is_match(name) {
       return Ok(CodeType::VUE)
    }
    let less_reg = Regex::new(r"\.(less)$").expect("fail to init regex for .less");
    if less_reg.is_match(name) {
       return Ok(CodeType::LESS)
    }
   return Err(String::from("Fail to init regex"))
}

pub fn parse_deps_by_file_name(name:&String) -> Vec<String> {
    let js_parser = js_plugin::JsParser::new();
    let ts_parser = ts_plugin::TsParser::new();
    let vue_parser = vue_plugin::VueParser::new();
    let less_parser = less_plugin::LessParser::new();

    let r:Vec<String> = vec![];
    r
    
}