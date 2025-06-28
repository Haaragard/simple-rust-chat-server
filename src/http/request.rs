use std::ops::Deref;
use std::{collections::HashMap, io::Read, net::TcpStream, rc::Rc};

use regex::Regex;

use crate::http::consts::*;
use crate::http::enums::{HttpMethodEnum, HttpContentTypeEnum};

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethodEnum,
    pub host: String,
    pub path: String,
    pub path_params: Rc<HashMap<String, String>>,
    pub headers: Rc<HashMap<String, String>>,
    pub content_type: HttpContentTypeEnum,
    pub data: HashMap<String, String>,
}

impl Request {
    pub fn new(stream_data: Vec<String>) -> Self {
        let mut stream_data_first_part = {
            stream_data.first()
                .unwrap()
                .split(CRLF)
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
                .split(QUERY_PARAM_START)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            (path, path_params) = Self::build_path_and_params(path_data);
        }

        let headers = Self::build_headers(stream_data_first_part);
        let host = Self::build_host(&headers);

        let content_type = Self::build_content_type(&headers);
        
        let stream_second_part_as_string = {
            let mut stream_clone = stream_data.clone();
            stream_clone.remove(0);

            stream_clone.join("")
        };
        let data = Self::build_data(
            &content_type,
            stream_second_part_as_string
        );

        Self {
            method,
            host,
            path,
            path_params: Rc::new(path_params),
            headers: Rc::new(headers),
            content_type,
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
            .split(QUERY_PARAM_SEPARATOR);
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

    fn build_content_type(headers: &HashMap<String, String>) -> HttpContentTypeEnum {
        let raw_content_type = headers
            .get(&String::from("content-type"))
            .unwrap();

        HttpContentTypeEnum::from(raw_content_type.clone())
    }

    fn build_data(content_type: &HttpContentTypeEnum, raw_data: String) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        match content_type {
            HttpContentTypeEnum::FormData(content) => {
                // Get raw data as string
                let data_as_string = raw_data
                    .split(content.boundary_end.deref())
                    .nth(0)
                    .unwrap();

                // Split string as Vec<String>
                let data = data_as_string.replace(content.boundary_start.deref(), "")
                    .split(CRLF)
                    .map(
                        |string| string
                            .replace("--", "")
                            .replace("Content-Disposition: form-data;", "")
                    )
                    .collect::<Vec<String>>()
                    .iter()
                    .map(|values| {
                        let trimmed_string = values.trim();

                        let key_regex_start = Regex::new("(name=)(\\\")").unwrap();
                        let key_regex_end = Regex::new("(\\\")(.*)").unwrap();

                        let mut key = key_regex_start.replace(trimmed_string, "").to_string();
                        key = key_regex_end.replace(&key, "").to_string();

                        let value_regex = Regex::new("(name=)(\\\")(.*)(\\\")").unwrap();
                        let value = value_regex.replace(trimmed_string, "").to_string();

                        (key, value)
                    })
                    .filter(|value| !value.0.is_empty())
                    .collect::<Vec<(String, String)>>();

                data.iter().for_each(|values| {
                    result.insert(values.0.clone(), values.1.clone());
                });
            },
            _ => {},
        };

        result
    }
}

pub fn get_stream_data(stream: &mut TcpStream) -> Result<Vec<String>, ()> {
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer[..]).unwrap();

    let result_data = buffer.map(|byte| (byte as char).to_string())
        .join("")
        .split(DOUBLE_CRLF)
        .map(|s| s.to_string())
        .collect::<Vec::<String>>();

    Ok(result_data)
}
