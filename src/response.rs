use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseType{
    pub name: String,
    pub time: String,
    pub payload: String,
}