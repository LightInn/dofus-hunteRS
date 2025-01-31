use enigo::{Enigo, KeyboardControllable, MouseButton};
use thiserror::Error;

#[derive(Debug)]
pub enum Action {
    MouseClick { x: i32, y: i32 },
    TypeText(String),
    PressKey(String),
}

pub struct ActionHandler {
    enigo: Enigo,
}

impl ActionHandler {
    pub fn new() -> Self {
        Self {
            enigo: Enigo::new(),
        }
    }

    pub fn execute(&mut self, action: Action) -> Result<(), ActionError> {
        match action {
            Action::MouseClick { x, y } => {
                self.enigo.mouse_move_to(x, y);
                self.enigo.mouse_click(MouseButton::Left);
            }
            Action::TypeText(text) => {
                self.enigo.key_sequence(&text);
            }
            Action::PressKey(key) => {
                self.enigo.key_click(key.parse()?);
            }
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Invalid key: {0}")]
    InvalidKey(String),
}
