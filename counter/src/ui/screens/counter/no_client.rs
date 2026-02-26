use crate::{
    prelude::*,
    model::screens,
};

pub fn render(_app: &App, _state: &screens::counter::State, area: Rect, f: &mut Frame) -> Result<()> {
    let vertical_center_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ])
        .split(area);

    let no_client_selected = Paragraph::new("No client selected.")
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(no_client_selected, vertical_center_chunks[1]);

    Ok(())
}