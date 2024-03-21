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
    /// Save a password <Alias> <Password>
    Save(Save),
    /// Delete a saved password <Alias>
    Delete(Delete),
    /// Find a specific saved password
    Find(Find),
    /// Get a specific saved password, and save it on your clipboard
    Get(Get),
}

#[derive(Args)]
pub struct Save {
    /// The password alias
    pub alias: Option<String>,
    /// The password to save
    pub password: Option<String>,
}

#[derive(Args)]
pub struct Delete {
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
    /// The alias of the password to get, if it is located
    pub alias: Option<String>,
}

pub fn parse() -> io::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => handlers::init(),
        Some(Commands::List) => handlers::list(),
        Some(Commands::Save(save)) => match (save.alias.clone(), save.password.clone()) {
            (Some(ref alias), Some(ref password)) => handlers::save(alias, password),
            (None, Some(ref _password)) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found",
            )),
            (Some(ref _alias), None) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Password argument not found",
            )),
            (None, None) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias and password arguments not found",
            )),
        },
        Some(Commands::Delete(delete)) => match delete.alias {
            Some(ref alias) => handlers::delete(alias),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found",
            )),
        },
        Some(Commands::Find(find)) => match find.alias {
            Some(ref alias) => handlers::find(alias),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found",
            )),
        },
        Some(Commands::Get(get)) => match get.alias {
            Some(ref alias) => handlers::get(alias),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias argument not found",
            )),
        },
        None => Ok(()),
    }
}
