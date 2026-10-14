use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::client::RedustClient;
use crate::config::read_address;

pub enum Commands {
    SET,
    GET,
    DELETE,
    None,
}

#[derive(Default)]
pub struct InteractiveMode;

impl InteractiveMode {
    pub fn new() -> Self {
        Self
    }

    /// Runs the interactive prompt loop until the user exits.
    ///
    /// # Errors
    ///
    /// Returns an error if the line editor cannot be initialized.
    pub fn read_cmd(&self) -> Result<()> {
        let address = read_address()?;
        let mut rl = DefaultEditor::new()?;
        let mut client = RedustClient::connect(&address)?;

        loop {
            let readline = rl.readline("redust> ");
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
                        break;
                    }
                    let _ = rl.add_history_entry(line);
                    println!("{}", client.execute(line)?);
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Ctrl-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Ctrl-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {err:#?}");
                    break;
                }
            }
        }

        Ok(())
    }
}
