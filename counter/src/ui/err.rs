use crate::{
    prelude::*,
    ui::*,
};

pub fn render(_app: &App, f: &mut Frame) {
    let err_area = centered_rect(&f.area(), 21, 4).unwrap_or(Rect::default());

    let err_block = Block::default().borders(Borders::ALL).border_type(BorderType::Thick);

    let err =
        Paragraph::new(
            wrap_text(15, HELP_TEXT.common.render_err.to_string())
        )
        .block(err_block)
        .centered();

    f.render_widget(err, err_area);
}