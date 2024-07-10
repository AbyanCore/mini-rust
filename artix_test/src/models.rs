use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Status<'a> {
    pub status: &'a  str,
    pub code: u8
}

#[derive(Serialize,Deserialize)]
pub struct Response<'a> {
    pub data: &'a str,
    pub message: &'a str,
    pub code: u8
}