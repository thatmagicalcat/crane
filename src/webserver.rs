use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use crate::{response, Query};
use response::Response;

use scoped_threadpool::Pool;
use url::Url;

type Handler = dyn Fn(String, Query) -> Response + 'static + Send + Sync;

/// A web server.
///
/// This is the core type of this crate, and is used to create a new
/// server and listen for connections.
///
/// # Examples
///
/// A basic web server that serves "Hello, World!"
/// ```rust
/// use crane_webserver::webserver::WebServer;
/// fn main() {
///     let server = WebServer::bind("127.0.0.1:8888", |path, _query| {
///         match path.as_str() {
///             "/" => root()
///             _ => ResponseBuilder::new().build()
///         }
///     }).unwrap();
///
///     server.start();
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
pub struct WebServer {
    listener: TcpListener,
    route_handler: Box<Handler>,
    read_timeout: Option<Duration>,
}

impl WebServer {
    /// Construct a new WebServer.
    ///
    /// # Errors
    ///
    /// Returns an error if there if it fails to
    /// bind. Most likely when the port is already in use.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crane_webserver::webserver::WebServer;
    /// fn main() {
    ///     let server = WebServer::bind("127.0.0.1:8888", |path, _query| {
    ///         match path.as_str() {
    ///             "/" => root()
    ///             _ => ResponseBuilder::new().build()
    ///         }
    ///     }).unwrap(); 
    ///
    ///     server.start();
    /// }
    /// ```
    pub fn bind<T: ToSocketAddrs, F: Fn(String, Query) -> Response + Send + Sync + 'static>(
        addr: T,
        route_handler: F,
    ) -> std::io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            route_handler: Box::new(route_handler),
            read_timeout: None,
        })
    }

    /// Sets the max reading time for the request.
    ///
    /// If reading the request takes longer than the timeout
    /// than it will simply panic.
    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.read_timeout = Some(timeout);
        self
    }

    /// Returns the local socket address of the listener.
    pub fn get_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }

    /// Start the webserver.
    pub fn start(&self) -> ! {
        let mut incoming = self.listener.incoming();
        let mut pool = Pool::new(crate::NUM_THREADS);

        loop {
            let stream = incoming.next().unwrap();
            let stream = stream.expect("Error handling TCP stream.");

            stream
                .set_read_timeout(self.read_timeout)
                .expect("[Error] Couldn't set read timeout on socket");

            pool.scoped(|scope| {
                scope.execute(|| self.handle_connection(stream));
            });
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        #[cfg(debug)]
        println!("New connection: {}", stream.local_addr().unwrap());

        let mut buffer = [0; 4096];
        stream.read(&mut buffer).unwrap();

        let buffer_str = std::str::from_utf8(&buffer).unwrap();
        let path = Self::get_requested_path(buffer_str);

        // Base url and scheme is not used here, that's why arbitrary url and scheme used
        let url = Url::parse(&format!("http://localhost{path}")).unwrap();

        let mut query_map: HashMap<String, Vec<String>> =
            HashMap::with_capacity(url.query_pairs().count());
        let query_pairs = url.query_pairs();
        query_pairs.into_iter().for_each(|(k, v)| {
            query_map
                .entry(k.into_owned())
                .or_default()
                .push(v.into_owned())
        });

        let response = self.route_handler.as_ref()(url.path().to_string(), query_map);

        write!(stream, "{}", response).expect("Failed to respond");
        stream.flush().expect("Failed to respond");
    }

    fn get_requested_path(request: &str) -> &str {
        let request_line = request.lines().next().unwrap_or("");
        let path = request_line.split_whitespace().nth(1).unwrap_or("/");
        path
    }
}
