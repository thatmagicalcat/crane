use crate::HttpStatus;

/// Response builder.
///
/// If you don't set any fields of the builder,
/// it will return a 404 Not Found response
/// with no header or body.
///
/// # Examples
///
/// ```no_run
/// use crane_webserver::*;
///
/// fn main() {
///     // .. server setup
/// }
///
/// fn root() -> Response {
///     ResponseBuilder::new()
///         .status(HttpStatus::OK)
///         .header("Content-Type", "text/plain")
///         .body("Hello, World!")
///         .build()
/// }
/// ```
#[derive(Debug)]
pub struct ResponseBuilder {
    status: HttpStatus,
    headers: Vec<(String, String)>,
    body: String,
}

impl ResponseBuilder {
    /// Construct a new `ResponseBuilder`.
    pub fn new() -> Self {
        ResponseBuilder {
            status: HttpStatus::Not_Found,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    /// Set the html status code.
    pub fn status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }

    /// Set a header key-value pair.
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    /// Set the body of the response.
    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    /// Consume the `ResponseBuilder` and construct a `Response`.
    pub fn build(self) -> Response {
        Response {
            status: self.status,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// The response which will be sent when requested.
pub struct Response {
    status: HttpStatus,
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
            "HTTP/1.1 {}\r\n{headers}\r\n\r\n{}",
            self.status.get_string(), self.body
        )
    }
}