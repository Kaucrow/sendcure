use crate::{
    prelude::*,
    model::screens,
};

pub fn render(_app: &App, state: &screens::counter::State, area: Rect, f: &mut Frame) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    let width = area.width;

    let desc_text = " Description: ";
    let mut desc_style = Style::default().fg(Color::DarkGray);
    let mut desc_highlight_style = Style::default().fg(Color::DarkGray);
    let client_ci_scroll = state.inputs.get(0)?.input.visual_scroll(width as usize - desc_text.len());
    let desc_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(desc_style);
    let desc_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(desc_text, desc_highlight_style),
        Span::styled(state.inputs.get(1)?.input.value(), desc_style)
    ])))
    .block(desc_block)
    .scroll((0, client_ci_scroll as u16));

    f.render_widget(desc_input, chunks[0]);

    /*
    let mut header_style = Style::default().bg(Color::DarkGray);

    if let Some(2) = state.action_sel {
        header_style = header_style.bg(Color::Green);
    }

    let header_cells = ["ID", "Description", "Weight"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));

    let header = Row::new(header_cells)
        .style(header_style)
        .height(1)
        .bottom_margin(1);

    let rows = state.packages.iter().map(|p| {
        Row::new(vec![
            p.package_id.to_string(),
            p.package_desc.clone(),
            format!("{:.2}kg", p.package_weight),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),      // ID
            Constraint::Min(11),        // Desc
            Constraint::Length(9),      // Weight
        ],
    )
    .header(header)
    .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
    .highlight_spacing(HighlightSpacing::Always)
    .highlight_symbol("> ");

    f.render_stateful_widget(table, area, &mut state.package_table_state.clone());
    */

    Ok(())
}