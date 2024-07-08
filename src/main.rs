mod frontend;
mod backend;

fn main() -> Result<(), std::io::Error> {
    frontend::console::run(&mut std::io::stdout())
}