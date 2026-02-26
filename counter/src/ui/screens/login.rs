use crate::{
    prelude::*,
    HELP_TEXT,
    model::{
        common::TimeoutType,
        input::InputMode,
        screens,
        Popup,
    },
    ui::{popups, centered_rect},
};

pub fn render(app: &App, state: &screens::login::State, f: &mut Frame) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(centered_rect(&f.area(), 45, 8)?);

    let title_block = Block::default();

    let title = Paragraph::new(Text::from(
        "Login"
    ))
    .block(title_block)
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    let width = chunks[0].width.max(3) - 3;
    let name_scroll = state.inputs.get(0)?.input.visual_scroll(width as usize - "* C.I.: ".len());
    let password_scroll = state.inputs.get(1)?.input.visual_scroll(width as usize - "* Password: ".len());
    let mut name_style = Style::default();
    let mut password_style = Style::default();

    if let InputMode::Editing(field) = state.input_mode {
        if field == 0 {
            password_style = password_style.fg(Color::DarkGray);
            f.set_cursor_position((chunks[1].x
                            + ((state.inputs.get(0)?.input.visual_cursor()).max(name_scroll) - name_scroll) as u16
                            + "* C.I.: ".len() as u16
                            + 1,
                            chunks[1].y + 1,
                        ));
        } else {
            name_style = name_style.fg(Color::DarkGray);
            f.set_cursor_position((chunks[2].x
                            + ((state.inputs.get(1)?.input.visual_cursor()).max(password_scroll) - password_scroll) as u16
                            + "* Password: ".len() as u16
                            + 1,
                        chunks[2].y + 1,
                        ));
        }
    }

    let name_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(name_style);

    let input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("* C.I.: ", Style::default().fg(Color::Yellow)),
        Span::styled(state.inputs.get(0)?.input.value(), name_style)
    ])))
    .block(name_block)
    .scroll((0, name_scroll as u16));

    f.render_widget(input, chunks[1]);

    let password_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(password_style);

    let input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled("* Password: ", Style::default().fg(Color::Yellow)),
        Span::styled(state.inputs.get(1)?.input.value(), password_style)
    ])))
    .block(password_block)
    .scroll((0, password_scroll as u16));

    f.render_widget(input, chunks[2]);

    let help_text = {
        if state.failed_logins == 3 {
            Line::styled(format!("{}{}", HELP_TEXT.login.login_failed_lock, app.timeout.get(&TimeoutType::Login).unwrap().counter), Style::default().fg(Color::Red))
        }
        else if state.failed_logins > 0 {
            Line::styled(HELP_TEXT.login.login_failed, Style::default().fg(Color::Red))
        } else {
            Line::raw(HELP_TEXT.login.main)
        }
    };
    let help_block = Block::default();
    let help = Paragraph::new(help_text).block(help_block);
    f.render_widget(help, chunks[3]);

    if let Some(popup) = &state.active_popup {
        match popup {
            Popup::LoginSuccessful(pop_state) => popups::login::successful::render(app, state, pop_state, f)?,
            _ => unimplemented!()
        }
    }

    Ok(())
}