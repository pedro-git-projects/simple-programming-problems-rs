use std::io;

use crate::utils::exercises::Exercise;

pub struct Sum {
    range: Vec<i32>,
    result: i32,
    reader: io::Stdin,
}

impl Default for Sum {
    fn default() -> Self {
        Sum {
            range: vec![],
            result: 0,
            reader: io::stdin(),
        }
    }
}

impl Sum {
    fn read_number(&mut self) -> Result<i32, String> {
        println!("Please enter a number!");

        let mut buffer = String::new();
        self.reader
            .read_line(&mut buffer)
            .expect("failed to read line!");
        let buffer = buffer.trim();

        match buffer.parse::<i32>() {
            Ok(number) => Ok(number),
            Err(_) => Err(format!("Invalid input: {}", buffer)),
        }
    }

    fn read_range(&mut self) -> Result<(), String> {
        println!("Please enter the upper bound of the range");
        let upper = self.read_number()?;

        println!("Please enter the lower bound of the range, it is inclusive");
        let lower = self.read_number()?;

        self.range = (lower..=upper).collect();
        Ok(())
    }

    fn get_sum(&mut self) -> Result<i32, String> {
        self.range.iter().for_each(|num| {
            self.result = self.result + num;
        });
        Ok(self.result)
    }
}

impl Exercise for Sum {
    fn run(&mut self) -> Result<(), String> {
        self.read_range()?;
        let sum = self.get_sum()?;
        println!("The sum was {}", sum);
        Ok(())
    }
}
