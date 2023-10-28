//! A simple and fast webserver.
//!
//! `crane-webserver` proves the tools you need to quickly build
//! a webserver.
//!
//! Don't ask me why I named it `crane`!!!
//!
//! # How it works?
//! 
//! At its core, `crane-webserver` contains a `WebServer`.
//! The `WebServer` is a "builder", which you can use to setup
//! the routes and stuff and then call the `start` function
//! to run the web server.
//!
//! # Examples
//! 
//! A basic web server that serves "Hello, World!"
//! ```rs
//! use crane_webserver::webserver::WebServer;
//! fn main() {
//!     let server = WebServer::bind("127.0.0.1:8888").route("/", root);
//!     server.start();
//! }
//!
//! fn root(_: Query) -> Response {
//!     ResponseBuilder::new()
//!         .status(200)
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

pub(crate) mod response;
pub(crate) mod webserver;

pub type Query = std::collections::HashMap<String, Vec<String>>;

// TODO: use environment variable
pub(crate) const NUM_THREADS: u32 = 4;
