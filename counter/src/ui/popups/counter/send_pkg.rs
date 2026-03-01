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
    _scn_state: &screens::counter::State,
    pop_state: &popups::counter::send_pkg::State,
    f: &mut Frame
) -> Result<()> {
    let popup_rect = centered_rect(&f.area(), 34, 5)?;

    let package_sent_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick);

    let text_highlight_style = Style::default().fg(Color::Yellow);

    let package_sent_popup = Paragraph::new(Text::from(vec![
        Line::from("Package sent with"),
        Line::from(vec![
            Span::raw("shipment ID "),
            Span::styled(pop_state.shipment_id.to_string(), text_highlight_style),
            Span::raw(" and"),
        ]),
        Line::from(vec![
            Span::raw("guide number "),
            Span::styled(pop_state.guide_num.to_string(), text_highlight_style),
        ]),
    ]))
    .alignment(Alignment::Center)
    .block(package_sent_block);

    f.render_widget(Clear, popup_rect);
    f.render_widget(package_sent_popup, popup_rect);

    Ok(())
}