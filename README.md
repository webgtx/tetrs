<div align="center"><img width="440" src="https://repository-images.githubusercontent.com/816034047/9eba09ef-d6da-4b4c-9884-630e7f87e102" /></div>


# tetrs
Tetromino game engine and a playable interface for the terminal.

# Usage
## How to run `tetrs_terminal`
Pre-compiled:
- Download a release for your platform if available and run the application.

Compiling yourself:
- Have the [Rust](https://www.rust-lang.org/) compiler (and Cargo) installed.
- [Download](<https://github.com/Strophox/tetrs/archive/refs/heads/main.zip>) (or `git clone`) this repo.
- Navigate to `tetrs/` (or `tetrs_terminal/`) and compile with `cargo run`.
- (Relevant keys [`Esc`,`Enter`,`←`,`→`,`↑`,`↓`,`A`,`D`] also shown inside the application)

Additional notes:
- Set the framerate of the game by running `./tetrs_terminal --fps=60` (or `cargo run -- --fps=60`) (default is 30fps).
- Use a terminal like [kitty](<https://sw.kovidgoyal.net/kitty/>) for smoothest gameplay and visual experience. *Explanation:* Terminals do not usually send "key released" signals, which is a problem for mechanics such as "press left to move left repeatedly **until key is released**". We rely on [Crossterm](https://docs.rs/crossterm/latest/crossterm/event/struct.PushKeyboardEnhancementFlags.html) to automatically detect terminals where this is possible. Otherwise DAS/ARR will be determined by Keyboard/OS/terminal settings and not by the game.)

## How to use `tetrs_engine`
- TODO: [Git dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).

# Features

## Engine
TODO: `all` the features here.

## Frontend
TODO: `all` the tui pain here.


# Idea
