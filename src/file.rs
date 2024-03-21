use std::fs;
use std::io;

use crate::models;
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
            let mut contents = fs::read_to_string(path.clone())?;
            if !contents.is_empty() {
                contents = contents + "\n" + json_password.as_str();
                fs::write(path, contents)?;
            } else {
                fs::write(path, json_password)?;
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn read() -> io::Result<Vec<String>> {
    let path = utils::path();
    let contents_str = fs::read_to_string(path)?;
    if contents_str.is_empty() {
        let contents: Vec<String> = Vec::new();
        Ok(contents)
    } else {
        let contents: Vec<String> = contents_str.split("\n").map(|s| s.to_string()).collect();
        Ok(contents)
    }
}

pub fn delete(alias: &String) -> io::Result<()> {
    let path = utils::path();
    let mut was_removed = false;
    let mut contents_str = fs::read_to_string(path.clone())?;
    let mut contents: Vec<String> = contents_str.split("\n").map(|s| s.to_string()).collect();
    contents_str = "".to_string();
    let mut i: usize = 0;
    for password in contents.clone() {
        let p: models::Password = serde_json::from_str(&password)?;
        if p.alias == alias.to_string() {
            contents.remove(i);
            was_removed = true;
            println!("Password removed!");
        } else {
            contents_str += password.as_str();
        }
        i += 1;
    }
    if was_removed {
        fs::write(path, contents_str)?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Password not found",
        ))
    }
}
