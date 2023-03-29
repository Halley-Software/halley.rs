use std::collections::HashMap;

pub struct Request {
    pub remote_address: String,
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub data: Vec<String>
}

impl Request {

}