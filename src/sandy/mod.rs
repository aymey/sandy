use thiserror::Error;

#[derive(Error, Debug)]
pub enum SandyError {
    #[error("display {0} not found")]
    DisplayNotFound(String),
}

pub struct Sandy {

}

impl Sandy {
    /// connect to X server and create a new [`Sandy`] Instance
    pub fn new(display_name: &str) -> Result<Self, SandyError> {
        Ok(Self {})
    }

    /// init events systems
    pub fn init(&self) -> Result<(), SandyError> {
        Ok(())
    }

    /// event listner
    pub fn run(&self) {
        println!("running");
    }
}
