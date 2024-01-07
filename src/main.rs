mod elementary;
mod utils;
use std::io;

use utils::runner::Runner;

use crate::elementary::hello::hello;

fn main() -> Result<(), String> {
    let mut runner = Runner::default();
    runner.add_exercise(String::from("hello"), Box::new(|| hello()))?;

    runner.print_messages();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");

    runner.run_exercise(input.trim().to_string())?;

    Ok(())
}
