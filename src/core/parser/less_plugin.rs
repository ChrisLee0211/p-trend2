use regex::{Regex, Error};

use super::{CodeType, ParserPlugin};
#[derive(Debug)]
pub struct LessParser {
    rule: &'static str
}

impl ParserPlugin for LessParser {
     fn new() -> LessParser {
        LessParser { rule: r"\.(less)$" }
    }

     fn match_code_type(&self, name:&String) -> Result<CodeType, Error> {
        let less_reg = Regex::new(self.rule)?;
        if less_reg.is_match(name) {
            return Ok(CodeType::LESS)
         } else {
            return Err(Error::Syntax(String::from("fail to match less reg")))
         }
    }

     fn import_parser(&self,file_name:&String) -> Vec<String> {
       let res:Vec<String> = vec![];
       res
    }
}