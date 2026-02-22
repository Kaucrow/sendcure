use crate::prelude::*;

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Normal,
    /// The value represents the InputField being edited
    Editing(u8),
}

#[derive(Debug, Default)]
pub struct InputFields(pub InputField, pub InputField);

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