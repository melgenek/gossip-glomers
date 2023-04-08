use serde::{Deserialize, Serialize};
use gossip_glomers::common::message::req_resp::Request;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "generate")]
pub struct GenerateRequestValue {
    #[serde(flatten)]
    n: (),
}

pub type GenerateRequest = Request<GenerateRequestValue>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "generate_ok")]
pub struct GenerateResponseValue {
    pub id: String,
}
