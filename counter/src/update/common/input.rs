use crate::{
    prelude::*,
    model::input::{InputFields, InputMode, InputBlacklist},
};
use crossterm::event::{Event as CrosstermEvent};

pub fn input(key_event: KeyEvent, input_mode: &InputMode, inputs: &mut InputFields) -> Result<()> {
    let input_field = if let InputMode::Editing(idx) = input_mode {
        inputs.get_mut(*idx as usize)?
    } else {
        bail!("Invalid input mode: {:?}", input_mode)
    };

    // If key input is a character
    if let KeyCode::Char(char) = key_event.code {
        match input_field.blacklist {
            InputBlacklist::None => {}
            InputBlacklist::Money => {
                let input_value = input_field.input.value();

                if char != '.' {
                    if !char.is_numeric() {
                        return Ok(());
                    } else {
                        if let Some(dot_index) = input_value.find('.') {
                            if input_value[dot_index + 1..].len() == 2 { return Ok(()) }
                        }
                    }
                } else {
                    if input_value.contains('.') {
                        return Ok(())
                    }
                }
            }
            InputBlacklist::Alphabetic => {
                if !char.is_alphabetic() && char != ' ' {
                    return Ok(())
                }
            }
            InputBlacklist::AlphanumericNoSpace => {
                if !char.is_alphanumeric() {
                    return Ok(())
                }
            }
            InputBlacklist::Alphanumeric => {
                if !char.is_alphanumeric() && char != ' ' {
                    return Ok(())
                }
            }
            InputBlacklist::NoSpace => {
                if char == ' ' {
                    return Ok(())
                }
            }
            InputBlacklist::Numeric => {
                if !char.is_numeric() {
                    return Ok(())
                }
            }
        }
    }

    input_field.input.handle_event(&CrosstermEvent::Key(key_event));

    Ok(())
}

pub fn switch_input(input_mode: &mut InputMode) -> Result<()> {
    if let InputMode::Editing(field) = input_mode {
        if *field == 0 { *input_mode = InputMode::Editing(1) }
        else { *input_mode = InputMode::Editing(0) }
    }
    Ok(())
}