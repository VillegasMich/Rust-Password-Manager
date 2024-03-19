use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    pub alias: String,
    pub password: String,
}
