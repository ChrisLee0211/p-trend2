use std::{fs, path::{Path, PathBuf}};
use std::io::{Error};
use regex::{Regex};

/**
 * 读取文件内容
 */
pub fn read_file_to_string(path:&String) -> Result<String,Error> {
    fs::read_to_string(path)
}

/**
 * 从路径中获取文件名称
 */
pub fn get_file_name_by_path(path_string:&String) -> String {
    let path_ins = Path::new(path_string);
    let file_name = path_ins.file_name().expect("can not resolve entry file name to &OsStr");
    let result = file_name.to_str().expect("fail to transfer &OsStr to &str");
    result.to_string()
}

/**
 * 以当前相对路径转化为绝对路径，失败则返回原路径
 */
pub fn get_file_absolute_path(path_string:&String) -> String {
    let absolute_path_result = Path::canonicalize(Path::new(path_string));
    match absolute_path_result {
        Ok(path_buf) => {
            let absolute_path = path_buf.to_str().expect("fail to transform path buffer to string").to_string();
            return absolute_path;
        },
        Err(err) => {
            path_string.to_string()
        }
    }
}



#[derive(Clone,Debug)]
pub struct FileNodePaths {
   pub absolute_path: String,
   pub absolute_path_with_file_name: String,
   pub normal_path: String,
   pub file_name: String
}

/**
 * 通过当前节点的文件目录和pathBuf计算出绝对路径、文件名等重要信息
 */
pub fn normalize_file_node_path(current_node_path:&String,path_buffer: &PathBuf) -> FileNodePaths {
    let normal_path = path_buffer.as_path().to_str().expect("fail to resolve file path as str").to_string();

    let file_name = path_buffer.file_name().expect("can not resolve entry file name to &OsStr").to_str().expect("fail to transfer &OsStr to &str").to_string();

    let absolute_path_result_buffer = &fs::canonicalize(current_node_path).expect("fail to get absolute path buffer");

    let absolute_path = absolute_path_result_buffer.to_str().expect("fail to transform path buffer to string").to_string();

    let absolute_path_with_file_name = absolute_path_result_buffer.join(&file_name).to_str().expect("fail to transform path buffer to string").to_string();

    let result:FileNodePaths = FileNodePaths{
        absolute_path,
        absolute_path_with_file_name,
        normal_path,
        file_name
    };
    result
}

/**
 * 通过文件路径信息集合计算出该文件被引用时可以使用的有效路径
 * 比如：
 * A文件绝对路径 user/src/utils/index.js; C文件绝对路径 user/src/utils/timezone.js
 * B文件中可以这样引用A： import xx from 'user/src/utils'
 * 也可以这样引用A: import xx from 'user/src/utils/index'
 * 可以这样引用C：import xx from 'user/src/utils/timezone.js'
 * 也可以这样引用C：import xx from 'user/src/utils/timezone'
 */
pub fn get_enbale_paths(file_node_paths: &FileNodePaths) -> Vec<String> {
    let FileNodePaths {normal_path,file_name,absolute_path,absolute_path_with_file_name} = file_node_paths;
    let mut result:Vec<String> = vec![];
    result.push(absolute_path_with_file_name.clone());
    let index_type_reg = Regex::new(r"^index\.").expect("fail to init regex in function: get_enbale_paths ");
    if index_type_reg.is_match(&file_name) {
        result.push(absolute_path.clone());
    } else {
        let file_name_split_list:Vec<&str> = absolute_path_with_file_name.split(".").collect();
        let absolute_path_without_extname = file_name_split_list.get(0).expect("fail to get file name by file_name_split_list").to_string();
        result.push(absolute_path_without_extname.clone());
    }
    result
}

/**
 * 栈相关工具
 */
pub struct Stack <T>{
    top: Option<Box<StackNode<T>>>,
   pub len: i64
}

pub struct  StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>
}

impl<T> StackNode<T> {
   pub fn new(value:T) -> StackNode<T> {
        StackNode{value, next: None}
    }
}

impl<T> Stack<T> {
   pub fn new() -> Stack<T> {
        Stack { top: None, len:0 }
    }

   pub fn push(&mut self,val:T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
        self.len += 1;
    }

   pub fn pop(&mut self) -> Option<T> {
        let result = self.top.take();
        match result {
            Some(mut v) => {
                self.top = v.next.take();
                self.len -= 1;
                Some(v.value)
            },
            None => None
        }
    }
    
}