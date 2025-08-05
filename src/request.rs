use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub query_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: &str, path: &str, headers: HashMap<String, String>, body: String) -> Self {
        let (path, query_params) = Self::parse_path_and_query(path);

        Request {
            method: method.to_string(),
            path,
            headers,
            body,
            query_params,
        }
    }

    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }

    pub fn get_query_param(&self, name: &str) -> Option<&String> {
        self.query_params.get(name)
    }

    fn parse_path_and_query(full_path: &str) -> (String, HashMap<String, String>) {
        if let Some((path, query_string)) = full_path.split_once('?') {
            let mut query_params = HashMap::new();

            for param in query_string.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    query_params.insert(
                        Self::url_decode(key),
                        Self::url_decode(value),
                    );
                }
            }

            (path.to_string(), query_params)
        } else {
            (full_path.to_string(), HashMap::new())
        }
    }

    fn url_decode(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars();

        while let Some(ch) = chars.next() {
            match ch {
                '%' => {
                    if let (Some(high), Some(low)) = (chars.next(), chars.next()) {
                        if let (Some(high_digit), Some(low_digit)) =
                            (Self::hex_to_u8(high), Self::hex_to_u8(low)) {
                            let byte = (high_digit << 4) | low_digit;
                            if byte < 128 {
                                result.push(byte as char);
                            } else {
                                let bytes = vec![byte];
                                if let Ok(s) = String::from_utf8(bytes) {
                                    result.push_str(&s);
                                } else {
                                    result.push('%');
                                    result.push(high);
                                    result.push(low);
                                }
                            }
                        } else {
                            result.push('%');
                            result.push(high);
                            result.push(low);
                        }
                    } else {
                        result.push('%');
                    }
                }
                '+' => result.push(' '),
                _ => result.push(ch),
            }
        }

        result
    }

    fn hex_to_u8(ch: char) -> Option<u8> {
        match ch {
            '0'..='9' => Some((ch as u8) - b'0'),
            'A'..='F' => Some((ch as u8) - b'A' + 10),
            'a'..='f' => Some((ch as u8) - b'a' + 10),
            _ => None,
        }
    }
}