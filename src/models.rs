use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub alias: String,
    pub password: String,
}
