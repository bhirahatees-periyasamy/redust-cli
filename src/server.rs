use crate::codec::{Codec, CodecOperations};
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
