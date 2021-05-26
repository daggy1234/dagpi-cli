use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    #[serde(rename(deserialize = "uu"))]
    pub uuid: String,
    #[serde(rename(deserialize = "appname"))]
    pub name: String,
    #[serde(rename(deserialize = "appdescription"))]
    pub description: String,
    #[serde(rename(deserialize = "appurl"))]
    pub url: String,
    #[serde(rename(deserialize = "appuserid"))]
    pub user_id: String,
    pub approved: bool,
    pub premium: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppParent {
    pub app: App,
}
