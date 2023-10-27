pub mod response;
pub mod webserver;

pub type QueryParis = Vec<(String, String)>;

// TODO: use environment variable
pub const NUM_THREADS: u32 = 4;

#[cfg(test)]
mod test {
    use crate::{
        response::{Response, ResponseBuilder},
        webserver::WebServer,
        QueryParis,
    };

    #[test]
    fn main() {
        let server = WebServer::bind("127.0.0.1:8888")
            .route("/", root)
            .route("/foo", foo);

        server.start();
    }

    fn root(_: Vec<(String, String)>) -> Response {
        ResponseBuilder::new()
            .status(200)
            .header("Content-Type", "text/html")
            .body("<h1>Hello</h1>")
            .build()
    }

    fn foo(_: QueryParis) -> Response {
        ResponseBuilder::new()
            .status(200)
            .header("Content-Type", "text/html")
            .body("<h1>Bar</h1>")
            .build()
    }
}
