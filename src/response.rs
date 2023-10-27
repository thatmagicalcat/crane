#[derive(Debug)]
pub struct ResponseBuilder {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        ResponseBuilder {
            status: 200,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn build(self) -> Response {
        Response {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}

pub struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let headers = self
            .headers
            .iter()
            .map(|(key, value)| format!("{key}: {value}"))
            .collect::<Vec<_>>()
            .join("\r\n");

        write!(
            f,
            "HTTP/1.1 {} {}\r\n{headers}\r\n\r\n{}",
            self.status, "OK", self.body
        )
    }
}