use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientToken {
    pub id: i32,
    pub client_id: String,
    pub name: String,
    pub token: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TomlDump {
    pub cli_token: ClientToken,
}
