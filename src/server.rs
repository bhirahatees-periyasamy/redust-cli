use crate::codec::{Codec, CodecOperations};
use crate::config::write_address;
use crate::service::{RedustService, ServiceOperations};
use std::io::{self, Read, Write};
use std::net::TcpListener;

#[expect(unused)]
pub struct RedustServer {
    host: String,
    port: u16,
    pub listener: TcpListener,
}

impl RedustServer {
    /// Binds a TCP listener on the given host and port.
    ///
    /// # Errors
    /// Returns an error if the address cannot be bound.
    pub fn new(host: String, port: u16) -> io::Result<Self> {
        let address = format!("{host}:{port}");
        let listener = TcpListener::bind(address)?;
        println!("Starting Redust Server on {host}:{port}");

        let actual_addr = listener.local_addr()?;
        let _ = write_address(&actual_addr.to_string())?;

        Ok(Self {
            host,
            port,
            listener,
        })
    }
    /// Accepts incoming connections and handles one command per connection.
    ///
    /// # Errors
    /// Returns an error if accepting a connection, or reading from or writing
    /// to a stream, fails.
    pub fn start(&self) -> io::Result<()> {
        let mut service = RedustService::new();
        let codec = Codec::new();

        for incoming in self.listener.incoming() {
            let mut stream = incoming?;
            let mut buffer = [0_u8; 1024];

            let bytes_read = stream.read(&mut buffer)?;

            if bytes_read == 0 {
                continue;
            }

            let decoded = codec.decode(&buffer[..bytes_read]);

            let response = service.execute(decoded);

            stream.write_all(response.as_bytes())?;
            stream.write_all(b"\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, TcpStream};
    use std::thread;

    /// Binds on an ephemeral port and runs the server on a background thread.
    fn spawn_server() -> SocketAddr {
        let server = RedustServer::new(String::from("127.0.0.1"), 0).unwrap();
        let addr = server.listener.local_addr().unwrap();
        thread::spawn(move || {
            let _ = server.start();
        });
        addr
    }

    /// Sends one command on a fresh connection and returns the reply.
    fn send(addr: SocketAddr, command: &str) -> String {
        let mut stream = TcpStream::connect(addr).unwrap();
        stream.write_all(command.as_bytes()).unwrap();
        let mut reply = String::new();
        stream.read_to_string(&mut reply).unwrap();
        reply
    }

    #[test]
    fn new_binds_to_requested_address() {
        let server = RedustServer::new(String::from("127.0.0.1"), 0).unwrap();
        let addr = server.listener.local_addr().unwrap();
        assert!(addr.ip().is_loopback());
        assert_ne!(addr.port(), 0);
    }

    #[test]
    fn new_fails_when_port_is_taken() {
        let first = RedustServer::new(String::from("127.0.0.1"), 0).unwrap();
        let port = first.listener.local_addr().unwrap().port();
        assert!(RedustServer::new(String::from("127.0.0.1"), port).is_err());
    }

    #[test]
    fn new_fails_on_invalid_host() {
        assert!(RedustServer::new(String::from("not a host"), 0).is_err());
    }

    #[test]
    fn set_replies_ok_with_trailing_newline() {
        let addr = spawn_server();
        assert_eq!(send(addr, "SET k v"), "OK\n");
    }

    #[test]
    fn get_returns_value_stored_by_earlier_connection() {
        let addr = spawn_server();
        send(addr, "SET name redust");
        assert_eq!(send(addr, "GET name"), "redust\n");
    }

    #[test]
    fn get_unknown_key_replies_none() {
        let addr = spawn_server();
        assert_eq!(send(addr, "GET ghost"), "None\n");
    }

    #[test]
    fn delete_removes_key() {
        let addr = spawn_server();
        send(addr, "SET k v");
        assert_eq!(send(addr, "DELETE k"), "OK\n");
        assert_eq!(send(addr, "GET k"), "None\n");
    }

    #[test]
    fn invalid_command_replies_with_error() {
        let addr = spawn_server();
        assert_eq!(send(addr, "BOGUS k"), "Invalid command\n");
    }

    #[test]
    fn empty_connection_is_skipped_and_server_keeps_serving() {
        let addr = spawn_server();
        // Connect and close without sending anything (read returns 0 bytes).
        drop(TcpStream::connect(addr).unwrap());
        assert_eq!(send(addr, "SET k v"), "OK\n");
    }

    #[test]
    fn state_is_isolated_per_server_instance() {
        let a = spawn_server();
        let b = spawn_server();
        send(a, "SET k v");
        assert_eq!(send(b, "GET k"), "None\n");
    }
}
