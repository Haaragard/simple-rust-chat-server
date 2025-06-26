#[derive(Debug)]
pub enum HttpMethodEnum {
    GET,
    POST,
}

impl HttpMethodEnum {
    pub fn from(http_method_name: String) -> Self {
        match http_method_name.as_str() {
            "GET" => Self::GET,
            _ => panic!("Request method not implemented.")
        }
    }
}
