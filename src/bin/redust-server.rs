
use clap::Parser;
use redust::server::RedustServer;
use std::{
    error::Error,
    fs::OpenOptions,
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(version, about = "Redust TCP Server")]
struct Args {
    /// Host address to bind to
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(short = 'p', long, default_value_t = 6380)]
    port: u16,

    /// Run the server in the background
    #[arg(short = 'd', long)]
    detach: bool,

    /// Internal flag used by the detached child
    #[arg(long, hide = true)]
    foreground_child: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Redust server error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.detach && !args.foreground_child {
        return detach_server(&args);
    }

    let server = RedustServer::new(args.host, args.port)?;
    server.start()?;

    Ok(())
}

fn detach_server(args: &Args) -> Result<(), Box<dyn Error>> {
    let executable = std::env::current_exe()?;

    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open("redust-server.log")?;

    let stderr = log.try_clone()?;

    let child = Command::new("setsid")
        .arg(executable)
        .arg("--host")
        .arg(&args.host)
        .arg("--port")
        .arg(args.port.to_string())
        .arg("--foreground-child")
        .stdin(Stdio::null())
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(stderr))
        .spawn()?;

    println!(
        "Redust server process launched (PID: {}). Logs: redust-server.log",
        child.id()
    );

    Ok(())
}
