use std::{
    collections::HashMap,
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
    time::Instant,
};

use crossterm::event::{self, Event, KeyEvent};
use device_query::{CallbackGuard, DeviceEvents};

use crate::backend::game::Button;

pub use crossterm::event::KeyCode as CT_Keycode;
pub use device_query::keymap::Keycode as DQ_Keycode;

pub type ButtonSignal = Option<(Instant, Button, bool)>;

pub trait GameInputHandler {
    type KeycodeType;
}

struct CrosstermHandler {
    _handle: JoinHandle<()>,
}

impl GameInputHandler for CrosstermHandler {
    type KeycodeType = CT_Keycode;
}

#[allow(dead_code)]
pub fn new_input_handler_crossterm(
    sender: &Sender<ButtonSignal>,
    keybinds: &HashMap<CT_Keycode, Button>,
) -> Box<dyn GameInputHandler<KeycodeType = CT_Keycode>> {
    let sender = sender.clone();
    let keybinds = std::sync::Arc::new(keybinds.clone());
    let _handle = thread::spawn(move || {
        loop {
            let event = match event::read() {
                Ok(event) => event,
                // Spurious io::Error: ignore.
                Err(_) => continue,
            };
            let instant = Instant::now();
            let button_signals = match event {
                // Escape pressed: send interrupt.
                Event::Key(KeyEvent {
                    code: CT_Keycode::Esc,
                    ..
                }) => vec![None],
                // Candidate key pressed.
                Event::Key(KeyEvent { code: key, .. }) => match keybinds.get(&key) {
                    // Binding found: send button press.
                    Some(&button) => vec![
                        Some((instant, button, true)),
                        Some((instant, button, false)),
                    ],
                    // No binding: ignore.
                    None => continue,
                },
                // Don't care about other events: ignore.
                _ => continue,
            };
            for button_signal in button_signals {
                // crossterm::QueueableCommand::queue(&mut std::io::stderr(), crossterm::style::Print(format!("ct-send: {button_signal:?}."))).unwrap();
                // crossterm::QueueableCommand::queue(&mut std::io::stderr(), crossterm::cursor::MoveToNextLine(1)).unwrap();
                let _ = sender.send(button_signal);
            }
        }
    });
    Box::new(CrosstermHandler { _handle })
}

struct DeviceQueryHandler<D, U> {
    _guard_key_down: CallbackGuard<D>,
    _guard_key_up: CallbackGuard<U>,
}

impl<D, U> GameInputHandler for DeviceQueryHandler<D, U> {
    type KeycodeType = DQ_Keycode;
}

#[allow(dead_code)]
pub fn new_input_handler_devicequery(
    sender: &Sender<ButtonSignal>,
    keybinds: &HashMap<DQ_Keycode, Button>,
) -> Box<dyn GameInputHandler<KeycodeType = DQ_Keycode>> {
    let sender1 = sender.clone();
    let sender2 = sender.clone();
    let keybinds1 = std::sync::Arc::new(keybinds.clone());
    let keybinds2 = keybinds1.clone();
    // Initialize callbacks which send `Button` inputs.
    let device_state = device_query::DeviceState::new();
    let _guard_key_down = device_state.on_key_down(move |key| {
        let instant = Instant::now();
        let button_signal = match key {
            // Escape pressed: send interrupt.
            DQ_Keycode::Escape => None,
            // Candidate key pressed.
            key => match keybinds1.get(key) {
                // Binding found: send button press.
                Some(&button) => Some((instant, button, true)),
                // No binding: ignore.
                None => return,
            },
        };
        let _ = sender1.send(button_signal);
    });
    let _guard_key_up = device_state.on_key_up(move |key| {
        let instant = Instant::now();
        let button_signal = match key {
            // Escape released: ignore.
            DQ_Keycode::Escape => return,
            // Candidate key pressed.
            key => match keybinds2.get(key) {
                // Binding found: send button release.
                Some(&button) => Some((instant, button, false)),
                // No binding: ignore.
                None => return,
            },
        };
        let _ = sender2.send(button_signal);
    });
    Box::new(DeviceQueryHandler {
        _guard_key_down,
        _guard_key_up,
    })
}
