use crate::utils::{
  get_enbale_paths, normalize_file_node_path, resolve_related_path_to_absoluted_path, FileNodePaths,
};

use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs, io::Error};
mod io;
mod parser;
use io::file_node::FileNode;

#[derive(Debug)]
pub struct ModuleNode {
  // 节点指向
  node: Rc<RefCell<FileNode>>,
  // 被引用的节点path
  reference_path: Rc<RefCell<Vec<String>>>,
}

impl ModuleNode {
  pub fn new(node: Rc<RefCell<FileNode>>, reference_path: Vec<String>) -> ModuleNode {
    ModuleNode {
      node,
      reference_path: Rc::new(RefCell::new(reference_path)),
    }
  }

  pub fn add_reference(&mut self, path: String) {
    self.reference_path.borrow_mut().push(path);
  }
}

pub struct NpmPackages {
  pub pkg_map: HashMap<String, i32>,
}

impl NpmPackages {
  pub fn new(list: Vec<String>) -> NpmPackages {
    let mut pkg_map: HashMap<String, i32> = HashMap::new();
    for npm in list.iter() {
      pkg_map.insert(npm.to_string(), 0);
    }
    return NpmPackages { pkg_map };
  }

  pub fn check_is_npm_pkg(&self, target: &String) -> Option<String> {
    let all_pkg_names = self.pkg_map.keys();
    for pkg in all_pkg_names {
      if target.contains(pkg) {
        return Some(pkg.clone());
      }
    }
    return None;
  }

  pub fn add_npm_reference_count(&mut self, target: &String) -> Result<i32, String> {
    let original_target = self.pkg_map.get(target);
    match original_target {
      Some(num) => {
        let result = num + 1;
        self.pkg_map.insert(target.to_string(), result);
        return Ok(result);
      }
      None => return Err(String::from("fail to get npm package in pkg_map")),
    }
  }
}



pub fn scan_by_entry(
  entry: String,
  alias_config: HashMap<String, String>,
  npm_packages: Vec<String>,
  excludes: Vec<String>,
) -> Result<(), Error> {
  // 用于收集npm package依赖引用
  let npm_map = NpmPackages::new(npm_packages);
  // 存储所有解析出来的moduleNode的列表
  let module_node_list: Rc<RefCell<Vec<RefCell<ModuleNode>>>> =
    Rc::new(RefCell::new(vec![]));
  // file_hash_map可以通过路径获取索引，然后去module_node_list找到真正的唯一的fileNode
  // 设计原因：因为可能有多个引用路径指向同一个fileNode
  let mut file_hash_map: HashMap<String, Box<usize>> = HashMap::new();

  let current_dir_path = env::current_dir()
    .expect("Fail to get current work dir pathbuf")
    .to_str()
    .expect("Fail to transform current dir pathbuf to string")
    .to_string();
  let normalize_entry = resolve_related_path_to_absoluted_path(&entry, &current_dir_path);
  let mut root_file_node = FileNode::new(normalize_entry);
  let mut parser = parser::Parser::new(alias_config,&excludes, npm_map);

  let mut file_node_hashmap_marker = |node_target: &mut FileNode, node_paths: FileNodePaths| {
    let file_node_for_hash =
      ModuleNode::new(Rc::new(RefCell::new(node_target.clone())), Vec::new());
    module_node_list
      .borrow_mut()
      .push(RefCell::new(file_node_for_hash));
    let index = module_node_list.borrow_mut().len() - 1;
    let enable_paths: Vec<String> = get_enbale_paths(&node_paths);
    for enable_path in enable_paths {
      file_hash_map.insert(enable_path, Box::new(index));
    }
  };
  
  build_file_node(
    &mut root_file_node,
    &mut parser,
    &mut file_node_hashmap_marker,
  )?;

  let file_nodes_count = module_node_list.borrow().len();
  let mut loop_cursor: usize = 0;
  loop {
    if loop_cursor == file_nodes_count {
      break;
    }
    mark_reference(loop_cursor, &module_node_list, &file_hash_map);
    loop_cursor += 1;
  }
  let json_string =
    serde_json::to_string(&root_file_node.clone()).expect("fail to transform json string");
  let current_dir = env::current_dir().expect("fail to get current dir pathbuf");

  fs::write(current_dir.join("data.json"), json_string).expect("fail to create json");
  Ok(())
}

pub fn mark_reference(
  cursor: usize,
  module_node_list: &Rc<RefCell<Vec<RefCell<ModuleNode>>>>,
  file_hash_map: &HashMap<String, Box<usize>>,
) {
  let node = module_node_list
    .borrow_mut()
    .get(cursor)
    .expect("fail to get deps by module_node_list")
    .borrow_mut()
    .node
    .clone();

  let file_node_path = node.borrow_mut().file_path.clone();
  let deps = node.borrow_mut().deps.clone();

  let deps_len = deps.len();

  let mut dep_cursor: usize = 0;

  loop {
    if dep_cursor == deps_len {
      break;
    } else {
      let dep = deps
        .get(dep_cursor)
        .expect("fail to get deps by module_node_list")
        .clone();
      let target_index = file_hash_map.get(&dep);
      match target_index {
        Some(index) => {
          module_node_list
            .borrow_mut()
            .get(*index.clone())
            .expect("fail to get file_node_for_hash by index")
            .borrow_mut()
            .add_reference(file_node_path.clone());
        }
        None => {}
      }
    }
    dep_cursor += 1;
  }
}

/**
 * 构建文件节点树，并生成生成关系
 * 
 */
fn build_file_node<F>(
  current_node: &mut FileNode,
  parser: &mut parser::Parser,
  reference_marker: &mut F,
) -> Result<(), Error>
where
  F: FnMut(&mut FileNode, FileNodePaths),
{
  let current_node_path = current_node.file_path.clone();
  for file in fs::read_dir(current_node.get_path())? {
    let path_buffer = file?.path();
    let file_node_paths = normalize_file_node_path(&current_node_path, &path_buffer);
    let FileNodePaths {
      normal_path: _,
      file_name: _,
      absolute_path: _,
      absolute_path_with_file_name,
    } = &file_node_paths;
    // 创建文件节点
    let file_node_path: String = absolute_path_with_file_name.to_string();
    if parser.exclude_checker.check(&file_node_path) == true {
        continue;
    }
    let mut child = FileNode::new(file_node_path);
    if child.is_folder {
      build_file_node(&mut child, parser, reference_marker)?;
    } else {
      let deps: Vec<String> = parser.parse_deps_by_file_name(&mut child);
      child.set_deps(deps);
      reference_marker(&mut child, file_node_paths);
    }
    child.set_parent(current_node.file_path.clone());
    current_node.insert_child(child);
  }
  Ok(())
}
