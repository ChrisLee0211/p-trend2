use std::{fs, path::{self, Path}};

use crate::utils::get_file_name_by_path;

/// 文件节点
#[derive(Debug, Clone)]
pub struct FileNode {
    name: String,
    path: String,
    deps: Vec<String>,
    children: Vec<FileNode>,
    pub is_folder:bool,
}
impl FileNode {
    pub fn new(path: String) -> FileNode{
        let name = get_file_name_by_path(&path);
        let is_folder = fs::metadata(&path).unwrap().is_dir();
        Self {
            name,
            path,
            is_folder,
            deps: [].to_vec(),
            children: [].to_vec(),
        }
    }

    pub fn add_deps (&mut self, dep: String) {
        self.deps.push(dep);
    }

    pub fn add_child (&mut self, child: FileNode) {
        self.children.push(child);
    }

    pub fn get_path (&mut self) -> &Path {
        return Path::new(&self.path);
    }

    fn set_name (&mut self, value: String) {
        self.name = value;
    }
}