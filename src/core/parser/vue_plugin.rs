use std::{collections::HashMap, rc::Rc, cell::RefCell};

use regex::{Regex, Error};

use super::{ParserMethods};
#[derive(Debug)]
pub struct VueParser {
    pub rule: &'static str
}

impl ParserMethods for VueParser {

    fn match_code_type(&self, name:&String) -> Result<bool, Error> {
        let vue_reg = Regex::new(self.rule)?;
        if vue_reg.is_match(name) {
            return Ok(true)
         } else {
            return Ok(false)
         }
    }

    fn parse_import(&self, file_name:&String, alias_map: Rc<RefCell<HashMap<String, String>>>) -> Vec<String> {
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
