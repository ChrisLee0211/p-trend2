use std::collections::HashMap;
use std::{fs, io::Error, env};

pub struct FileNode {
    file_path: String,
    file_name: String,
    is_folder:bool,
    deps:Vec<DependenceNode>,
    parent: Box<FileNode>,
    children:Vec<Box<FileNode>>
}

pub struct DependenceNode {
    dep_path:String,
}

pub fn scan_by_entry(entry: String, alias_config:HashMap<String, String>, excludes:Vec<String>) -> Result<(), Error> {
    for file in fs::read_dir(entry)? {
        let file = file?;
        let path = file.path();

        let path_str = path.to_str().expect("msg");
        let metadata = fs::metadata(&path)?;
        let dir = env::current_dir()?.as_os_str().to_os_string();
        println!("{:?}",&metadata);
        println!("{:?}", dir);
        println!("{:?}", path_str);
    }
    Ok(())
}