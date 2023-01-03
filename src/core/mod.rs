use std::collections::HashMap;
use std::{fs, io::Error, env};
use crate::utils::{Stack, get_file_name_by_path};

#[derive(Debug,Clone)]
pub struct FileNode {
    file_path: String,
    file_name: String,
    is_folder:bool,
    deps:Vec<String>,
    parent: Option<Box<FileNode>>,
    children:Vec<Box<FileNode>>
}

impl FileNode {
    pub fn new(path:String, name:String, is_folder:bool) -> FileNode {
        FileNode { file_path: path, file_name: name, is_folder, deps: vec![], parent: None, children: vec![] }
    }
}


pub struct FileNodeForHash {
    // 节点指向
    node: Box<FileNode>,
    // 被引用的节点path
    referencePath: Vec<String>
}

pub fn scan_by_entry(entry: String, alias_config:HashMap<String, String>, excludes:Vec<String>) -> Result<(), Error> {
    let file_hash_map:HashMap<String, FileNodeForHash> = HashMap::new();
    let mut stack:Stack<&FileNode> = Stack::new();

    let entry_file_name = get_file_name_by_path(&entry);
    let mut root_file_node = FileNode::new(entry.clone(), entry_file_name, true);

    let mut node_cursor = &root_file_node;
    stack.push(&root_file_node);

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
            let file_node = FileNode::new(path_str,file_name,true);
            if (is_folder) {
                stack.push(&file_node);
            } else {

            }
        }
    }

   
    Ok(())
}