use crate::prelude::*;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Normal,
    /// The value represents the InputField being edited
    Editing(u8),
}

#[derive(Debug, Default)]
pub struct InputFields(pub Input, pub Input);