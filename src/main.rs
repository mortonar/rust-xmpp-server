use std::env;
use server::XmppServer;

mod server;
mod client_session;

fn main() {
    // parse port from command line
    let args: Vec<_> = env::args().collect();
    let default_port = 5222;
    let port = if (args.len() != 2) {
        println!("Port not given, using default of {}", default_port);
        default_port
    } else {
        match args[1].parse::<i32>() {
            Ok(a) => a,
            _ => {
                println!("Invalid port given({}), using default of {}", args[1], default_port);
                default_port
            }
        }
    };

    println!("Staring XMPP Server on port {}...", port);
    XmppServer::new("127.0.0.1".to_string(), port).start();
}
