use std::net::TcpListener;

pub struct Connection {
    listener: TcpListener,
}

impl Connection {
    pub fn new(listener: TcpListener) -> Self {
        Self { listener }
    }
}
