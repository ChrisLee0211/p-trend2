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
                    let parse_error_message = String::from("Fail to parse code") + file_name;
                    let module = parse_file_as_module(
                        &fm, 
                        Default::default(),
                        Default::default(),
                        None,
                        &mut vec![]).expect(&parse_error_message);
                    let mut code_ast_body = module.body;
                    for module_item in &mut code_ast_body {
                        if module_item.is_module_decl() {
                            let decl_token = module_item.as_mut_module_decl().expect("fail to get module declare ast node");
                            if decl_token.is_import() {
                                let import_path = decl_token.as_mut_import()
                                .expect("fail to get import token")
                                .src.raw
                                .expect("fail to transform import token box to atom")
                                .to_string();
                            }
                        }
                        if module_item.is_stmt() {

                        }
                    }
                }
            },
            Err(err) => {
             println!("{:?}",err)
            }
        }
        res
     }
}