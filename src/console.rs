use crate::game_logic::{Button, ButtonChange, ButtonMap, Game, Gamemode};

use std::{
    collections::HashMap, io::Write, sync::mpsc, time::{Duration, Instant}
};

//use device_query;
use crossterm::{
    cursor::{self, MoveLeft},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style,
    terminal,
    ExecutableCommand, QueueableCommand,
};
use device_query::{keymap as dq, DeviceEvents};

const GAME_DRAW_RATE: u64 = 3; // 60fps

struct Settings {
    keybinds: HashMap<dq::Keycode, Button>,
    //TODO information stored throughout application?
}

enum Screen {
    Title, //TODO Store selected gamemode or smth for the selection screen for convenience
    Gaming(Game),
    Settings, //TODO Get inspired by Noita's system on how to handle exiting to main menu (or not) or how to start a new game while within a game and pausing/opening settings
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

fn enter_settings(w: &mut dyn Write, settings: &mut Settings) -> std::io::Result<ScreenChange> {
    //TODO implement options overlay
    return Ok(ScreenChange::Exit);
}

fn enter_game(w: &mut dyn Write, settings: &Settings, game: &mut Game) -> std::io::Result<ScreenChange> {
    // Prepare channel from which to receive Button inputs or pause interrupt
    let (sx1, rx) = mpsc::channel();
    let sx2 = sx1.clone();
    let keybinds1 = std::sync::Arc::new(settings.keybinds.clone());
    let keybinds2 = keybinds1.clone();
    // Initialize callbacks with which to send
    let device_state = device_query::DeviceState::new();
    let _guard1 =  device_state.on_key_down(move |key| {
        let signal = match key {
            dq::Keycode::Escape => None,
            _ => match keybinds1.get(key) {
                None => return,
                Some(&button) => Some((button, true)),
            }
        };
        let _ = sx1.send(signal);
    });
    let _guard2 =  device_state.on_key_up(move |key| {
        let signal = match key {
            dq::Keycode::Escape => None,
            _ => match keybinds2.get(key) {
                None => return,
                Some(&button) => Some((button, false)),
            }
        };
        let _ = sx2.send(signal);
    });
    // Game Loop
    let start = Instant::now();
    let frame_len = Duration::from_secs_f64(1.0 / 60.0);
    for i in 1u32.. {
        let next_frame = start + i*frame_len;
        let frame_left = next_frame - Instant::now();
        match rx.recv_timeout(frame_left) {
            Ok(None) => return Ok(ScreenChange::Enter(Screen::Settings)),
            Ok(Some((button, is_press_down))) => {
                let now = Instant::now();
                let mut changes = ButtonMap::new(None);
                changes[button] = Some(is_press_down);
                game.update(Some(changes), now);
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let now = Instant::now();
                game.update(None, now);
            },
            Err(mpsc::RecvTimeoutError::Disconnected) => todo!(),
        };
    }
    Ok(ScreenChange::Exit)
}

pub fn run(w: &mut impl Write) -> std::io::Result<()> {
    // Initialize console
    terminal::enable_raw_mode()?;
    w.execute(terminal::EnterAlternateScreen)?;
    w.execute(cursor::Hide)?;
    //TODO use kitty someday w.execute(event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES))?;
    // Prepare and run main update loop
    let keybinds = HashMap::from([
        (dq::Keycode::Left, Button::MoveLeft),
        (dq::Keycode::Right, Button::MoveRight),
        (dq::Keycode::A, Button::RotateLeft),
        (dq::Keycode::D, Button::RotateRight),
        (dq::Keycode::Down, Button::DropSoft),
        (dq::Keycode::Up, Button::DropHard),
    ]);
    let mut settings = Settings { keybinds }; // Application settings
    let mut active_screens = vec![Screen::Title]; // Active screens
    loop {
        // Retrieve active screen, stop application if all exited
        let Some(screen) = active_screens.last_mut() else {
            break;
        };
        // Enter screen until it returns what to do next
        let update = match screen {
            Screen::Title => enter_title_screen(w),
            Screen::Settings => enter_settings(w, &mut settings),
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
    terminal::disable_raw_mode()?;
    //TODO use kitty someday w.execute(event::PopKeyboardEnhancementFlags)?;
    Ok(())
}