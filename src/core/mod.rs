use std::collections::HashMap;
use std::{fs, io::Error, env};

pub fn scan_by_entry(entry: String, alias_config:HashMap<String, String>) -> Result<(), Error> {
    for file in fs::read_dir(entry)? {
        let file = file?;
        let path = file.path();

        let metadata = fs::metadata(&path)?;
        let dir = env::current_dir()?.as_os_str().to_os_string();
        println!("{:?}",&metadata);
        println!("{:?}", dir);
    }
    Ok(())
}