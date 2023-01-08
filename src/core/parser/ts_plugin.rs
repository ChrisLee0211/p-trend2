use regex::{Regex, Error};

use super::{CodeType, ParserPlugin};
#[derive(Debug)]
pub struct TsParser {
    rule: &'static str
}

impl ParserPlugin for TsParser {
    fn new() -> TsParser {
        TsParser { rule: r"\.(ts|tsx)$" }
    }

    fn match_code_type(&self, name:&String) -> Result<CodeType, Error> {
        let ts_reg = Regex::new(self.rule)?;
        let dts_reg = Regex::new(r"\.\d\.ts")?;
        if ts_reg.is_match(name) && dts_reg.is_match(name) {
            return Ok(CodeType::TS)
         } else {
            return Err(Error::Syntax(String::from("fail to match ts reg")))
         }
    }

    fn import_parser(&self, file_name:&String) -> Vec<String> {
       let res:Vec<String> = vec![];
       res
    }
}