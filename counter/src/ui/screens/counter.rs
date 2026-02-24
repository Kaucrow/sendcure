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

pub fn render(app: &App, state: &screens::counter::State, f: &mut Frame) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ])
        .split(centered_rect(&f.area(), 65, 20)?);

    let width = chunks[0].width.max(3) - 3;

    // ===============================
    //  Block styles
    // ===============================

    let client_ci_text = " Client C.I.: ";
    let mut client_ci_style = Style::default().fg(Color::DarkGray);
    let mut client_ci_highlight_style = Style::default().fg(Color::DarkGray);
    let client_ci_scroll = state.inputs.0.input.visual_scroll(width as usize - client_ci_text.len());

    let mut recv_pkg_style = Style::default().fg(Color::DarkGray);

    let mut sidebar_style = Style::default().fg(Color::DarkGray);
    let mut sidebar_highlight_style = Style::default().fg(Color::DarkGray);

    let mut table_style = Style::default().fg(Color::DarkGray);

    match state.action_sel {
        Some(0) => {
            client_ci_style = client_ci_style.fg(Color::White);
            client_ci_highlight_style = client_ci_highlight_style.fg(Color::Yellow);

            f.set_cursor_position((chunks[0].x
                            + ((state.inputs.0.input.visual_cursor()).max(client_ci_scroll) - client_ci_scroll) as u16
                            + client_ci_text.len() as u16
                            + 1,
                            chunks[0].y + 1,
                        ));
        }
        Some(1) => {
            recv_pkg_style = recv_pkg_style.fg(Color::White);
            sidebar_style = sidebar_style.fg(Color::White);
            sidebar_highlight_style = sidebar_highlight_style.fg(Color::Yellow);
        }
        Some(2) => {
            recv_pkg_style = recv_pkg_style.fg(Color::White);
            table_style = table_style.fg(Color::White);
        }
        _ => {}
    }

    // ===============================
    //  Client CI Input
    // ===============================

    let client_ci_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(client_ci_style);

    let input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(client_ci_text, client_ci_highlight_style),
        Span::styled(state.inputs.0.input.value(), client_ci_style)
    ])))
    .block(client_ci_block)
    .scroll((0, client_ci_scroll as u16));

    f.render_widget(input, chunks[0]);

    // ===============================
    //  Received packages block
    // ===============================

    let recv_pkg_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(recv_pkg_style);

    f.render_widget(&recv_pkg_block, chunks[1]);

    // ===============================
    //  Main chunks
    // ===============================

    let inner_area = recv_pkg_block.inner(chunks[1]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75)
        ])
        .split(inner_area);

    // ===============================
    //  Sidebar
    // ===============================

    let sidebar_block = Block::default()
        .borders(Borders::RIGHT)
        .border_type(BorderType::Plain)
        .border_style(recv_pkg_style)
        .padding(Padding::new(1, 1, 0, 1));

    f.render_widget(&sidebar_block, main_chunks[0]);

    let sidebar_inner_area = sidebar_block.inner(main_chunks[0]);

    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Percentage(100),
        ])
        .split(sidebar_inner_area);

    let sidebar_header_block = Block::default().borders(Borders::BOTTOM).style(sidebar_style);
    let sidebar_header = Paragraph::new("Counter").block(sidebar_header_block);

    f.render_widget(sidebar_header, sidebar_chunks[0]);

    let sidebar_items: Vec<ListItem> = vec![
        ListItem::new(vec![
            Line::from("Received"),
            Line::from("packages"),
            Line::raw(""),
        ]),
        ListItem::new(vec![
            Line::from("Send a"),
            Line::from("package"),
        ]),
    ];

    let sidebar = List::new(sidebar_items)
        .style(sidebar_style)
        .highlight_symbol("> ")
        .highlight_style(sidebar_highlight_style);

    let mut sidebar_state = state.sidebar_state.clone();

    f.render_stateful_widget(sidebar, sidebar_chunks[1], &mut sidebar_state);

    // ===============================
    //  Main container
    // ===============================

    let main_block = Block::default().style(table_style);
    let main_inner_area = main_block.inner(main_chunks[1]);
    f.render_widget(main_block, main_chunks[1]);

    let vertical_center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), // Dynamic empty space above
            Constraint::Length(1),      // The exact height of our text
            Constraint::Percentage(50), // Dynamic empty space below
        ])
        .split(main_inner_area);

    let no_client_selected = Paragraph::new("No client selected.")
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(no_client_selected, vertical_center_chunks[1]);

    // ===============================
    //  Help text
    // ===============================

    let help_text = {
        /*if state.failed_logins == 3 {
            Line::styled(format!("{}{}", HELP_TEXT.login.login_failed_lock, app.timeout.get(&TimeoutType::Login).unwrap().counter), Style::default().fg(Color::Red))
        }
        else if state.failed_logins > 0 {
            Line::styled(HELP_TEXT.login.login_failed, Style::default().fg(Color::Red))
        } else {*/
            Line::raw(HELP_TEXT.counter.recv_pkg)
        //}
    };
    let help_block = Block::default();
    let help = Paragraph::new(help_text).block(help_block);
    f.render_widget(help, chunks[2]);

    /*if let Some(popup) = &state.active_popup {
        match popup {
            Popup::LoginSuccessful(pop_state) => popups::login::successful::render(app, state, pop_state, f)?,
            _ => unimplemented!()
        }
    }*/

    Ok(())
}