use std::io::{self, Read, Write};
use std::net::TcpStream;

pub struct RedustClient {
    stream: TcpStream,
}

impl RedustClient {
    /// Connects to a Redust server at the given address.
    ///
    /// # Errors
    ///
    /// Returns an error if the address cannot be resolved or the TCP
    /// connection to the server cannot be established.
    pub fn connect(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(Self { stream })
    }

    /// Sends a command to the server and returns its trimmed response.
    ///
    /// # Errors
    ///
    /// Returns an error if writing the command to the connection or reading
    /// the server's response fails.
    pub fn execute(&mut self, command: &str) -> io::Result<String> {
        // Send the command
        self.stream.write_all(command.as_bytes())?;
        self.stream.write_all(b"\n")?;

        // Read the response
        let mut buffer = [0_u8; 1024];
        let bytes_read = self.stream.read(&mut buffer)?;

        Ok(String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_string())
    }
}
