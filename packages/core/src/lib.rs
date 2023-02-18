#![deny(clippy::all)]

use std::collections::HashMap;
mod utils;
mod core;

use crate::core::{scan_by_entry};
#[macro_use]
extern crate napi_derive;



#[napi(object)]
pub struct ConfigObject {
  /** 应当是一个完整的路径 */
  pub path: String,
  pub alias: HashMap<String,String>,
  pub npm_packages: Vec<String>,
  pub excludes: Vec<String>,
}


#[napi]
pub fn init(config: ConfigObject) {
  // 应该在js层就做好默认值处理，rust只做核心逻辑部分
  let ConfigObject {path:entry,excludes,alias,npm_packages} = config;
  scan_by_entry(entry, alias, npm_packages, excludes);
}
