use regex::{Regex, Error};
use std::{path::Path, sync::Arc};
use super::{ParserMethods};
use swc_ecma_parser::{parse_file_as_module, EsConfig, Syntax};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};

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
                if res {
                    let cm = Arc::<SourceMap>::default();
                    let fm = cm
                        .load_file(Path::new("foo.js"))
                        .expect("failed to load file");
                    let module = parse_file_as_module(
                        &fm, 
                        Default::default(),
                        Default::default(),
                        None,
                        &mut vec![]).unwrap();
                    // module.body
                }
            },
            Err(err) => {
             println!("{:?}",err)
            }
        }
        res
     }
}