# crane
[![Rust](https://github.com/Pranjal-Patel/crane/actions/workflows/rust.yml/badge.svg)](https://github.com/Pranjal-Patel/crane/actions/workflows/rust.yml)

A simple and fast webserver :)

## Getting started
In order to build a webserver, you need to add `crane-webserver` as a dependency in your rust project by:
```
cargo add crane-webserver
```


## Examples

Create an HTTP server that responds with a message.

```rust
use crane_webserver::*;

fn main() {
    let server = WebServer::bind("127.0.0.1:8888", |path, query| {
        match path.as_str() {
            "/" => default_route_fn(query),
            _ => ResponseBuilder::new().build(),
        }
    }).unwrap()

    server.start();
}

fn root() -> Response {
    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .header("Content-Type", "text/plain")
        .body("Hello, World!")
        .build()
}
```

```
$ curl localhost:8888/
Hello, World!
```
