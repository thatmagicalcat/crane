use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use std::net::{TcpListener, TcpStream};

use crate::{response, QueryParis};
use response::Response;

use scoped_threadpool::Pool;
use url::Url;

type Handler = dyn Fn(QueryParis) -> Response + 'static + Send + Sync;

pub struct WebServer {
    listener: TcpListener,
    routes: Vec<(String, Box<Handler>)>,
}

impl WebServer {
    pub fn bind<T: ToSocketAddrs>(addr: T) -> Self {
        Self {
            listener: match TcpListener::bind(addr) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[Error] Failed to bind to port: {e}");
                    std::process::exit(1);
                }
            },

            routes: Vec::new(),
        }
    }

    pub fn route<F: Fn(QueryParis) -> Response + 'static + Send + Sync>(mut self, path: &str, func: F) -> Self {
        self.routes.push((path.to_string(), Box::new(func)));
        self
    }

    pub fn start(self) -> ! {
        let mut incoming = self.listener.incoming();
        let mut pool = Pool::new(crate::NUM_THREADS);

        loop {
            let stream = incoming.next().unwrap();
            let stream = stream.expect("Error handling TCP stream.");

            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(10)))
                .expect("[Error] Couldn't set read timeout on socket");

            pool.scoped(|scope| {
                scope.execute(|| self.handle_connection(stream));
            });
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        println!("New connection: {}", stream.local_addr().unwrap());

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let buffer_str = std::str::from_utf8(&buffer).unwrap();
        let path = Self::get_requested_path(buffer_str);

        // Base url and scheme is not used here, that's why arbitrary url and scheme used
        let url = Url::parse(&format!("http://localhost{path}")).unwrap();

        for (path, function) in self.routes.iter() {
            if path == url.path() {
                let query_pairs = url.query_pairs();
                let response = function(
                    query_pairs
                        .into_iter()
                        .map(|(key, value)| (key.into_owned(), value.into_owned()))
                        .collect(),
                );

                write!(stream, "{}", response).expect("Failed to write to stream");
                stream.flush().expect("Failed to flush stream");
            }
        }
    }

    fn get_requested_path(request: &str) -> &str {
        let request_line = request.lines().next().unwrap_or("");
        let path = request_line.split_whitespace().nth(1).unwrap_or("/");
        path
    }
}
