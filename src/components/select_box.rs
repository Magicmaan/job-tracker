use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

pub struct SelectBoxState {
    pub selected_index: usize,
}

pub struct SelectBox {
    pub options: Vec<String>,
    pub title: String,
}

impl StatefulWidget for SelectBox {
    type State = SelectBoxState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::ALL).title(self.title);

        let selected = self
            .options
            .get(state.selected_index)
            .cloned()
            .unwrap_or_default();

        let text = Text::from(Line::from(vec![
            Span::raw(selected),
            Span::styled("↕", Style::default()),
        ]));

        let paragraph = Paragraph::new(text).block(block);

        paragraph.render(area, buf);
    }
}
