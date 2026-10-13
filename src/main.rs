use redust::server::RedustServer;

fn main() -> std::io::Result<()> {
    let server = RedustServer::new(String::from("127.0.0.1"), 1234)?;
    server.start()?;
    Ok(())
}
