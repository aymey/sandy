mod sandy;

use std::error::Error;

use sandy::Sandy;

fn main() -> Result<(), Box<dyn Error>> {
    let display_name = std::env::var("DISPLAY")?;

    let sandy = Sandy::new(&display_name)?;

    sandy.init()?;
    sandy.run();

    Ok(())
}
