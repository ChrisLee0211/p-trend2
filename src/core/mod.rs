use std::collections::HashMap;
use std::{fs, io::Error, env, path};
use crate::utils::{Stack, get_file_name_by_path, get_enbale_paths, normalize_file_node_path, FileNodePaths};

mod  parser;

#[derive(Debug,Clone)]
pub struct FileNode {
    file_path: String,
    file_name: String,
    is_folder:bool,
    deps:Vec<String>,
    parent_path: String,
    children:Vec<Box<FileNode>>
}

impl FileNode {
    pub fn new(path:String, name:String, is_folder:bool) -> FileNode {
        FileNode { file_path: path, file_name: name, is_folder, deps: vec![], parent_path: String::from(""), children: vec![] }
    }

    pub fn set_parent(&mut self, path_string:String) {
        self.parent_path = path_string;
    }

    pub fn insert_child(&mut self, node:Box<FileNode>) {
        self.children.push(node);
    }

    pub fn set_deps(&mut self, deps:Vec<String>) { 
        self.deps = deps;
    }
}

#[derive(Debug)]
pub struct FileNodeForHash {
    // 节点指向
    node: Box<FileNode>,
    // 被引用的节点path
    referencePath: Vec<String>
}


impl FileNodeForHash {
    pub fn new(node:Box<FileNode>, referencePath:Vec<String>) -> FileNodeForHash {
        FileNodeForHash{node,referencePath}
    }
}

pub fn scan_by_entry(entry: String, alias_config:HashMap<String, String>, excludes:Vec<String>) -> Result<(), Error> {
    // 存储所有解析出来的fileNode的列表
    let mut whole_file_nodes_for_hash:Vec<FileNodeForHash> = vec![];
    // file_hash_map可以通过路径获取索引，然后去whole_file_nodes_for_hash找到真正的唯一的fileNode
    let mut file_hash_map:HashMap<String, Box<usize>> = HashMap::new();
    let mut stack:Stack<Box<FileNode>> = Stack::new();

    let entry_file_name = get_file_name_by_path(&entry);
    let mut root_file_node = FileNode::new(entry.clone(), entry_file_name, true);

    let mut node_cursor = root_file_node.clone();
    stack.push(Box::new(root_file_node));

    while stack.len > 0 {
        let mut current_node = stack.pop().expect("fail to pop file node in stack");
        let current_node_path = &current_node.file_path;
        for file in fs::read_dir(current_node_path)? {
            let file = file?;
            let path_buffer = file.path();
            let file_node_paths = normalize_file_node_path(&current_node_path,&path_buffer);
            let FileNodePaths {normal_path, file_name, absolute_path, absolute_path_with_file_name} = &file_node_paths;
            let metadata = fs::metadata(&path_buffer)?;
            let is_folder = metadata.file_type().is_dir();
            let mut file_node = FileNode::new(absolute_path_with_file_name.clone(),file_name.clone(),true);
            file_node.set_parent(node_cursor.file_path.clone());
            if is_folder {
                stack.push(Box::new(file_node.clone()));
                node_cursor = file_node;
            } else {
                let file_path_clone = absolute_path_with_file_name.clone();
                let deps:Vec<String> = parser::parse_deps_by_file_name(&file_path_clone);
                println!("deps ==>{:?},file name ===>{:?} , path ===>{:?}, absoluted path ===> {:?}",&deps,&file_name, &file_path_clone, &absolute_path_with_file_name);
                let reference_path:Vec<String> = vec![];
                file_node.set_deps(deps);
                let file_node_for_hash = FileNodeForHash::new(Box::new(file_node.clone()),reference_path);
                whole_file_nodes_for_hash.push(file_node_for_hash);
                let index = whole_file_nodes_for_hash.len() - 1;
                let enable_paths:Vec<String> = get_enbale_paths(&file_node_paths);
                for enable_path in enable_paths {
                    file_hash_map.insert(enable_path, Box::new(index));
                };
            }
        }
    }

   
    Ok(())
}