use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(Default)]
pub struct InteractiveMode;

impl InteractiveMode {
    pub fn new() -> Self {
        Self
    }

    pub fn read_cmd(&self) -> Result<()> {
        let mut rl = DefaultEditor::new()?;

        loop {
            let readline = rl.readline("redust> ");
            match readline {
                Ok(line) => {
                    println!("{line}");
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
