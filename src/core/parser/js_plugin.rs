use regex::{Regex, Error};

use super::{CodeType, ParserPlugin};
#[derive(Debug)]
pub struct JsParser {
    rule: &'static str
}

impl ParserPlugin for JsParser {
    fn new() -> JsParser {
        JsParser { rule: r"\.(js|jsx)$" }
    }

    fn match_code_type(&self, name:&String) -> Result<CodeType, Error> {
        let js_reg = Regex::new(self.rule)?;
        if js_reg.is_match(name) {
            return Ok(CodeType::JS)
         } else {
            return Err(Error::Syntax(String::from("fail to match js reg")))
         }
    }

    fn import_parser(&self,file_name:&String) -> Vec<String> {
       let res:Vec<String> = vec![];
       res
    }
}