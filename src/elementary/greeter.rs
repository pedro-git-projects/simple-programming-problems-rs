use std::io;

use crate::utils::exercises::Exercise;

pub struct Greeter {
    reader: io::Stdin,
    username: String,
    message: String,
}

impl Greeter {
    fn new() -> Result<Greeter, String> {
        let mut g = Greeter {
            reader: io::stdin(),
            username: String::new(),
            message: String::new(),
        };

        let mut username = String::new();
        println!("Please enter your name!");

        match g.reader.read_line(&mut username) {
            Ok(_) => {
                let trimmed_username = username.trim().to_string();
                g.username = trimmed_username;
                g.message = format!("Hello, {}! Nice to meet you", g.username);
                Ok(g)
            }
            Err(err) => Err(format!("Failed to read name: {}", err)),
        }
    }

    fn greet(&self) {
        println!("{}", self.message);
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
        let greeter = Greeter::new()?;
        greeter.greet();
        *self = greeter;
        Ok(())
    }
}
