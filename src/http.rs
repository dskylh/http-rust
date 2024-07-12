use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    str::from_utf8,
};

use bytes::Bytes;

use crate::utils::split_path;

pub enum ContentType {
    HTML,
    JSON,
    XML,
    PLAIN,
}

impl ContentType {
    pub fn to_string(&self) -> String {
        match self {
            ContentType::HTML => "text/html".to_string(),
            ContentType::JSON => "application/json".to_string(),
            ContentType::XML => "application/xml".to_string(),
            ContentType::PLAIN => "text/plain".to_string(),
        }
    }
}

pub enum StatusCode {
    OK,
    NotFound,
}

impl StatusCode {
    pub fn to_str(&self) -> &str {
        match self {
            StatusCode::OK => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        }
    }
}

pub struct HTTPResponse {
    pub status_code: StatusCode,
    pub content_type: ContentType,
    pub body: Bytes,
}

impl HTTPResponse {
    pub fn to_bytes(&self) -> Bytes {
        let mut response = String::from(format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.status_code.to_str(),
            self.content_type.to_string(),
            self.body.len()
        ));
        response.push_str(from_utf8(&self.body).unwrap());
        Bytes::from(response)
    }
}

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
    pub fn to_string(&self) -> String {
        match self {
            HTTPMethod::GET => "GET".to_string(),
            HTTPMethod::POST => "POST".to_string(),
            HTTPMethod::PUT => "PUT".to_string(),
            HTTPMethod::DELETE => "DELETE".to_string(),
            HTTPMethod::PATCH => "PATCH".to_string(),
            HTTPMethod::OPTIONS => "OPTIONS".to_string(),
            HTTPMethod::HEAD => "HEAD".to_string(),
            HTTPMethod::CONNECT => "CONNECT".to_string(),
            HTTPMethod::TRACE => "TRACE".to_string(),
        }
    }
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
    pub paths: Vec<String>,
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
        let paths = split_path(path);
        let headers = http_request[1..]
            .iter()
            .map(|line| {
                let header = line.split(": ").collect::<Vec<&str>>();
                (header[0].to_string(), header[1].to_string())
            })
            .collect::<Vec<(String, String)>>();
        HTTPRequest {
            method: HTTPMethod::from_str(method).unwrap(),
            paths,
            headers,
            body: Bytes::new(),
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        let mut request = String::from(format!(
            "{} {}\r\n",
            self.method.to_string(),
            self.paths.join("/")
        ));
        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        request.push_str("\r\n");
        Bytes::from(request)
    }

    pub fn get_headers(&self, key: &str) -> Option<String> {
        for (k, v) in &self.headers {
            if k == key {
                return Some(v.clone());
            }
        }
        None
    }
}

pub fn handle_connection(stream: &mut TcpStream) {
    let request = HTTPRequest::new(stream);
    match request.method {
        HTTPMethod::GET => {
            // print paths
            println!("{:?}", request.paths);
            if request.paths[1].trim() == "echo" {
                let response = HTTPResponse {
                    status_code: StatusCode::OK,
                    content_type: ContentType::PLAIN,
                    body: Bytes::from(request.paths[2].clone()),
                };
                let _ = stream.write_all(&response.to_bytes());
            } else if request.paths[1] == "" {
                // retrun empty OK HTTP Response
                let response = HTTPResponse {
                    status_code: StatusCode::OK,
                    content_type: ContentType::PLAIN,
                    body: Bytes::from(""),
                };

                let _ = stream.write_all(&response.to_bytes());
            } else if request.paths[1] == "user-agent" {
                let user_agent = request.get_headers("User-Agent").unwrap();
                let response = HTTPResponse {
                    status_code: StatusCode::OK,
                    content_type: ContentType::PLAIN,
                    body: Bytes::from(user_agent),
                };
                let _ = stream.write_all(&response.to_bytes());
            } else {
                // return 404 Not Found HTTP Response
                let response = HTTPResponse {
                    status_code: StatusCode::NotFound,
                    content_type: ContentType::PLAIN,
                    body: Bytes::from(""),
                };

                let _ = stream.write_all(&response.to_bytes());
            }
        }
        _ => {
            let response = Bytes::from("HTTP/1.1 405 Method Not Allowed\r\n\r\n");
            let _ = stream.write_all(&response);
        }
    }
}
