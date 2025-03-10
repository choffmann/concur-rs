use std::io::{self, Write};



pub struct Logger {
    name: String,
}

impl Logger {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn println(&self, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "\x1b[93m[ {} ] {}\x1b[0m", self.name, value)?;

        Ok(())
    }
}

