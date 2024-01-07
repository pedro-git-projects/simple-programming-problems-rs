pub trait Exercise {
    fn run(&mut self) -> Result<(), String>;
}
