use std::vec;

use regex::{Error};
mod js_plugin;
mod ts_plugin;
mod vue_plugin;
mod less_plugin;

pub trait ParserMethods {

    fn match_code_type(&self, name:&String) -> Result<bool, Error>;

    fn parse_import(&self,file_name:&String) -> Vec<String>;
}

pub struct Plugins {
    pub plugins:Vec<Box<dyn ParserMethods>>
}

impl Plugins {
    pub fn collect_import(&self, name:&String) -> Vec<String> {
        let mut result:Vec<String> = vec![];
        let empty:usize = 0;
        for plugin in self.plugins.iter() {
            if &result.len() != &empty { break }
            result = plugin.parse_import(name);
        }
        return result
    }
}

pub fn parse_deps_by_file_name(name:&String) -> Vec<String> {
    let js_parser = js_plugin::JsParser {rule:r"\.(js|jsx)$"};
    let ts_parser = ts_plugin::TsParser{rule:r"\.(ts|tsx)$"};
    let vue_parser = vue_plugin::VueParser{rule:r"\.(vue)$"};
    let less_parser = less_plugin::LessParser{rule:r"\.(less)$"};
    let parser_plugins = Plugins {
        plugins:vec![
            Box::new(js_parser),
            Box::new(ts_parser),
            Box::new(vue_parser),
            Box::new(less_parser)
        ]
    };
    let result:Vec<String> = parser_plugins.collect_import(name);
    result
    
}