#![deny(clippy::all)]
mod utils;
#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

use napi::JsObject;

#[napi]
pub fn init(config: JsObject) -> Option<String> {
  let entry:Option<String> = config.get("path").expect("msg");
  entry
}