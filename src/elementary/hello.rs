use crate::utils;

pub struct Hello;

impl utils::exercises::Exercise for Hello {
    fn run(&mut self) -> Result<(), String> {
        println!("Hello,  rust!");
        Ok(())
    }
}
