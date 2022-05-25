mod handler;
mod socket_server;

use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::thread;
use std::usize;

use aether_lib::peer::Aether;
use log::error;
use log::info;
use log::trace;
use simple_logger::SimpleLogger;
use socket_server::socket_server::init_socket_server;

use crate::handler::handle_command;

pub const RECV_BUFFER_SIZE: usize = 2048;

fn handle_client(mut client: (impl Read + Write)) {
    info!("Client connected!");
    loop {
        let mut buf = [0; RECV_BUFFER_SIZE];
        let n: usize = match client.read(&mut buf) {
            Ok(n) => n,
            Err(error) => match error.kind() {
                ErrorKind::ConnectionReset => {
                    println!("{:?}", error);
                    info!("Connection Closed");
                    break;
                }
                _ => {
                    error!("Unexpected error handling client: {}", error);
                    break;
                }
            },
        };

        if n > 0 {
            trace!("Received command");

            // handle command
            handle_command(&buf[0..n]);

            // respond to client
            match client.write(&buf) {
                Ok(_) => (),
                Err(error) => match error.kind() {
                    ErrorKind::ConnectionReset => {
                        info!("Connection Closed");
                        break;
                    }
                    _ => {
                        error!("Unexpected error handling client: {}", error);
                    }
                },
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .env()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("Unable to initialize logger");

    info!("Aether Service started...");

    let tracker_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(149, 129, 129, 226)), 8982);
    let aether = Aether::new(tracker_addr);

    aether.start();

    let listener = init_socket_server().unwrap();

    // accept connections and process them, spawning a new thread for each one
    for client in listener {
        thread::spawn(|| handle_client(client));
    }

    Ok(())
}
