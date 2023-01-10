use regex::{Regex, Error};

use super::{ParserMethods};
#[derive(Debug)]
pub struct LessParser {
    pub rule: &'static str
}

impl ParserMethods for LessParser {

     fn match_code_type(&self, name:&String) -> Result<bool, Error> {
        let less_reg = Regex::new(self.rule)?;
        if less_reg.is_match(name) {
            return Ok(true)
         } else {
            return Err(Error::Syntax(String::from("fail to match less reg")))
         }
    }

    fn parse_import(&self, file_name:&String) -> Vec<String> {
        let mut res:Vec<String> = vec![];
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