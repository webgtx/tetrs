mod backend;
mod frontend;

use frontend::terminal;

fn main() -> Result<(), std::io::Error> {
    let msg = terminal::TetrsTerminal::new(std::io::stdout()).run()?;
    println!("{msg}");
    Ok(())
}
