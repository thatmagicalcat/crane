use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use crate::{response, Query};
use response::Response;

use scoped_threadpool::Pool;
use url::Url;

type Handler = dyn Fn(Query) -> Response + 'static + Send + Sync;

/// A web server.
///
/// This is the core type of this crate, and is used to create a new
/// server and listen for connections.
///
/// # Examples
///
/// A basic web server that serves "Hello, World!"
/// ```rs
/// use crane_webserver::webserver::WebServer;
/// fn main() {
///     let server = WebServer::bind("127.0.0.1:8888").route("/", root);
///     server.start();
/// }
///
/// fn root(_: Query) -> Response {
///     ResponseBuilder::new()
///         .status(200)
///         .header("Content-Type", "text/plain")
///         .body("Hello, World!")
///         .build()
/// }
/// ```
pub struct WebServer {
    listener: TcpListener,
    routes: Vec<(String, Box<Handler>)>,
    default_route: Option<Box<Handler>>,
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
    /// ```rs
    /// use crane_webserver::webserver::WebServer;
    /// fn main() {
    ///     let server = WebServer::bind("127.0.0.1:8888");
    /// }
    /// ```
    pub fn bind<T: ToSocketAddrs>(addr: T) -> std::io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(addr)?,
            routes: Vec::new(),
            default_route: None,
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

    /// The function `func` will be called if the user requests a path
    /// which is not mapped by the `route` function.
    pub fn default_route<F: Fn(Query) -> Response + 'static + Send + Sync>(
        mut self,
        func: F,
    ) -> Self {
        self.default_route = Some(Box::new(func));
        self
    }

    /// Maps a path to a function.
    pub fn route<F: Fn(Query) -> Response + 'static + Send + Sync>(
        mut self,
        path: &str,
        func: F,
    ) -> Self {
        self.routes.push((path.to_string(), Box::new(func)));
        self
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

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let buffer_str = std::str::from_utf8(&buffer).unwrap();
        let path = Self::get_requested_path(buffer_str);

        // Base url and scheme is not used here, that's why arbitrary url and scheme used
        let url = Url::parse(&format!("http://localhost{path}")).unwrap();

        let route_fn = self
            .routes
            .iter()
            .find(|(p, _)| p == url.path())
            .map(|(_, f)| f);

        if route_fn.is_some() || self.default_route.is_some() {
            let function = route_fn.unwrap_or_else(|| self.default_route.as_ref().unwrap());

            let mut query_map: HashMap<String, Vec<String>> =
                HashMap::with_capacity(url.query_pairs().count());
            let query_pairs = url.query_pairs();
            query_pairs.into_iter().for_each(|(k, v)| {
                query_map
                    .entry(k.into_owned())
                    .or_default()
                    .push(v.into_owned())
            });

            let response = function(query_map);

            write!(stream, "{}", response).expect("Failed to write to stream");
            stream.flush().expect("Failed to flush stream");
        }
    }

    fn get_requested_path(request: &str) -> &str {
        let request_line = request.lines().next().unwrap_or("");
        let path = request_line.split_whitespace().nth(1).unwrap_or("/");
        path
    }
}
