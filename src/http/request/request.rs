use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub http_version: String,
}

impl Request {
    pub fn new() -> Request {
        Request {
            method: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            body: String::new(),
            http_version: String::new(),
        }
    }

    pub fn parse_request(&mut self, request: &str) {
        let mut headers = HashMap::new();
        let mut lines = request.lines();

        // Skip the request line
        let request_line = lines.next();

        let request_line_parts;
        let mut method: String = String::from("");
        let mut path: String = String::from("");
        let mut http_version: String = String::from("");
        if request_line.is_some() {
            request_line_parts = request_line.unwrap().split_whitespace();
            method = request_line_parts
                .clone()
                .nth(0)
                .unwrap_or("N/A")
                .to_string();
            path = request_line_parts.clone().nth(1).unwrap_or("/").to_string();
            http_version = request_line_parts
                .clone()
                .nth(2)
                .unwrap_or("N/A")
                .to_string();
        }

        // Parse headers
        for line in lines.by_ref() {
            if line.is_empty() {
                break; // End of headers
            }

            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        self.headers = headers;
        // We asume remaining lines are the body
        self.body = lines.collect::<Vec<&str>>().join("\n");
        self.method = method.to_string();
        self.path = path.to_string();
        self.http_version = http_version.to_string();
    }
}
