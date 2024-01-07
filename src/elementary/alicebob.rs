use std::io;

use crate::utils::exercises::Exercise;

pub struct AliceAndBob {
    valid_names: [&'static str; 2],
    reader: io::Stdin,
    actual_name: String,
}

impl Default for AliceAndBob {
    fn default() -> Self {
        AliceAndBob {
            valid_names: ["Alice", "Bob"],
            reader: io::stdin(),
            actual_name: String::new(),
        }
    }
}

impl AliceAndBob {
    pub fn greet(&mut self) -> Result<(), String> {
        println!("Please enter your name!");

        self.reader
            .read_line(&mut self.actual_name)
            .expect("Failed to read from stdin!");

        self.actual_name = self.actual_name.trim().to_string();
        let valid_iter = self.valid_names.into_iter();

        let matches = valid_iter
            .filter(|name| name.to_string() == self.actual_name)
            .collect::<Vec<&str>>();
        if matches.is_empty() {
            Err("I can only greet Alice and Bob".to_string())
        } else {
            println!("Hello, {}!", self.actual_name);
            Ok(())
        }
    }
}

impl Exercise for AliceAndBob {
    fn run(&mut self) -> Result<(), String> {
        self.greet()?;
        Ok(())
    }
}
