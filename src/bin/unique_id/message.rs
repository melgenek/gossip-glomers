use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "generate")]
pub struct GenerateRequest {
    pub msg_id: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "generate_ok")]
pub struct GenerateResponse {
    pub in_reply_to: u64,
    pub id: String,
}
