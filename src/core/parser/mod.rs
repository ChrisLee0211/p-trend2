use regex::{Regex, Captures};
pub enum CodeType {
    Vue,
    JS,
    TS,
    DTS,
    CSS
}

impl CodeType {
    pub fn new(&self, name:&String) -> CodeType {
        let js_reg = Regex::new(r"\.(js|jsx)$");
    }
}

pub fn parse_deps_by_file_name(name:&String) -> Vec<String> {

}