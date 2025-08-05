pub mod logger;
pub mod router;
pub mod template;
pub mod request;
pub mod response;

use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::sync::Arc;
use router::Router;
use request::Request;
use response::Response;

#[macro_export]
macro_rules! listen {
    ($host:literal, $port:expr) => {
        $crate::listen($host, $port, false)
    };
    ($host:literal, $port:expr, $log:expr) => {
        $crate::listen($host, $port, $log)
    };
}

pub fn listen(address: &str, port: u16, log: bool) {
    let listener = TcpListener::bind(format!("{address}:{port}")).expect("Failed to bind");
    let router = Arc::new(Router::new());

    info!("Listening on http://{}:{}", address, port);

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        if log {
            info!("Connection from {}", stream.peer_addr().expect("Failed to get request address"));
        }
        let router_clone = Arc::clone(&router);
        handle_connection(stream, router_clone);
    }
}

pub fn listen_with_router(address: &str, port: u16, log: bool, router: Router) {
    let listener = TcpListener::bind(format!("{address}:{port}")).expect("Failed to bind");
    let router = Arc::new(router);

    info!("Listening on http://{}:{}", address, port);

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to accept connection");
        if log {
            info!("Connection from {}", stream.peer_addr().expect("Failed to get request address"));
        }
        let router_clone = Arc::clone(&router);
        handle_connection(stream, router_clone);
    }
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();

    let request_line = lines.next().unwrap().unwrap();
    let mut headers = std::collections::HashMap::new();
    let body = String::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    let parts: Vec<&str> = request_line.split(' ').collect();
    if parts.len() >= 2 {
        let method = parts[0];
        let path = parts[1];

        let request = Request::new(method, path, headers, body);
        let response = router.handle_request(&request);

        let response_string = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            response.status_code,
            response.status_text,
            response.headers_string(),
            response.body
        );

        stream.write_all(response_string.as_bytes()).unwrap();
    } else {
        let error_response = Response::new(400, "Bad Request", "Invalid request format");
        let response_string = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            error_response.status_code,
            error_response.status_text,
            error_response.headers_string(),
            error_response.body
        );
        stream.write_all(response_string.as_bytes()).unwrap();
    }
}