use crate::game_logic::{Game, Gamemode};

use std::{
    num::NonZeroU64,
    time::{Duration, Instant},
    io::Write,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style,
    terminal,
    ExecutableCommand, QueueableCommand,
};

const GAME_DRAW_RATE: u64 = 3; // 60fps

struct Settings {
    //TODO information stored throughout application?
}

enum Screen {
    Title, //TODO Store selected gamemode or smth for the selection screen for convenience
    Gaming(Game),
    Options,
}

enum ScreenChange {
    Exit,
    Keep,
    Enter(Screen),
}


fn enter_title_screen(w: &mut dyn Write) -> std::io::Result<ScreenChange> {
    return Ok(ScreenChange::Enter(Screen::Gaming(Game::new(Gamemode::endless()))));
    /*TODO make title screen
    while event::poll(Duration::from_secs(0))? {
        match event::read()? {
            // Abort
            Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: _}) => {
                break 'update_loop
            }
            // Handle common key inputs
            Event::Key(KeyEvent) => {
                // TODO handle key inputs!
            }
            Event::Resize(cols, rows) => {
                // TODO handle resize
            }
            // Console lost focus: Pause, re-enter update loop
            Event::FocusLost => {
                // TODO actively UNfocus application (requires flag)?
                if let Screen::Gaming(_) = screen {
                    active_screens.push(Screen::Options);
                    continue 'update_loop
                }
            }
            // Console gained focus: Do nothing, just let player continue
            Event::FocusGained => { }
            // NOTE We do not handle mouse events (yet?)
            Event::Mouse(MouseEvent) => { }
            // Ignore pasted text
            Event::Paste(String) => { }
        }
    }*/
}

fn enter_options(w: &mut dyn Write, settings: &mut Settings) -> std::io::Result<ScreenChange> {
    //TODO implement options overlay
    return Ok(ScreenChange::Exit);
}

fn enter_game(w: &mut dyn Write, settings: &Settings, game: &mut Game) -> std::io::Result<ScreenChange> {
    //TODO implement game loop!!
    todo!("Ayo") 
}

pub fn run(w: &mut impl Write) -> std::io::Result<()> {
    // Initialize console
    w.execute(terminal::EnterAlternateScreen)?;
    //TODO use kitty someday w.execute(event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
    terminal::enable_raw_mode()?;
    // Prepare and run main update loop
    let mut settings = Settings {}; // Application settings
    let mut active_screens = vec![Screen::Title]; // Active screens
    loop {
        // Retrieve active screen, stop application if all exited
        let Some(screen) = active_screens.last_mut() else {
            break;
        };
        // Enter screen until it returns what to do next
        let update = match screen {
            Screen::Title => enter_title_screen(w),
            Screen::Options => enter_options(w, &mut settings),
            Screen::Gaming(game) => enter_game(w, &settings, game),
        }?;
        // Change screen session depending on what response screen gave
        match update {
            ScreenChange::Exit => { active_screens.pop(); },
            ScreenChange::Keep => { }
            ScreenChange::Enter(new_screen) => { active_screens.push(new_screen); }
        }
    }
    // Deinitialize console
    w.execute(style::ResetColor)?;
    w.execute(cursor::Show)?;
    w.execute(terminal::LeaveAlternateScreen)?;
    //TODO use kitty someday w.execute(event::PopKeyboardEnhancementFlags)?;
    terminal::disable_raw_mode()?;
    Ok(())
}