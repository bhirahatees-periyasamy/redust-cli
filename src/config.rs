use std::{fs, io, path::PathBuf};

fn address_file() -> io::Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME directory not found"))?;

    Ok(PathBuf::from(home).join(".redust-server").join("address"))
}

/// Writes the server address to `~/.redust-server/address` and returns the path.
///
/// # Errors
///
/// Returns an error if the `HOME` environment variable is not set, or if the
/// directory cannot be created or the file cannot be written.
pub fn write_address(addr: &str) -> io::Result<PathBuf> {
    let path = address_file()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&path, addr)?;

    Ok(path)
}

/// Reads the server address from `~/.redust-server/address`.
///
/// # Errors
///
/// Returns an error if the `HOME` environment variable is not set, or if the
/// address file does not exist (the server has not been started) or cannot be
/// read.
pub fn read_address() -> io::Result<String> {
    let path = address_file()?;

    match fs::read_to_string(&path) {
        Ok(address) => Ok(address.trim().to_string()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            eprintln!("Failed to find the Redust server address file.");
            eprintln!("Start the server before running the CLI.");

            Err(err)
        }
        Err(err) => Err(err),
    }
}
