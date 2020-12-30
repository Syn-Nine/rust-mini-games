mod game;
use std::io::Error;

fn main() -> Result<(), Error> {
    println!("Knights and Barbarians, by Syn9");

    let mut app = game::new();
    app.run()?;

    Ok(())
}
