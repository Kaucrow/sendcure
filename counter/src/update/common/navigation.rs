use crate::prelude::*;

pub fn navigate(action_sel: &mut Option<u8>, max: u8) -> Result<()> {
    if let Some(action) = action_sel {
        if *action == max || *action == u8::MAX {
            // If action is equal to max or 255, reset
            *action_sel = Some(0);
        } else {
            // Else, increase by 1
            *action_sel = Some(*action + 1);
        }
    } else {
        // If action is None, set it to 0
        *action_sel = Some(0);
    }

    Ok(())
}