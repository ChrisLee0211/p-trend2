#![deny(clippy::all)]

use std::collections::HashMap;
mod utils;
#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi(object)]
pub struct ConfigObject {
  pub path: Option<String>,
  pub alias: Option<HashMap<String,String>>
}

#[napi]
pub fn init(config: ConfigObject) -> Option<String> {
  let entry:Option<String> = config.path;
  entry
}