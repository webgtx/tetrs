mod game_screen_renderers;
mod input_handler;
pub mod terminal_tetrs;

fn main() -> Result<(), std::io::Error> {
    println!(
        "{}",
        terminal_tetrs::TerminalTetrs::new(std::io::stdout()).run()?
    );
    Ok(())
}
