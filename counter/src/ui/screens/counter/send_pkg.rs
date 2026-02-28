use crate::{
    prelude::*,
    model::screens::{self, counter},
};

pub fn render(
    _app: &App,
    state: &screens::counter::State,
    tab_state: &counter::send_pkg::State,
    area: Rect,
    f: &mut Frame
) -> Result<()> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(100),
            Constraint::Length(3),
        ])
        .split(area);

    // ===============================
    //  Header
    // ===============================

    let header_area = chunks[0].inner(Margin {
        horizontal: 3,
        vertical: 0,
    });

    let header_style = if let Some(2) = state.action_sel {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(header_style);

    let header = Paragraph::new("Package Details")
        .alignment(Alignment::Center)
        .block(header_block);

    f.render_widget(header, header_area);

    // ===============================
    //  Inputs
    // ===============================

    let inputs_area = chunks[1].inner(Margin {
        horizontal: 3,
        vertical: 0,
    });

    let input_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(inputs_area);

    let input_chunks_1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ])
        .split(input_chunks[3]);

    let input_chunks_2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(1),
            Constraint::Percentage(50),
        ])
        .split(input_chunks[4]);

    let inactive_style = Style::default().fg(Color::DarkGray);
    let active_border: Style = Style::default().fg(Color::White);
    let active_label: Style = Style::default().fg(Color::Yellow);

    let get_styles = |input_index: usize| -> (Style, Style) {
        if let Some(2) = state.action_sel {
            if let Some(field) = tab_state.inputs.selected_idx() {
                if field == input_index {
                    return (active_border, active_label);
                } else {
                    return (inactive_style, inactive_style);
                }
            }
        }
        (inactive_style, inactive_style)
    };

    let input_block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Rounded);

    // ===== Recipient input =====

    let recipient_width = input_chunks[0].width;
    let recipient_style = get_styles(0);
    let recipient_text = " Recipient C.I.: ";
    let recipient_scroll = tab_state.inputs.get(0)?.input.visual_scroll(recipient_width as usize - recipient_text.len());
    let recipient_block = input_block.clone()
        .border_style(recipient_style.0);
    let recipient_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(recipient_text, recipient_style.1),
        Span::styled(tab_state.inputs.get(0)?.input.value(), recipient_style.0)
    ])))
    .block(recipient_block)
    .scroll((0, recipient_scroll as u16));

    f.render_widget(recipient_input, input_chunks[0]);

    // ===== Address input =====

    let address_width = input_chunks[1].width;
    let address_style = get_styles(1);
    let address_text = " Address: ";
    let address_scroll = tab_state.inputs.get(1)?.input.visual_scroll(address_width as usize - address_text.len());
    let address_block = input_block.clone()
        .border_style(address_style.0);
    let address_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(address_text, address_style.1),
        Span::styled(tab_state.inputs.get(1)?.input.value(), address_style.0)
    ])))
    .block(address_block)
    .scroll((0, address_scroll as u16));

    f.render_widget(address_input, input_chunks[1]);

    // ===== Description input =====

    let desc_width = input_chunks[2].width;
    let desc_style = get_styles(2);
    let desc_text = " Description: ";
    let desc_scroll = tab_state.inputs.get(2)?.input.visual_scroll(desc_width as usize - desc_text.len());
    let desc_block = input_block.clone()
        .border_style(desc_style.0);
    let desc_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(desc_text, desc_style.1),
        Span::styled(tab_state.inputs.get(2)?.input.value(), desc_style.0)
    ])))
    .block(desc_block)
    .scroll((0, desc_scroll as u16));

    f.render_widget(desc_input, input_chunks[2]);

    // ===== Weight input =====

    let weight_width = input_chunks_1[0].width;
    let weight_style = get_styles(3);
    let weight_text = " Weight: ";
    let weight_scroll = tab_state.inputs.get(3)?.input.visual_scroll(weight_width as usize - weight_text.len());
    let weight_block = input_block.clone()
        .border_style(weight_style.0);
    let weight_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(weight_text, weight_style.1),
        Span::styled(tab_state.inputs.get(3)?.input.value(), weight_style.0)
    ])))
    .block(weight_block)
    .scroll((0, weight_scroll as u16));

    f.render_widget(weight_input, input_chunks_1[0]);

    // ===== Length input =====

    let length_width = input_chunks_1[2].width;
    let length_style = get_styles(4);
    let length_text = " Length: ";
    let length_scroll = tab_state.inputs.get(4)?.input.visual_scroll(length_width as usize - length_text.len());
    let length_block = input_block.clone()
        .border_style(length_style.0);
    let length_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(length_text, length_style.1),
        Span::styled(tab_state.inputs.get(4)?.input.value(), length_style.0)
    ])))
    .block(length_block)
    .scroll((0, length_scroll as u16));

    f.render_widget(length_input, input_chunks_1[2]);

    // ===== Width input =====
    
    let width_width = input_chunks_2[0].width;
    let width_style = get_styles(5);
    let width_text = " Width: ";
    let width_scroll = tab_state.inputs.get(5)?.input.visual_scroll(width_width as usize - width_text.len());
    let width_block = input_block.clone()
        .border_style(width_style.0);
    let width_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(width_text, width_style.1),
        Span::styled(tab_state.inputs.get(5)?.input.value(), width_style.0)
    ])))
    .block(width_block)
    .scroll((0, width_scroll as u16));

    f.render_widget(width_input, input_chunks_2[0]);

    // ===== Height input =====

    let height_width = input_chunks_2[2].width;
    let height_style = get_styles(6);
    let height_text = " Height: ";
    let height_scroll = tab_state.inputs.get(6)?.input.visual_scroll(height_width as usize - height_text.len());
    let height_block = input_block.clone()
        .border_style(height_style.0);
    let height_input = Paragraph::new(Text::from(Line::from(vec![
        Span::styled(height_text, height_style.1),
        Span::styled(tab_state.inputs.get(6)?.input.value(), height_style.0)
    ])))
    .block(height_block)
    .scroll((0, height_scroll as u16));

    f.render_widget(height_input, input_chunks_2[2]);

    // ===============================
    //  Cursor placement
    // ===============================

    if let Some(2) = state.action_sel {
        if let Some(selected_idx) = tab_state.inputs.selected_idx() {
            let (active_rect, prompt_len) = match selected_idx {
                0 => (input_chunks[0], recipient_text.len()),
                1 => (input_chunks[1], address_text.len()),
                2 => (input_chunks[2], desc_text.len()),
                3 => (input_chunks_1[0], weight_text.len()),
                4 => (input_chunks_1[2], length_text.len()),
                5 => (input_chunks_2[0], width_text.len()),
                6 => (input_chunks_2[2], height_text.len()),
                _ => (Rect::default(), 0),
            };

            let active_input = &tab_state.inputs.get(selected_idx)?.input;
            let scroll = active_input.visual_scroll(active_rect.width as usize - prompt_len);

            let cursor_x = active_rect.x 
                + prompt_len as u16 
                + active_input.visual_cursor() as u16 
                - scroll as u16;

            let cursor_y = active_rect.y;

            f.set_cursor_position((cursor_x, cursor_y));
        }
    }

    // ===============================
    //  Send package button
    // ===============================

    let send_btn_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Length(16),
            Constraint::Percentage(100),
        ])
        .split(chunks[2])[1];

    let send_btn_style = if let Some(1) = tab_state.action_sel {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let send_btn_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(send_btn_style);

    let send_btn = Paragraph::new("Send Package")
        .alignment(Alignment::Center)
        .block(send_btn_block);

    f.render_widget(send_btn, send_btn_area);

    Ok(())
}