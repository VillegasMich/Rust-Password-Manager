use crate::handlers;
use clap::*;
use std::io;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "rust password manager - ab easy and secure way to manage your passwords")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates passwords file in ~/rpm/
    Init,
    /// List all passwords
    List,
    /// Inspects a password
    Inspect(Inspect),
    /// Save a password <Alias> <Password>
    Save(Save),
    // Delete a saved password
    Delete(Delete),
}

#[derive(Args)]
pub struct Inspect {
    /// The password to inspect
    pub password: Option<String>,
    #[arg(short = 'd', long = "decrypted")]
    pub show_decrypted: bool,
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
    /// The password to delete, if it is located
    pub password: Option<String>,
    #[arg(short = 'f', long = "force")]
    pub force_delete: bool,
}

pub fn parse() -> io::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => handlers::init(),
        Some(Commands::List) => Ok(()),
        Some(Commands::Inspect(_password)) => Ok(()),
        Some(Commands::Save(save)) => match (save.alias.clone(), save.password.clone()) {
            (Some(ref alias), Some(ref password)) => handlers::save(alias, password),
            (None, Some(ref _password)) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias not found",
            )),
            (Some(ref _alias), None) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Password not found",
            )),
            (None, None) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Alias and password not found",
            )),
        },
        Some(Commands::Delete(_password)) => Ok(()),
        None => Ok(()),
    }
}
