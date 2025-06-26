use std::{collections::HashMap, io::Read, net::TcpStream, rc::Rc};

use crate::http::enums::{HttpMethodEnum};

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethodEnum,
    pub host: String,
    pub path: String,
    pub path_params: Rc<HashMap<String, String>>,
    pub headers: Rc<HashMap<String, String>>,
    pub data: String,
}

impl Request {
    pub fn new(stream_data: Vec<String>) -> Self {
        let mut stream_data_first_part = {
            stream_data.first()
                .unwrap()
                .split("\r\n")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };

        let http_first_part = stream_data_first_part
            .first()
            .unwrap()
            .clone()
            .trim()
            .to_string();
        stream_data_first_part.remove(0);

        let method: HttpMethodEnum;
        let path: String;
        let path_params: HashMap<String, String>;
        {
            let request_string_split = http_first_part
                .split(" ")
                .collect::<Vec<&str>>();

            let string_method = request_string_split
                .first()
                .unwrap()
                .to_string();
            method = Self::build_method(string_method);

            let path_data = request_string_split
                .get(1)
                .unwrap()
                .split("?")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (path, path_params) = Self::build_path_and_params(path_data);
        }

        let headers = Self::build_headers(stream_data_first_part);
        let host = Self::build_host(&headers);

        println!("\n\nRequest Data Length: {}\n\n{:?}\n\n\n\n\n", stream_data.len(), &stream_data);

        let data = stream_data.last().unwrap().clone();

        Self {
            method,
            host,
            path,
            path_params: Rc::new(path_params),
            headers: Rc::new(headers),
            data,
        }
    }

    fn build_method(method: String) -> HttpMethodEnum {
        HttpMethodEnum::from(method)
    }

    fn build_host(headers: &HashMap<String, String>) -> String {
        headers.get(&String::from("host")).unwrap().to_string()
    }

    fn build_path_and_params(raw_path_data: Vec<String>) -> (String, HashMap<String, String>) {
        let path = raw_path_data
            .first()
            .unwrap()
            .to_string();

        let mut path_params = HashMap::<String, String>::new();
        if raw_path_data.len() > 1 {
            let path_params_raw_iterator = raw_path_data
            .get(1)
            .unwrap()
            .split("&");
            for path_param_raw in path_params_raw_iterator {
                let path_param = path_param_raw
                    .split("=")
                    .collect::<Vec<&str>>();

                path_params.insert(
                    path_param.get(0).unwrap().to_string(),
                    path_param.get(1).unwrap().to_string()
                );
            }
        }

        (path, path_params)
    }

    fn build_headers(raw_headers: Vec<String>) -> HashMap<String, String> {
        let mut headers: HashMap<String, String> = HashMap::new();
        for header in raw_headers.iter() {
            let formatted_header = header.to_ascii_lowercase();

            let mut parsed_header = formatted_header
                .split(":")
                .collect::<Vec<&str>>();

            let header_key = parsed_header
                .first()
                .unwrap()
                .trim()
                .to_string();
            parsed_header.remove(0);

            let header_value = parsed_header
                .join(":")
                .trim()
                .to_string();

            headers.insert(
                header_key,
                header_value
            );
        }

        headers
    }
}

pub fn get_stream_data(stream: &mut TcpStream) -> Result<Vec<String>, ()> {
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer[..]).unwrap();

    let result_data = buffer.map(|byte| (byte as char).to_string())
        .join("")
        .split("\r\n\r\n")
        .map(|s| s.to_string())
        .collect::<Vec::<String>>();

    Ok(result_data)
}
