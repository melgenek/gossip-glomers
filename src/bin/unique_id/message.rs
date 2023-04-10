use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum GenerateMessage {
    #[serde(rename = "generate")]
    Generate,
    #[serde(rename = "generate_ok")]
    GenerateOk {
        id: String
    },
}
