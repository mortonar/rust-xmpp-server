use std::net::TcpStream;
use std::thread;

extern crate xml;

use client_session::xml::reader::{EventReader, XmlEvent};

use stanzas;

pub struct XmppClientSession {}

impl XmppClientSession {
    pub fn new() -> XmppClientSession {
        XmppClientSession {}
    }

    pub fn process_connection(session: &XmppClientSession, tcp_stream: TcpStream) {
        thread::spawn(move || {
            let parser = EventReader::new(tcp_stream);
            println!("Starting new session...");
            for e in parser {
                match e {
                    se @ Ok(XmlEvent::StartElement { .. }) => {
                        println!("{:?}", se);
                    }
                    Ok(XmlEvent::EndElement { name, .. }) => {
                        println!("{}", name);
                    }
                    _ => {}
                }
            }
        });
    }
}
