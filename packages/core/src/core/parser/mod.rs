use std::{collections::HashMap, vec};

use regex::Error;

use crate::utils::resolve_related_path_to_absoluted_path;

use self::utils::alias::AliasChecker;

use super::{io, NpmPackages};

mod utils;
use utils::exclude::ExcludeChecker;

pub mod common;
mod js_plugin;
mod less_plugin;
mod vue_plugin;

pub trait ParserMethods {
  fn match_code_type(&self, name: &String) -> Result<bool, Error>;

  fn parse_import(&self, file_name: &String) -> Vec<String>;
}

pub struct Plugins {
  pub plugins: Vec<Box<dyn ParserMethods>>,
}

impl Plugins {
  /// 收集依赖
  pub fn collect_import(&self, file_path: &String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for plugin in self.plugins.iter() {
      if &result.len() != &0 {
        break;
      }
      result = plugin.parse_import(file_path);
    }
    return result;
  }
}

/// 解析器
pub struct Parser<'a> {
  pub exclude_checker: ExcludeChecker<'a>,
  pub alias_checker: AliasChecker,
  pub npm_map: NpmPackages,
  parser_plugins: Plugins,
}

impl<'a> Parser<'a> {
  pub fn new(
    alias_config: HashMap<String, String>,
    excludes: &'a Vec<String>,
    npm_map: NpmPackages,
  ) -> Parser<'a> {
    let js_parser = js_plugin::init_parser();
    let vue_parser = vue_plugin::init_parser();
    let less_parser = less_plugin::init_parser();
    let parser_plugins = Plugins {
      plugins: vec![
        Box::new(js_parser),
        Box::new(vue_parser),
        Box::new(less_parser),
      ],
    };
    let exclude_checker: ExcludeChecker<'a> = ExcludeChecker::new(&excludes);
    let alias_checker = AliasChecker::new(&alias_config);
    Self {
      exclude_checker,
      alias_checker,
      npm_map,
      parser_plugins,
    }
  }

  pub fn parse_deps_by_file_name(
    &mut self,
    file_node: &mut io::file_node::FileNode,
  ) -> Vec<String> {
    let file_path = &file_node.file_path.clone();
    let deps: Vec<String> = self.parser_plugins.collect_import(file_path);
    let result: Vec<String> = deps
      .iter()
      .filter(|dep_path| {
        // 移除并标记npm包引用次数
        let npm = self.npm_map.check_is_npm_pkg(dep_path);
        match npm {
          Some(pkg_name) => {
            let err_msg = String::from("fail to add npm reference count by") + dep_path;
            self
              .npm_map
              .add_npm_reference_count(&pkg_name)
              .expect(&err_msg);
            file_node.insert_pkg(pkg_name.clone());
            return false;
          }
          None => true,
        }
      })
      .filter(|dep_path| {
        // 移除命中exclude的依赖
        return self.exclude_checker.check(&dep_path) == false;
      })
      .map(|dep_path| {
        //  替换alias路径别名
        if self.alias_checker.check(&dep_path) {
          return self.alias_checker.replace(&dep_path);
        }
        return resolve_related_path_to_absoluted_path(&dep_path, file_path);
      })
      .collect();
    result
  }
}
