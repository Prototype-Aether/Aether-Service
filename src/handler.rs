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

pub fn handle_command(buf: &[u8]) {
    let size = buf.len();
    match buf[0] {
        // 1 for connect
        49 => {
            println!("{}", "connect");
            let mut username = String::new();
            let _n = parse_data(&mut username, &buf[1..size]);
            println!("{}", username);

            // Put aether connect here
        }
        // 2 for send
        50 => {
            println!("{}", "send");
            let mut username = String::new();
            let n1 = parse_data(&mut username, &buf[1..size]);
            println!("{}", username);
            let mut message = String::new();
            let _n2 = parse_data(&mut message, &buf[n1 + 2..size]);
            println!("{}", message);

            // Put aether send here
        }
        // 3 for recv
        51 => {
            println!("{}", "recv");
            let mut username = String::new();
            let _n = parse_data(&mut username, &buf[1..size]);

            // Put aether recv here
        }
        _ => println!("{}", "unknown"),
    }
}
