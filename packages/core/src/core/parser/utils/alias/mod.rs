use std::collections::HashMap;

use regex::Regex;

pub struct AliasChecker {
    pub alias:HashMap<String,String>
}

impl AliasChecker {
    pub fn new(alias_config:&HashMap<String,String>) -> AliasChecker {
        Self { alias: alias_config.clone() }
    }

    pub fn replace(&self,dep_path: &String) -> String {
        let mut result = dep_path.clone();
        for (k,v) in self.alias.iter() {
            let replace_key = k.clone() + "/";
            let replace_value = v.clone() + "/";
            result = result.replace(&replace_key,&replace_value);
        }
        result
    }

    pub fn check(&self, dep_path:&String) ->bool {
        for(k,_) in self.alias.iter() {
            let alia_regex = Regex::new(k).expect("Fail to create regex for alias");
            if alia_regex.is_match(dep_path) {
                return  true;
            } else {
                continue;
            }
        }
        return false
    }
}