use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    #[serde(rename(deserialize = "apikey"))]
    pub api_key: String,
    #[serde(rename(deserialize = "userid"))]
    pub user_id: String,
    pub enhanced: bool,
    pub ratelimit: i8,
    pub totaluses: i8,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenParent {
    pub token: Token,
}
