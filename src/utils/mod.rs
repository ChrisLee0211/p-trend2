use std::{fs};
use std::io::{Error};

/**
 * 读取文件内容
 */
pub fn read_file_to_string(path:&String) -> Result<String,Error> {
    fs::read_to_string(path)
}


pub struct Stack <T>{
    top: Option<Box<StackNode<T>>>
}

pub struct  StackNode<T> {
    value: T,
    next: Option<Box<StackNode<T>>>
}

impl<T> StackNode<T> {
    fn new(value:T) -> StackNode<T> {
        StackNode{value, next: None}
    }
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { top: None }
    }

    fn push(&mut self,val:T) {
        let mut node = StackNode::new(val);
        let next = self.top.take();
        node.next = next;
        self.top = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        let result = self.top.take();
        match result {
            Some(mut v) => {
                self.top = v.next.take();
                Some(v.value)
            },
            None => None
        }
    }
}