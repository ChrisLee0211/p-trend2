use std::{collections::HashMap};
use swc_ecma_ast::{ModuleItem, Lit};

pub fn replace_alias_for_import_path (import_path:String, alias:&HashMap<String, String>) -> String {
    let mut result = import_path;
    for (k,v) in alias.iter() {
        result = result.replace(k, v);
    }
    result
}

pub fn get_import_paths_by_ast(code_ast_body:&mut Vec<ModuleItem>)-> Vec<String> {
    let mut import_list:Vec<String> = vec![];
    for module_item in  code_ast_body {
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
    return import_list;
}