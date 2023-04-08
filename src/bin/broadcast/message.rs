use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "broadcast")]
pub struct GenerateRequest {
    pub message: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "broadcast_ok")]
pub struct GenerateResponse {
    pub in_reply_to: u64,
}
