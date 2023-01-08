use regex::{Regex, Error};

use super::{CodeType, ParserPlugin};
#[derive(Debug)]
pub struct VueParser {
    rule: &'static str
}

impl ParserPlugin for VueParser {
    fn new() -> VueParser {
        VueParser { rule: r"\.(vue)$" }
    }

    fn match_code_type(&self, name:&String) -> Result<CodeType, Error> {
        let vue_reg = Regex::new(self.rule)?;
        if vue_reg.is_match(name) {
            return Ok(CodeType::VUE)
         } else {
            return Err(Error::Syntax(String::from("fail to match vue reg")))
         }
    }

    fn import_parser(&self, file_name:&String) -> Vec<String> {
       let res:Vec<String> = vec![];
       res
    }
}
