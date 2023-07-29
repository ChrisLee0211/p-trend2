use crate::utils::{
  get_enbale_paths, normalize_file_node_path,FileNodePaths,
};
use globmatch::Builder;
use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs, io::Error};
mod io;
mod parser;
use io::file_node::FileNode;


#[derive(Debug)]
pub struct FileNodeForHash {
  // 节点指向
  node: Rc<RefCell<FileNode>>,
  // 被引用的节点path
  reference_path: Rc<RefCell<Vec<String>>>,
}

impl FileNodeForHash {
  pub fn new(node: Rc<RefCell<FileNode>>, reference_path: Vec<String>) -> FileNodeForHash {
    FileNodeForHash {
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

pub struct ExcludeChecker<'a> {
  rules: Rc<Vec<Rc<Builder<'a>>>>,
}

impl<'a> ExcludeChecker<'a> {
  pub fn new<'b>(excludes: &'b Vec<String>) -> ExcludeChecker<'b> {
    let rule_checkers: Vec<Rc<Builder<'b>>> = excludes
      .iter()
      .map(|rule| return Rc::new(Builder::new(rule)))
      .collect();
    return ExcludeChecker {
      rules: Rc::new(rule_checkers),
    };
  }

  pub fn check(&self, path: &String) -> bool {
    let mut result = true;
    let rules = &self.rules;
    for rule in rules.iter() {
      let check_result = rule
        .build_glob()
        .expect("fail to load exclude rule in check method")
        .is_match(path);
      if check_result != result {
        result = check_result;
        break;
      }
    }
    return result;
  }
}

pub fn scan_by_entry(
  entry: String,
  alias_config: HashMap<String, String>,
  npm_packages: Vec<String>,
  excludes: Vec<String>,
) -> Result<(), Error> {
  // 用于过滤exclude规则文件
  let exclude_checker = ExcludeChecker::new(&excludes);
  // 用于收集npm package依赖引用
  let mut npm_map = NpmPackages::new(npm_packages);
  // 存储所有解析出来的fileNode的列表
  let whole_file_nodes_for_hash: Rc<RefCell<Vec<RefCell<FileNodeForHash>>>> =
    Rc::new(RefCell::new(vec![]));
  // file_hash_map可以通过路径获取索引，然后去whole_file_nodes_for_hash找到真正的唯一的fileNode
  let mut file_hash_map: HashMap<String, Box<usize>> = HashMap::new();


  let mut file_node_hashmap_marker = |node_target: &mut FileNode, node_paths:FileNodePaths| {
    let file_node_for_hash = FileNodeForHash::new(Rc::new((RefCell::new(node_target.clone()))), Vec::new());
    whole_file_nodes_for_hash
      .borrow_mut()
      .push(RefCell::new(file_node_for_hash));
    let index = whole_file_nodes_for_hash.borrow_mut().len() - 1;
    let enable_paths: Vec<String> = get_enbale_paths(&node_paths);
    for enable_path in enable_paths {
      file_hash_map.insert(enable_path, Box::new(index));
    }
  };


  let mut root_file_node = FileNode::new(entry);
  let mut parser = parser::Parser::new(alias_config, npm_map);
  dfs(&mut root_file_node, &mut parser, &mut file_node_hashmap_marker)?;

  let file_nodes_count = whole_file_nodes_for_hash.borrow().len();
  let mut loop_cursor: usize = 0;
  loop {
    if loop_cursor == file_nodes_count {
      break;
    }
    mark_reference(loop_cursor, &whole_file_nodes_for_hash, &file_hash_map);
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
  whole_file_nodes_for_hash: &Rc<RefCell<Vec<RefCell<FileNodeForHash>>>>,
  file_hash_map: &HashMap<String, Box<usize>>,
) {
  let node = whole_file_nodes_for_hash
    .borrow_mut()
    .get(cursor)
    .expect("fail to get deps by whole_file_nodes_for_hash")
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
        .expect("fail to get deps by whole_file_nodes_for_hash")
        .clone();
      let target_index = file_hash_map.get(&dep);
      match target_index {
        Some(index) => {
          whole_file_nodes_for_hash
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

fn dfs<F> (
  current_node: &mut FileNode,
  parser: &mut parser::Parser,
  reference_marker:&mut F
) -> Result<(), Error>  where F: FnMut(&mut FileNode, FileNodePaths)  {
let current_node_path = current_node.file_path.clone();
  for file in fs::read_dir(current_node.get_path())? {
    let path_buffer = file?.path();
    let file_node_paths = normalize_file_node_path(&current_node_path, &path_buffer);
    let FileNodePaths {
        normal_path: _,
        file_name:_,
        absolute_path:_,
        absolute_path_with_file_name,
      } = &file_node_paths;
    // 创建文件节点
    let file_node_path: String = absolute_path_with_file_name.to_string();
    let mut child = FileNode::new(file_node_path);
    if child.is_folder {
      dfs(&mut child, parser, reference_marker)?;
    } else {
      let deps: Vec<String> = parser.parse_deps_by_file_name(&mut child);
      child.set_deps(deps);
      reference_marker(&mut child,file_node_paths);
    }
    child.set_parent(current_node.file_path.clone());
    current_node.insert_child(child);
  }
  Ok(())
}
