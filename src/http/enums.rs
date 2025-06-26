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

#[derive(Debug)]
pub struct HttpContentFormData {
    boundary_start: String,
    boundary_end: String,
}

impl HttpContentFormData {
    pub fn from(boundary: String) -> Self {
        // TODO
        Self {
            boundary_start: String::new(),
            boundary_end: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum HttpContentTypeRawEnum {
    Text,
    Json,
}

#[derive(Debug)]
pub enum HttpContentTypeEnum {
    None,
    FormData(HttpContentFormData),
    Raw(HttpContentTypeRawEnum),
}

impl HttpContentTypeEnum {
    pub fn from(content_type: String) -> Self {
        let split_content = content_type
            .split(";")
            .collect::<Vec<&str>>();

        let string_type = split_content
            .first()
            .unwrap()
            .as_ref();
        match string_type {
            "multipart/form-data" => {
                let mut form_data: HttpContentFormData = HttpContentFormData::from(String::from(""));

                let form_data_option = split_content.get(1);
                if form_data_option.is_some() {
                    form_data = HttpContentFormData::from(
                        form_data_option.unwrap().to_string()
                    );
                }

                Self::FormData(form_data)
            },
            "text/plain" => Self::Raw(HttpContentTypeRawEnum::Text),
            "application/json" => Self::Raw(HttpContentTypeRawEnum::Json),
            _ => Self::None,
        }
    }
}
