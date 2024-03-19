use std::fs;
use std::io;

use crate::models::Password;
use crate::utils;
extern crate dirs;

pub fn exist() -> io::Result<()> {
    fs::File::open(utils::path())?;
    Ok(())
}

pub fn write(password: Password) -> io::Result<()> {
    match utils::check_existing_alias(password.alias.clone()) {
        Ok(_) => {
            let path = utils::path();
            let json_password = serde_json::to_string(&password)?;
            let contents = fs::read_to_string(path.clone())? + "\n" + json_password.as_str();
            fs::write(path, contents)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn read() -> io::Result<Vec<String>> {
    let path = utils::path();
    let contents_str = fs::read_to_string(path)?;
    let mut contents: Vec<String> = contents_str.split("\n").map(|s| s.to_string()).collect();
    contents.remove(0);
    Ok(contents)
}
