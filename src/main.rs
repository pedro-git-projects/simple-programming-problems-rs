mod elementary;
mod utils;
use std::io;

use elementary::{alicebob::AliceAndBob, greeter::Greeter, hello::Hello, sum::Sum};
use utils::runner::Runner;

fn main() -> Result<(), String> {
    let mut runner = Runner::default();
    runner.add_exercise(String::from("hello"), Box::new(Hello))?;
    runner.add_exercise(String::from("greeter"), Box::new(Greeter::default()))?;
    runner.add_exercise(
        String::from("alice and bob"),
        Box::new(AliceAndBob::default()),
    )?;
    runner.add_exercise(String::from("sum"), Box::new(Sum::default()))?;

    runner.print_messages();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");

    runner.run_exercise(input.trim())?;

    Ok(())
}
