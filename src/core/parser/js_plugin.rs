use regex::{Regex, Error};
use std::{path::Path, sync::Arc,};
use super::{ParserMethods};
use swc_ecma_parser::{parse_file_as_module};
use swc_common::{
    SourceMap,
};
use crate::core::parser::common::{get_import_paths_by_ast};
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
            return Ok(false)
         }
    }

    fn parse_import(&self, path:&String) -> Vec<String> {
        let mut import_list:Vec<String> = vec![];
        let code_type = self.match_code_type(path);
        match code_type {
            Ok(res) => {
                if res {
                    let cm = Arc::<SourceMap>::default();
                    let error_msg = String::from("failed to load file by") + path;
                    let fm = cm
                        .load_file(Path::new(path))
                        .expect(&error_msg);
                    let parse_error_message = String::from("Fail to parse code") + path;
                    let module = parse_file_as_module(
                        &fm, 
                        Default::default(),
                        Default::default(),
                        None,
                        &mut vec![]).expect(&parse_error_message);
                    let mut code_ast_body = module.body;
                    import_list = get_import_paths_by_ast(&mut code_ast_body);
                }
            },
            Err(err) => {
             println!("{:?}",err)
            }
        }
        import_list
     }
}

pub fn init_parser() -> JsParser {
    return JsParser { rule:r"\.(js|jsx)$" }
}