use regex::{Regex, Error};
use std::{path::Path, sync::Arc};
use super::{ParserMethods};
use swc_ecma_parser::{parse_file_as_module};
use swc_ecma_ast::{Lit};
use swc_common::{
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
                    println!("{:?}",fm);
                    let parse_error_message = String::from("Fail to parse code") + path;
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
                                .src
                                .value.to_string();
                                import_list.push(import_path);
                                continue;
                            }
                        }
                        if module_item.is_stmt() {
                            println!("is stmt ===> {:?}",path);
                            let statement_token = module_item.as_mut_stmt().expect("fail to get statement ast node");
                            println!("{:?}",statement_token);
                            if statement_token.is_expr() {
                                let expression = statement_token.as_expr()
                                .expect("fail to get expression statement");
                                let expression_token = &expression.expr;
                                if expression_token.is_call(){
                                    let call_expression = expression_token.as_call().expect("fail to get call expression ast node");
                                    let callee = &call_expression.callee;
                                    if callee.is_import() {
                                        let callee_args = &call_expression.args;
                                        let first_arg = callee_args.get(0).expect("fail to get callee first arg");
                                        if first_arg.expr.is_lit() {
                                            let lit = first_arg.expr.as_lit().expect("fail to get callee first arg by lit");
                                            match lit {
                                                Lit::Str(token) => {
                                                    let import_path = token.value.to_string();
                                                    import_list.push(import_path);
                                                }
                                                _ => ()
                                            }
                                        }
                                    }
                                    
                                }
                                
                            }
                        }
                    }
                }
            },
            Err(err) => {
             println!("{:?}",err)
            }
        }
        import_list
     }
}