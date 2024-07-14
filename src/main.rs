mod backend;
mod frontend;

use frontend::terminal;

fn main() -> Result<(), std::io::Error> {
    let msg = terminal::run(&mut std::io::stdout())?;
    println!("{msg}");
    Ok(())
}
