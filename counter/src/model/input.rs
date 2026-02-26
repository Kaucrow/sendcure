use crate::prelude::*;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Normal,
    /// The value represents the InputField being edited
    Editing(u8),
}

#[derive(Debug, Default)]
pub struct InputFields {
    inputs: Vec<InputField>,
}

impl InputFields {
    pub fn new(amount: usize) -> Self {
        let mut inputs = Vec::new();
        inputs.resize_with(amount, InputField::default);
        Self {
            inputs,
        }
    }

    pub fn get(&self, idx: usize) -> Result<&InputField> {
        self.inputs
            .get(idx)
            .ok_or(anyhow::Error::msg(format!("No input object for idx {}", idx)))
    }

    pub fn get_mut(&mut self, idx: usize) -> Result<&mut InputField> {
        self.inputs
            .get_mut(idx)
            .ok_or(anyhow::Error::msg(format!("No input object for idx {}", idx)))
    }
}

#[derive(Debug, Default)]
pub struct InputField {
    pub input: Input,
    pub blacklist: InputBlacklist,
}

#[derive(Debug, Default)]
pub enum InputBlacklist {
    #[default]
    None,
    Money,
    Alphabetic,
    AlphanumericNoSpace,
    Alphanumeric,
    NoSpace,
    Numeric,
}