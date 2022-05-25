pub const UNIX_SOCKET_PATH: &str = "/tmp/aether";
pub const TCP_SOCKET_PATH: &str = "0.0.0.0:32898";

#[cfg(target_family = "unix")]
pub mod socket_server {
    use super::UNIX_SOCKET_PATH;
    use std::{
        io::{Error, Read, Write},
        os::unix::net::{UnixListener, UnixStream},
    };

    pub struct SocketServer {
        listener: UnixListener,
        path: &'static str,
    }

    pub fn init_socket_server() -> Result<impl Iterator<Item = (impl Read + Write)>, Error> {
        Ok(SocketServer {
            listener: UnixListener::bind(UNIX_SOCKET_PATH)?,
            path: UNIX_SOCKET_PATH,
        })
    }

    impl Iterator for SocketServer {
        type Item = UnixStream;
        fn next(&mut self) -> Option<Self::Item> {
            match self.listener.accept() {
                Ok(stream) => Some(stream.0),
                Err(_) => None,
            }
        }
    }
}

#[cfg(not(target_family = "unix"))]
pub mod socket_server {
    use super::TCP_SOCKET_PATH;
    use std::{
        io::{Error, Read, Write},
        os::unix::net::{TcpListener, TcpStream},
    };

    pub struct SocketServer {
        listener: TcpListener,
        path: &'static str,
    }

    pub fn init_socket_server() -> Result<impl Iterator<Item = (impl Read + Write)>, Error> {
        Ok(SocketServer {
            listener: TcpListener::bind(UNIX_SOCKET_PATH)?,
            path: UNIX_SOCKET_PATH,
        })
    }

    impl Iterator for SocketServer {
        type Item = TcpStream;
        fn next(&mut self) -> Option<Self::Item> {
            match self.listener.accept() {
                Ok(stream) => Some(stream.0),
                Err(_) => None,
            }
        }
    }
}
