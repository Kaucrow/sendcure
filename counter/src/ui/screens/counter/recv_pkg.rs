use crate::{
    prelude::*,
    model::screens::{self, counter},
};

pub fn render(
    _app: &App,
    state: &screens::counter::State,
    tab_state: &counter::recv_pkg::State,
    area: Rect,
    f: &mut Frame
) -> Result<()> {
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

    let rows = tab_state.packages.iter().map(|p| {
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

    f.render_stateful_widget(table, area, &mut tab_state.package_table_state.clone());

    Ok(())
}