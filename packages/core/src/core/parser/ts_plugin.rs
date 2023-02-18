use regex::{Regex, Error};

use super::{ ParserMethods};
#[derive(Debug)]
pub struct TsParser {
    pub rule: &'static str
}

impl ParserMethods for TsParser {
    fn match_code_type(&self, name:&String) -> Result<bool, Error> {
        let ts_reg = Regex::new(self.rule)?;
        let dts_reg = Regex::new(r"\.\d\.ts")?;
        if ts_reg.is_match(name) && !dts_reg.is_match(name) {
            return Ok(true)
         } else {
            return Ok(false)
         }
    }

    fn parse_import(&self, file_name:&String) -> Vec<String> {
        let res:Vec<String> = vec![];
        let code_type = self.match_code_type(file_name);
        match code_type {
            Ok(res) => {
 
            },
            Err(err) => {
             println!("{:?}",err)
            }
        }
        res
     }
}


pub fn init_parser() -> TsParser {
    return TsParser { rule:r"\.(ts|tsx)$" }
}