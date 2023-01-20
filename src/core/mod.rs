use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{fs, io::Error, env, path};
use crate::utils::{Stack, get_file_name_by_path, get_enbale_paths, normalize_file_node_path, FileNodePaths};

mod parser;

#[derive(Debug,Clone)]
pub struct FileNode {
    file_path: String,
    file_name: String,
    is_folder:bool,
    deps: RefCell<Vec<String>>,
    parent_path: RefCell<String>,
    children: RefCell<Vec<Rc<RefCell<FileNode>>>>
}

impl FileNode {
    pub fn new(path:String, name:String, is_folder:bool) -> FileNode {
        FileNode { file_path: path, file_name: name, is_folder, deps: RefCell::new(vec![]), parent_path: RefCell::new(String::from("")), children: RefCell::new(vec![]) }
    }

    pub fn set_parent(&mut self, path_string:String) {
        self.parent_path.borrow_mut().push_str(&path_string);
    }

    pub fn insert_child(&mut self, node:Rc<RefCell<FileNode>>) {
        self.children.borrow_mut().push(node);
    }

    pub fn set_deps(&mut self, deps:Vec<String>) {
        for dependence_path in deps.iter() {
            self.deps.borrow_mut().push(dependence_path.to_string())
        }
    }
}

#[derive(Debug)]
pub struct FileNodeForHash {
    // 节点指向
    node: Rc<RefCell<FileNode>>,
    // 被引用的节点path
    referencePath: Rc<RefCell<Vec<String>>>
}


impl FileNodeForHash {
    pub fn new(node:Rc<RefCell<FileNode>>, reference_path:Vec<String>) -> FileNodeForHash {
        FileNodeForHash{node,referencePath: Rc::new(RefCell::new(reference_path))}
    }

    pub fn add_reference(&mut self, path:String) {
        self.referencePath.borrow_mut().push(path);
    }
}

pub fn scan_by_entry(entry: String, alias_config:HashMap<String, String>,npm_packages:Vec<String> , excludes:Vec<String>) -> Result<(), Error> {
    // 存储所有解析出来的fileNode的列表
    let mut whole_file_nodes_for_hash:Vec<FileNodeForHash> = vec![];
    // file_hash_map可以通过路径获取索引，然后去whole_file_nodes_for_hash找到真正的唯一的fileNode
    let mut file_hash_map:HashMap<String, Box<usize>> = HashMap::new();
    let mut stack:Stack<Rc<RefCell<FileNode>>> = Stack::new();

    let entry_file_name = get_file_name_by_path(&entry);
    let root_file_node:Rc<RefCell<FileNode>> = Rc::new(RefCell::new(FileNode::new(entry.clone(), entry_file_name, true)));


    stack.push(root_file_node.clone());

    while stack.len > 0 {
        let current_node = stack.pop().expect("fail to pop file node in stack");

        let current_node_path:String = current_node.borrow_mut().file_path.clone();
        for file in fs::read_dir(current_node_path.clone())? {
            let file = file?;
            let path_buffer = file.path();
            let file_node_paths = normalize_file_node_path(&current_node_path,&path_buffer);
            let FileNodePaths {normal_path:_, file_name, absolute_path:_, absolute_path_with_file_name} = &file_node_paths;

            let is_folder = fs::metadata(&path_buffer)?.file_type().is_dir();
            let file_node = Rc::new(RefCell::new(FileNode::new(absolute_path_with_file_name.clone(),file_name.clone(),true)));
            current_node.borrow_mut().insert_child(file_node.clone());
            file_node.borrow_mut().set_parent(current_node_path.clone());
            if is_folder {
                stack.push(file_node.clone());
            } else {
                let file_path_clone = absolute_path_with_file_name.clone();
                let deps:Vec<String> = parser::parse_deps_by_file_name(&file_path_clone);
                let normalize_deps: Vec<String>= deps.iter()
                .filter(|&dep_path| {
                    let match_path_result = npm_packages.iter().find(|&npm_path| npm_path.eq(dep_path));
                    match match_path_result {
                        Some(path) => {
                            return true;
                        },
                        None => {
                            return false
                        }
                    }
                })
                .map(move |dep_path| {
                    let result:String = String::from("_");

                    result
                }).collect();
                println!("deps ==>{:?},file name ===>{:?} , path ===>{:?}, absoluted path ===> {:?}",&deps,&file_name, &file_path_clone, &absolute_path_with_file_name);
                let reference_path:Vec<String> = vec![];
                file_node.borrow_mut().set_deps(deps);
                let file_node_for_hash = FileNodeForHash::new(file_node.clone(),reference_path);
                whole_file_nodes_for_hash.push(file_node_for_hash);
                let index = whole_file_nodes_for_hash.len() - 1;
                let enable_paths:Vec<String> = get_enbale_paths(&file_node_paths);
                for enable_path in enable_paths {
                    file_hash_map.insert(enable_path, Box::new(index));
                };
            }
        }
    }

    println!("{:?}", root_file_node);
   
    Ok(())
}