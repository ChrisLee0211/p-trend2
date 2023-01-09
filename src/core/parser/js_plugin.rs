use regex::{Regex, Error};

use super::{ParserMethods};
#[derive(Debug)]
pub struct JsParser {
   pub rule: &'static str
}

impl ParserMethods for JsParser {
    fn match_code_type(&self, name:&String) -> Result<bool, Error> {
        let js_reg = Regex::new(self.rule)?;
        if js_reg.is_match(name) {
            return Ok(true)
         } else {
            return Err(Error::Syntax(String::from("fail to match js reg")))
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