use crate::{file, models, utils};
use ansi_term::Style;
use std::{fs, io};
extern crate dirs;

pub fn init() -> io::Result<()> {
    match file::exist() {
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already exists",
        )),
        Err(_) => {
            let master_pass = utils::master_password();
            match master_pass {
                Ok(_password) => {
                    let home = dirs::home_dir()
                        .expect("Home dir not found")
                        .to_str()
                        .expect("Can not convert to str")
                        .to_string();
                    fs::create_dir(home.clone() + "/rustPM")
                        .expect("Error creating the rpm dir on home directory");
                    let file = fs::File::create(utils::path());
                    match file {
                        Ok(_file) => {
                            // let master = "Master".to_string();
                            println!("passwords.json succesfull created in HOME directory");
                            // save(&master, &password)?;
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
}

pub fn save(alias: &String, password: &String) -> io::Result<()> {
    let new_password = models::Password {
        alias: alias.to_string(),
        password: password.to_string(),
    };
    match file::exist() {
        Ok(_) => {
            file::write(new_password)?;
            println!(
                "The password was succesfully saved with the alias {}!",
                Style::new().bold().paint(alias)
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn list() -> io::Result<()> {
    match file::exist() {
        Ok(_) => match file::read() {
            Ok(contents) => {
                for password in contents {
                    let p: models::Password = serde_json::from_str(&password)?;
                    println!("{:?}", p);
                }
                Ok(())
            }
            Err(_) => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "You don't have any password to list",
            )),
        },
        Err(e) => Err(e),
    }
}

pub fn find(alias: &String) -> io::Result<()> {
    match file::exist() {
        Ok(_) => file::find_get(alias, 'f'),
        Err(e) => Err(e),
    }
}

pub fn get(alias: &String) -> io::Result<()> {
    match file::exist() {
        Ok(_) => file::find_get(alias, 'g'),
        Err(e) => Err(e),
    }
}

pub fn delete(alias: &String) -> io::Result<()> {
    match file::exist() {
        Ok(_) => match utils::chech_master_password() {
            Ok(_) => file::delete(alias),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
