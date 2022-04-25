// use aether_lib::peer::Aether;
use std::io::Read;
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::thread;
use std::usize;

fn recv_data(buf: &mut [u8], stream: &mut UnixStream) -> usize {
    let n = stream.read(buf).unwrap();
    return n;
}
fn parse_data(data: &mut String, buf: &[u8]) -> usize {
    let mut n: String = String::new();
    let mut nn: usize = 0;

    println!("{}", String::from_utf8_lossy(buf));

    for i in 0..buf.len() {
        if buf[i] >= 48 && buf[i] <= 57 {
            n.push(buf[i] as char);
        } else {
            nn = n.parse::<usize>().unwrap();
            break;
        }
    }
    let data_ = String::from_utf8_lossy(&buf[1..(nn + 1)]).to_string();
    data.push_str(&data_);
    return nn;
}
fn parse_command(buf: &mut [u8; 1024]) {
    let n = buf.len();
    match buf[0] {
        // 1 for connect
        49 => {
            println!("{}", "connect");
            let mut username = String::new();
            let n = parse_data(&mut username, &buf[1..n]);
            println!("{}", username);

            // Put aether connect here
        }
        // 2 for send
        50 => {
            println!("{}", "send");
            let mut username = String::new();
            let n1 = parse_data(&mut username, &buf[1..n]);
            println!("{}", username);
            let mut message = String::new();
            let n2 = parse_data(&mut message, &buf[n1 + 2..n]);
            println!("{}", message);

            // Put aether send here
        }
        // 3 for recv
        51 => {
            println!("{}", "recv");
            let mut username = String::new();
            let n = parse_data(&mut username, &buf[1..n]);

            // Put aether recv here
        }
        _ => println!("{}", "unknown"),
    }
}
fn accept_command(client_address: &mut UnixStream) {
    let mut buf = [0; 1024];
    let n: usize = recv_data(&mut buf, client_address);
    if n == 0 {
        // nothing received from client
        return;
    }
    println!("accepting command");
    parse_command(&mut buf);
}
fn handle_client(client: UnixStream) {
    println!("{:?}", client);
    let mut client_address = client;

    loop {
        accept_command(&mut client_address);
    }
}

fn main() -> std::io::Result<()> {
    println!("Aether Service started ... ");
    let path_socket = "./uds_socket3";
    let listener = UnixListener::bind(path_socket)?;

    // accept connections and process them, spawning a new thread for each one
    for client in listener.incoming() {
        match client {
            Ok(client) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(client));
            }
            Err(_err) => {
                /* connection failed */
                break;
            }
        }
    }
    Ok(())
}
