use crate::{
    prelude::*,
    model::{
        screens,
        popups,
    },
    ui::centered_rect,
};

pub fn render(
    _app: &App,
    _scn_state: &screens::login::State,
    _pop_state: &popups::login::successful::State,
    f: &mut Frame
) -> Result<()> {
    let popup_rect = centered_rect(&f.area(), 28, 3)?;

    let login_successful_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);

    let login_successful_popup = Paragraph::new(Text::from(
        "Login successful."
    ))
    .alignment(Alignment::Center)
    .block(login_successful_block);

    f.render_widget(Clear, popup_rect);
    f.render_widget(login_successful_popup, popup_rect);

    Ok(())
}