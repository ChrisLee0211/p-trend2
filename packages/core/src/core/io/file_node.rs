use std::{
  fs,
  path::{Path},
};

use serde::{Deserialize, Serialize};

use crate::utils::get_file_name_by_path;

/// 文件节点

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
  pub file_name: String,
  pub file_path: String,
  pub is_folder: bool,
   deps: Vec<String>,
  pkgs: Vec<String>,
  parent_path: String,
  children: Vec<FileNode>,
}
impl FileNode {
  pub fn new(path: String) -> FileNode {
    Self {
      file_name:get_file_name_by_path(&path),
      file_path:path.clone(),
      is_folder:fs::metadata(&path).unwrap().is_dir(),
      parent_path:String::from(""),
      deps: [].to_vec(),
      pkgs: [].to_vec(),
      children: [].to_vec(),
    }
  }

  pub fn insert_child(&mut self, child: FileNode) {
    self.children.push(child);
  }

  pub fn set_parent(&mut self, path_string: String) {
    self.parent_path.push_str(&path_string);
  }


  pub fn set_deps(&mut self, deps: Vec<String>) {
    for dependence_path in deps.iter() {
      self.deps.push(dependence_path.to_string())
    }
  }

  pub fn get_path(&mut self) -> &Path {
    return Path::new(&self.file_path);
  }

  pub fn insert_pkg(&mut self, npm:String) {
    self.pkgs.push(npm);
}
}
