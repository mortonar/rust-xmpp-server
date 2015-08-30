use std::thread;
use std::net::TcpStream;

pub struct XmppClientSession {
    tcp_stream: TcpStream
}

impl XmppClientSession {
    pub fn new(tcp_stream: TcpStream) -> XmppClientSession {
        XmppClientSession {
            tcp_stream: tcp_stream
        }
    }
    pub fn start(&self) {
        thread::spawn(move|| {
            println!("Starting new session...");
        });
    }
}
