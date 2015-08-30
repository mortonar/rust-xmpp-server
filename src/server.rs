use std::net::{TcpListener};
use client_session::XmppClientSession;

pub struct XmppServer {
    address: String,
    port: i32,
    sessions: Vec<XmppClientSession>
}

impl XmppServer {
    pub fn new(address: String, port: i32) -> XmppServer {
        XmppServer {
            address: address,
            port: port,
            sessions: Vec::new()
        }
    }
    pub fn start(&mut self) {
        // listen on given port and address
        let listener = match TcpListener::bind(&*(self.address.clone() + ":" + &self.port.to_string())) {
            Ok(l) => {
                println!("XMPP server started on address {} port {}", self.address, self.port);
                l
            },
            Err(e) => {
                panic!("XMPP server could not be started on address {} port {}: {}", self.address, self.port, e); 
            }
        };

        // spin-off a new client session for each incoming connection
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let client_session = XmppClientSession::new(stream);
                    client_session.start();
                    self.sessions.push(client_session);
                },
                Err(e) => {
                    println!("Error accepting client connection: {}", e);
                }
            }         
        }
    }
}
