use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use bytes::Bytes;

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

impl HTTPMethod {
    pub fn from_str(method: &str) -> Option<HTTPMethod> {
        match method {
            "GET" => Some(HTTPMethod::GET),
            "POST" => Some(HTTPMethod::POST),
            "PUT" => Some(HTTPMethod::PUT),
            "DELETE" => Some(HTTPMethod::DELETE),
            "PATCH" => Some(HTTPMethod::PATCH),
            "OPTIONS" => Some(HTTPMethod::OPTIONS),
            "HEAD" => Some(HTTPMethod::HEAD),
            "CONNECT" => Some(HTTPMethod::CONNECT),
            "TRACE" => Some(HTTPMethod::TRACE),
            _ => None,
        }
    }
}

pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Bytes,
}

impl HTTPRequest {
    pub fn new(stream: &mut TcpStream) -> HTTPRequest {
        let reader = BufReader::new(stream);
        let http_request: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();
        let method = http_request[0].split_whitespace().collect::<Vec<&str>>()[0];
        let path = http_request[0].split_whitespace().collect::<Vec<&str>>()[1];
        let headers = http_request[1..]
            .iter()
            .map(|line| {
                let header = line.split(":").collect::<Vec<&str>>();
                (header[0].to_string(), header[1].to_string())
            })
            .collect::<Vec<(String, String)>>();
        HTTPRequest {
            method: HTTPMethod::from_str(method).unwrap(),
            path: path.to_string(),
            headers,
            body: Bytes::new(),
        }
    }
}

pub fn handle_connection(stream: &mut TcpStream) {
    let request = HTTPRequest::new(stream);
    match request.method {
        HTTPMethod::GET => {
            print!("{}", request.path);
            if request.path == "/" {
                let _response = Bytes::from("HTTP/1.1 200 OK\r\n\r\n");
            }
            let response = Bytes::from("HTTP/1.1 404 Not Found\r\n\r\n");
            let _ = stream.write_all(&response);
        }
        _ => {
            let response = Bytes::from("HTTP/1.1 405 Method Not Allowed\r\n\r\n");
            let _ = stream.write_all(&response);
        }
    }
}
