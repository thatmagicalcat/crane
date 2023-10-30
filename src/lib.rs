//! A simple and fast webserver.
//!
//! `crane-webserver` proves the tools you need to quickly build
//! a webserver.
//!
//! # How it works?
//! 
//! At its core, `crane-webserver` contains a `WebServer`.
//! The `WebServer` is a "builder", which takes a closure
//! which is responsible for mapping different functions for
//! different paths, then call the `start` function
//! to start the web server.
//!
//! # Examples
//! 
//! A basic web server that serves "Hello, World!"
//! ```rust
//! use crane_webserver::webserver::WebServer;
//!
//! fn main() {
//!     let server = WebServer::bind("127.0.0.1:8888", |path, _query| {
//!         match path.as_str() {
//!             "/" => root()
//!             _ => ResponseBuilder::new().build()
//!         }
//!     }).unwrap();
//!
//!     server.start();
//! }
//!
//! fn root() -> Response {
//!     ResponseBuilder::new()
//!         .status(HttpStatus::OK)
//!         .header("Content-Type", "text/plain")
//!         .body("Hello, World!")
//!         .build()
//! }
//! ```
//!
//! Run the program and then open your web browser
//! goto `http://localhost:8888/` and see the server in action!

pub use webserver::WebServer;
pub use response::{Response, ResponseBuilder};
pub use status::HttpStatus;

pub(crate) mod response;
pub(crate) mod webserver;
pub(crate) mod status;

pub type Query = std::collections::HashMap<String, Vec<String>>;

// TODO: use environment variable
pub(crate) const NUM_THREADS: u32 = 4;
