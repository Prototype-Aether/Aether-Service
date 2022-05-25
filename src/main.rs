mod socket_server;

use std::io::Read;
use std::io::Write;
use std::thread;
use std::usize;

use socket_server::socket_server::init_socket_server;

pub const RECV_BUFFER_SIZE: usize = 2048;

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

fn parse_command(buf: &mut [u8; RECV_BUFFER_SIZE]) {
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

fn handle_client(mut client: (impl Read + Write)) {
    loop {
        let mut buf = [0; RECV_BUFFER_SIZE];
        let n: usize = client.read(&mut buf).unwrap();
        if n > 0 {
            println!("accepting command");
            parse_command(&mut buf);
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("Aether Service started ... ");
    let listener = init_socket_server().unwrap();

    // accept connections and process them, spawning a new thread for each one
    for client in listener {
        thread::spawn(|| handle_client(client));
    }
    Ok(())
}
