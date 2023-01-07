use regex::Regex;

use super::CodeType;
#[derive(Debug)]
pub struct JsParser<'a> {
    rule: &'a str
}

impl <'a>JsParser<'a> {
    pub fn new() -> JsParser<'a> {
        JsParser { rule: r"\.(js|jsx)$" }
    }

    pub fn match_code_type(name:&String) -> Result<CodeType, String> {
        let js_reg = Regex::new(r"\.(js|jsx)$").expect("fail to init regex for .js");
        if js_reg.is_match(name) {
            return Ok(CodeType::JS)
         } else {
            return Err(String::from("Fail to init regex"))
         }
    }

    pub fn import_parser(file_name:&String) -> Vec<String> {
       let res:Vec<String> = vec![];
       res
    }
}