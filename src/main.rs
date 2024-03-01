// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
};

use bytes::{Bytes, BytesMut};
use http_server_starter_rust::http::{handle_connection, HTTPRequest};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
