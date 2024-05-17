use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub key: String,
    pub value: String,
    pub date_added: String,
    pub last_updated_at: String,
}


