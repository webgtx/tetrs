mod frontend;
mod backend;

fn main() -> Result<(), std::io::Error> {
    let msg = frontend::console::run(&mut std::io::stdout())?;
    println!("{msg}");
    Ok(())
}