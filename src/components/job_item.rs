use std::ops::{Add, AddAssign, SubAssign};

use crossterm::event::MouseEvent;
use ratatui::{
    layout::{Layout, Position},
    style::{self, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget},
};

use crate::{
    components::util::{is_focused_field_to_bg_color, is_focused_field_to_fg_color},
    database::schema::JobApplication,
};

#[derive(Clone, Default, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FocusedField {
    #[default]
    None = -1,
    Status = 0,
    Notes = 1,
    ApplicationLink = 2,
    CompanyWebsite = 3,
    CV = 4,
    CoverLetter = 5,
}
impl FocusedField {
    pub fn len() -> i8 {
        7
    }
}
impl Into<i8> for FocusedField {
    fn into(self) -> i8 {
        self as i8
    }
}
impl From<i8> for FocusedField {
    fn from(value: i8) -> Self {
        match value {
            0 => FocusedField::Status,
            1 => FocusedField::Notes,
            2 => FocusedField::ApplicationLink,
            3 => FocusedField::CompanyWebsite,
            4 => FocusedField::CV,
            5 => FocusedField::CoverLetter,
            _ => FocusedField::None,
        }
    }
}

impl SubAssign<i8> for FocusedField {
    fn sub_assign(&mut self, rhs: i8) {
        let new_value = (*self as i8) - rhs;
        *self = FocusedField::from(new_value);
    }
}
impl AddAssign<i8> for FocusedField {
    fn add_assign(&mut self, rhs: i8) {
        let new_value = (*self as i8) + rhs;
        *self = FocusedField::from(new_value);
    }
}
#[derive(Clone, Default)]
pub struct JobListingState {
    pub focused: bool,
    pub focused_field: FocusedField,
}

pub struct JobItem {
    job: JobApplication,
}

impl JobItem {
    pub fn new(job: JobApplication) -> Self {
        JobItem { job }
    }
    pub fn handle_mouse_event(mouse_event: MouseEvent, state: &mut JobListingState) {
        let pos = Position::new(mouse_event.column, mouse_event.row);
    }

    pub fn handle_mouse_moved_in_region(
        &self,
        region: ratatui::layout::Rect,
        pos: Position,
        state: &mut JobListingState,
    ) {
        let item_layout = self.layout(region).split(region);

        for (j, item_region) in item_layout.iter().enumerate() {
            if j == 1 && item_region.contains(pos) {
                state.focused_field = FocusedField::Notes;
            }
            if j == 2 && item_region.contains(pos) {
                // Adjust for padding of 1 at the top
                let mut relative_row = pos.y.saturating_sub(item_region.y);
                if relative_row == 0 {
                    relative_row = 1;
                } else {
                    relative_row -= 1;
                }
                let field_height = (item_region.height.saturating_sub(2)) / 4;
                let field_index = relative_row / field_height;
                state.focused_field = FocusedField::from(field_index as i8);
            }
        }
    }
    pub fn info_block(&self, state: &JobListingState) -> Paragraph {
        // left side block with basic job info
        let left_block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(if state.focused {
                ratatui::style::Style::default().fg(ratatui::style::Color::White)
            } else {
                ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray)
            })
            .padding(Padding::uniform(1))
            .title_top(Line::from(self.job.status.to_string()).centered().style(
                style::Style::default().fg(crate::components::util::status_colour(
                    self.job.status.clone(),
                )),
            ))
            .title_top(Line::from(self.job.application_date.clone()).left_aligned());
        // block.render(chunks[0], buf);

        let lines = Text::from(vec![
            Line::from(Span::styled(self.job.position.clone(), Style::default())),
            Line::from(Span::styled(
                self.job.company_name.clone(),
                Style::default(),
            )),
            Line::from(Span::raw("")),
            Line::from(Span::styled(self.job.location.clone(), Style::default())),
        ]);

        ratatui::widgets::Paragraph::new(lines)
            .centered()
            .block(left_block)
    }

    pub fn links_block(&self, state: &JobListingState) -> Paragraph {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(is_focused_field_to_fg_color(state, 1, true))
            .padding(Padding::uniform(1))
            .title_top(Line::from("Links").centered());

        let style = Style::default().fg(ratatui::style::Color::Blue);
        let modifier = ratatui::style::Modifier::UNDERLINED;

        let lines = Text::from(vec![
            Line::from(Span::styled(
                "Application Link".to_string(),
                style
                    .clone()
                    .bg(is_focused_field_to_bg_color(
                        state,
                        FocusedField::ApplicationLink as i8,
                        false,
                    ))
                    .add_modifier(modifier),
            )),
            Line::from(Span::styled(
                "Company Website".to_string(),
                style
                    .clone()
                    .bg(is_focused_field_to_bg_color(
                        state,
                        FocusedField::CompanyWebsite as i8,
                        false,
                    ))
                    .add_modifier(modifier),
            )),
            Line::from(Span::styled(
                "CV".to_string(),
                style
                    .clone()
                    .bg(is_focused_field_to_bg_color(
                        state,
                        FocusedField::CV as i8,
                        false,
                    ))
                    .add_modifier(modifier),
            )),
            Line::from(Span::styled(
                "Cover Letter".to_string(),
                style
                    .clone()
                    .bg(is_focused_field_to_bg_color(
                        state,
                        FocusedField::CoverLetter as i8,
                        false,
                    ))
                    .add_modifier(modifier),
            )),
        ]);

        ratatui::widgets::Paragraph::new(lines)
            .centered()
            .block(block)
    }

    pub fn notes_block(&self, state: &JobListingState) -> Paragraph {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(if state.focused {
                if state.focused_field == FocusedField::Notes {
                    ratatui::style::Style::default().fg(ratatui::style::Color::Blue)
                } else {
                    ratatui::style::Style::default().fg(ratatui::style::Color::White)
                }
            } else {
                ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray)
            })
            .padding(Padding::uniform(1))
            .title_top(Line::from("Notes").centered());

        let lines = Text::from(vec![Line::from(Span::raw(
            self.job.notes.clone().unwrap_or_default(),
        ))]);

        ratatui::widgets::Paragraph::new(lines)
            .centered()
            .block(block)
    }

    pub fn layout(&self, area: ratatui::layout::Rect) -> Layout {
        Layout::horizontal([
            ratatui::layout::Constraint::Length(40),
            ratatui::layout::Constraint::Fill(2),
            ratatui::layout::Constraint::Fill(1),
        ])
    }
}

impl StatefulWidget for JobItem {
    type State = JobListingState;
    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        let layout = self.layout(area).split(area);
        // let chunks = layout.split(area);

        self.info_block(state).render(layout[0], buf);

        self.notes_block(state).render(layout[1], buf);

        self.links_block(state).render(layout[2], buf);
    }
}
