use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
    pub number: i32,
}
