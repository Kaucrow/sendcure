pub mod prelude;
pub mod settings;
pub mod tui;
pub mod event;
pub mod model;
pub mod ui;
pub mod update;

use model::help_text::HelpText;

pub const HELP_TEXT: HelpText = HelpText::default();