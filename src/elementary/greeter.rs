use std::io;

use crate::utils::exercises::Exercise;

pub struct Greeter {
    reader: io::Stdin,
    username: String,
    message: String,
}

impl Greeter {
    fn greet(&mut self) -> Result<(), String> {
        println!("Please enter your name!");

        match self.reader.read_line(&mut self.username) {
            Ok(_) => {
                self.username = self.username.trim().to_string();
                self.message = format!("Hello, {}! Nice to meet you", self.username);
                println!("{}", self.message);
                Ok(())
            }
            Err(err) => Err(format!("Failed to read name: {}", err)),
        }
    }
}

impl Default for Greeter {
    fn default() -> Self {
        Greeter {
            reader: io::stdin(),
            username: String::new(),
            message: String::new(),
        }
    }
}

impl Exercise for Greeter {
    fn run(&mut self) -> Result<(), String> {
        self.greet()?;
        Ok(())
    }
}
