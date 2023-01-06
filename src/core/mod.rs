use std::collections::HashMap;
use std::{fs, io::Error, env};
use crate::utils::{Stack, get_file_name_by_path};

// mod  parser;

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
    let file_hash_map:HashMap<String, FileNodeForHash> = HashMap::new();
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
            let path = file.path();
    
            let path_str = path.to_str().expect("fail to transfer path to string").to_string();
            let file_name = get_file_name_by_path(&path_str);
            let metadata = fs::metadata(&path)?;
            let is_folder = metadata.file_type().is_dir();
            let mut file_node = FileNode::new(path_str,file_name,true);
            file_node.set_parent(node_cursor.file_path.clone());
            if (is_folder) {
                stack.push(Box::new(file_node.clone()));
                node_cursor = file_node;
            } else {
                // let deps:Vec<String> = parser::parse_deps_by_file_name(&file_name);
                let reference_path:Vec<String> = vec![];
                file_node.set_deps(deps);
                let file_node_for_hash = FileNodeForHash::new(Box::new(file_node.clone()),reference_path);
                file_hash_map.insert(path_str.clone(), file_node_for_hash);
            }
        }
    }

   
    Ok(())
}