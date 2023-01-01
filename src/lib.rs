#![deny(clippy::all)]

use std::collections::HashMap;
use lazy_static::lazy_static;
mod utils;
mod core;

use crate::core::{scan_by_entry};
#[macro_use]
extern crate napi_derive;



lazy_static! {
   static ref ALIAS: HashMap<String,String> = HashMap::new();
   static ref BASIC_CONFIG: HashMap<String,String> = HashMap::new();
}


#[napi(object)]
pub struct ConfigObject {
  /** 应当是一个完整的路径 */
  pub path: String,
  pub alias: Option<HashMap<String,String>>,
  pub excludes: Option<Vec<String>>
}


#[napi]
pub fn init(config: ConfigObject) {
  let entry = config.path;
  let alias = match config.alias {
    Some(alias_config) => alias_config,
    None => {
      let default_config:HashMap<String, String> = HashMap::new();
      default_config
    }
  };
  let excludes = match config.excludes {
    Some(excludes_collect) => excludes_collect,
    None => {
      let default_excludes:Vec<String> = vec![];
      default_excludes
    }
  };
  scan_by_entry(entry, alias, excludes);
}