use crate::models;
use std::io;

use crate::{file, models::Password};
extern crate dirs;
pub fn path() -> String {
    let home: String = dirs::home_dir()
        .expect("Home dir not found")
        .to_str()
        .expect("can not convert to str")
        .to_string();
    home + "/rustPM/passwords.json"
}

pub fn master_password() -> io::Result<String> {
    //TODO set master password security
    println!("Please enter the master password: ");
    let mut master_1 = String::new();
    let _ = io::stdin().read_line(&mut master_1);
    println!("Please enter the master password again: ");
    let mut master_2 = String::new();
    let _ = io::stdin().read_line(&mut master_2);
    if master_1 != master_2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "The passwords are different, please try again",
        ))
    } else {
        Ok(master_1.trim().to_string())
    }
}

pub fn chech_master_password() -> io::Result<()> {
    //TODO set master password security
    println!("Please enter the master password: ");
    let mut master = String::new();
    let _ = io::stdin().read_line(&mut master);
    match file::read() {
        Ok(contents) => {
            for password in contents {
                let p: models::Password = serde_json::from_str(&password)?;
                if p.alias == "Master".to_string() {
                    if master.trim() == p.password {
                        return Ok(());
                    }
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Incorrect master password",
                    ));
                }
            }
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Master password not found",
            ))
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "You don't have any passwords",
        )),
    }
}

pub fn check_existing_alias(pos_alias: String) -> io::Result<()> {
    let contents = file::read()?;
    if contents.len() > 0 {
        for password in contents {
            let p: Password = serde_json::from_str(&password)?;
            if p.alias == pos_alias {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "The alias already exists",
                ));
            };
        }
    }
    Ok(())
}
