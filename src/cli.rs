use crate::handlers;
use clap::*;
use std::io;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "rust password manager - An easy and secure way to manage your passwords")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates passwords file in ~/rustPM/password.json
    Init,
    /// List all passwords
    List,
    /// Save a password; <Master Password> <Alias> <Password>
    Save(Save),
    /// Delete a saved password; <Master Password> <Alias>
    Delete(Delete),
    /// Find a specific saved password; <Alias>
    Find(Find),
    /// Get a specific saved password, and save it on your clipboard; <Master Password> <Alias>
    Get(Get),
}

#[derive(Args)]
pub struct Save {
    /// The master password
    pub master_password: Option<String>,
    /// The password alias
    pub alias: Option<String>,
    /// The password to save
    pub password: Option<String>,
}

#[derive(Args)]
pub struct Delete {
    /// The master password
    pub master_password: Option<String>,
    /// The alias of the password to delete, if it is located
    pub alias: Option<String>,
    // #[arg(short = 'f', long = "force")]
    // pub force_delete: bool,
}

#[derive(Args)]
pub struct Find {
    /// The alias of the password to find, if it is located
    pub alias: Option<String>,
}

#[derive(Args)]
pub struct Get {
    /// The master password
    pub master_password: Option<String>,
    /// The alias of the password to get, if it is located
    pub alias: Option<String>,
}

pub fn parse() -> io::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => handlers::init(),
        Some(Commands::List) => handlers::list(),
        Some(Commands::Save(save)) => match (
            save.master_password.clone(),
            save.alias.clone(),
            save.password.clone(),
        ) {
            (Some(ref _master), Some(ref alias), Some(ref password)) => {
                handlers::save(alias, password)
            }
            (_, None, Some(ref _password)) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found.",
            )),
            (_, Some(ref _alias), None) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Password argument not found.",
            )),
            (None, Some(ref _alias), Some(ref _password)) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "master password argument not found.",
            )),
            (_, None, None) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias and password arguments not found.",
            )),
        },
        Some(Commands::Delete(delete)) => {
            match (delete.master_password.clone(), delete.alias.clone()) {
                (Some(ref _master), Some(ref alias)) => handlers::delete(alias),
                _ => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid arguments.",
                )),
            }
        }
        Some(Commands::Find(find)) => match find.alias {
            Some(ref alias) => handlers::find(alias),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found",
            )),
        },
        Some(Commands::Get(get)) => match (get.master_password.clone(), get.alias.clone()) {
            (Some(ref _master), Some(ref alias)) => handlers::get(alias),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found",
            )),
        },
        None => Ok(()),
    }
}
