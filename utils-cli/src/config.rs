use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub api_version: String,
    pub store: Store,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Store {
    pub list: RequireAuth,
    pub delete: RequireAuth,
    pub clear: RequireAuth,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequireAuth {
    require_auth: bool,
}

impl Default for RequireAuth {
    fn default() -> Self {
        Self { require_auth: true }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            api_version: "v1".to_string(),
            store: Default::default(),
        }
    }
}

impl Config {
    pub fn load() -> Config {
        confy::load("utils", None).unwrap_or_default()
    }
}
