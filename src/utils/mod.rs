use std::{fs};
use std::io::{Error};

/**
 * 读取文件内容
 */
pub fn read_file_to_string(path:&String) -> Result<String,Error> {
    fs::read_to_string(path)
}
