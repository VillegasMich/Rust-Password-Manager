use crate::{file, models, utils};
use std::{fs, io};
extern crate dirs;

pub fn init() -> io::Result<()> {
    match file::exist() {
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "File already exists",
        )),
        Err(_) => {
            let home = dirs::home_dir()
                .expect("Home dir not found")
                .to_str()
                .expect("can not convert to str")
                .to_string();
            fs::create_dir(home.clone() + "/rustPM")
                .expect("Error creating the rpm dir on home directory");
            let file = fs::File::create(utils::path());
            match file {
                Ok(_file) => {
                    println!("passwords.json succesfull created in HOME directory");
                    Ok(())
                }
                Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
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
            println!("Saving password {}...", new_password.alias);
            file::write(new_password)?;
            Ok(())
        }
        Err(e) => Err(io::Error::new(io::ErrorKind::NotFound, e)),
    }
}
