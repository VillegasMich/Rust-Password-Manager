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
    let path = utils::path();
    let json_password = serde_json::to_string(&password)?;
    let contents = fs::read_to_string(path.clone())? + "\n" + json_password.as_str();
    fs::write(path, contents)?;
    Ok(())
}

// fn check_existing_alias(alias: String) -> {
//
// }