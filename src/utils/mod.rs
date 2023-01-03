use std::{fs, path::{Path}};
use std::io::{Error};

/**
 * 读取文件内容
 */
pub fn read_file_to_string(path:&String) -> Result<String,Error> {
    fs::read_to_string(path)
}

/**
 * 从路径中获取文件名称
 */
pub fn get_file_name_by_path(path_string:&String) -> Option<String> {
    let path_ins = Path::new(path_string);
    let file_name = path_ins.file_name()?;
    let result = file_name.to_str()?;
    Some(result.to_string())
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