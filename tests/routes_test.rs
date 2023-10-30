use std::sync::{Arc, Mutex};
use std::thread::spawn;

use crane_webserver::Query;
use crane_webserver::*;

use reqwest::blocking::get;
use reqwest::StatusCode;

#[test]
fn default_route() {
    let server = Arc::new(Mutex::new(
        WebServer::bind("127.0.0.1:0", |path, _query| {
            match path.as_str() {
                "/" => default_route_fn(),
                _ => ResponseBuilder::new().build(),
            }
        })
            .unwrap()
    ));

    let addr = server.lock().unwrap().get_addr().unwrap();

    let s = Arc::clone(&server);
    spawn(move || {
        s.lock().unwrap().start();
    });

    let response = get(format!("http://{}", addr.to_string())).unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().unwrap(), "Hello, World!");
}

fn default_route_fn() -> Response {
    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .header("Content-Type", "text/plain")
        .body("Hello, World!")
        .build()
}

#[test]
fn routes_and_query() {
    let server = Arc::new(Mutex::new(
        WebServer::bind("127.0.0.1:0", |path, query| {
            match path.as_str() {
                "/get/data" => routes_and_query_fn(query),
                _ => ResponseBuilder::new().build()
            }
        })
            .unwrap()
    ));

    let addr = server.lock().unwrap().get_addr().unwrap();

    let s = Arc::clone(&server);
    spawn(move || {
        s.lock().unwrap().start();
    });

    let response = get(format!("http://{}/get/data?a=b", addr.to_string())).unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text().unwrap(), r#"a=["b"]"#);
}

fn routes_and_query_fn(q: Query) -> Response {
    let res_body = q
        .into_iter()
        .map(|(k, v)| format!("{k}={v:?}"))
        .collect::<Vec<_>>()
        .join("\n");

    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .header("Content-Type", "text/plain")
        .body(&res_body)
        .build()
}
