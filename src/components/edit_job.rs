use std::{
    collections::HashMap,
    ops::{Add, AddAssign, SubAssign},
};

use crate::{
    action::Action, app::Mode, components::component::Component, database::schema::JobApplication,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    widgets::{Block, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::field;
use tui_textarea::TextArea;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Field {
    None = -1,
    Position = 0,
    PositionCategory = 1,
    CompanyName = 2,
    WorkType = 3,
    Location = 4,
    LocationType = 5,
    ApplicationDate = 6,
    Status = 7,
    ContactInfo = 8,
    Url = 9,
    Files = 10,
    Notes = 11,
}
impl Field {
    pub fn len() -> i8 {
        12
    }
}
impl Into<i8> for Field {
    fn into(self) -> i8 {
        self as i8
    }
}
impl From<i8> for Field {
    fn from(value: i8) -> Self {
        match value {
            -1 => Field::None,
            0 => Field::Position,
            1 => Field::PositionCategory,
            2 => Field::CompanyName,
            3 => Field::WorkType,
            4 => Field::Location,
            5 => Field::LocationType,
            6 => Field::ApplicationDate,
            7 => Field::Status,
            8 => Field::ContactInfo,
            9 => Field::Url,
            10 => Field::Files,
            11 => Field::Notes,
            _ => Field::None,
        }
    }
}
impl SubAssign<i8> for Field {
    fn sub_assign(&mut self, rhs: i8) {
        let new_value = (*self as i8) - rhs;
        *self = Field::from(new_value);
    }
}
impl AddAssign<i8> for Field {
    fn add_assign(&mut self, rhs: i8) {
        let new_value = (*self as i8) + rhs;
        *self = Field::from(new_value);
    }
}

pub struct EditJob<'a> {
    command_tx: Option<UnboundedSender<crate::action::Action>>,
    config: crate::config::Config,
    job: JobApplication,
    text_fields: HashMap<Field, TextArea<'a>>,
    focused_field: Field,
    focused_updated: bool,
}

impl<'a> EditJob<'a> {
    pub fn new() -> Self {
        let mut text_fields: HashMap<Field, TextArea<'a>> =
            Self::create_fields().unwrap_or_default();
        Self {
            command_tx: None,
            config: crate::config::Config::new().unwrap_or_default(),
            job: JobApplication::default(),
            text_fields,
            focused_field: Field::Position,
            focused_updated: false,
        }
    }

    fn update_focused(&mut self) {
        for (field, text_area) in self.text_fields.iter_mut() {
            let block = text_area.block().cloned().unwrap_or_default();
            let is_focused = *field == self.focused_field;
            let mut style = Style::default().fg(Color::White);
            if is_focused {
                style = Style::default().fg(ratatui::style::Color::Blue);
                text_area.cancel_selection();

                text_area.set_cursor_style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::UNDERLINED),
                );
                text_area.set_cursor_line_style(Style::default());
            } else {
                text_area.set_cursor_style(Style::default());
            }
            text_area.set_block(block.border_style(style));
        }
    }

    fn create_fields() -> Result<HashMap<Field, TextArea<'a>>> {
        let mut default_block = Block::bordered().padding(Padding::horizontal(1));

        let mut fields: HashMap<Field, TextArea<'a>> = HashMap::new();
        let mut position = TextArea::default();
        position.set_placeholder_text("Position");
        position.set_block(default_block.clone().title("Position"));

        fields.insert(Field::Position, position);
        fields.insert(Field::CompanyName, TextArea::default());
        fields.insert(Field::PositionCategory, TextArea::default());
        fields.insert(Field::WorkType, TextArea::default());
        fields.insert(Field::Location, TextArea::default());
        fields.insert(Field::LocationType, TextArea::default());
        fields.insert(Field::ApplicationDate, TextArea::default());
        fields.insert(Field::Status, TextArea::default());
        fields.insert(Field::Notes, TextArea::default());
        fields.insert(Field::ContactInfo, TextArea::default());
        fields.insert(Field::Url, TextArea::default());
        fields.insert(Field::Files, TextArea::default());

        let fields = fields
            .iter()
            .map(|(field, textarea)| {
                let title = format!("{:?}", field);
                let block = default_block.clone().title(title);
                let mut ta = textarea.clone();
                ta.set_block(block);
                (*field, ta)
            })
            .collect::<HashMap<Field, TextArea>>();

        Ok(fields)
    }
}

impl Component for EditJob<'_> {
    fn mode(&self) -> Mode {
        Mode::EditJob
    }
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<crate::action::Action>,
    ) -> color_eyre::eyre::Result<()> {
        let _ = tx; // to appease clippy
        Ok(())
    }

    fn register_config_handler(
        &mut self,
        config: crate::config::Config,
    ) -> color_eyre::eyre::Result<()> {
        let _ = config; // to appease clippy
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        match key {
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state,
            } => {
                self.focused_field += 1;
                if self.focused_field as i8 > Field::len() {
                    self.focused_field = Field::Position;
                }
                self.focused_updated = false;
            }
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: _,
            } => {
                self.focused_field -= 1;
                if (self.focused_field as i8) < 0_i8 {
                    self.focused_field = Field::from(Field::len() - 1);
                }
                self.focused_updated = false;
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: _,
                kind: _,
                state: _,
            } => {
                self.focused_field += 1;
                if self.focused_field as i8 > Field::len() {
                    self.focused_field = Field::Position;
                }
                self.focused_updated = false;
            }
            _ => {
                let field = self.text_fields.get_mut(&self.focused_field).unwrap();
                field.input(key);
            }
        }
        Ok(None)
    }

    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        match action {
            Action::Render => {
                let focus_updated = &self.focused_updated;
                if !focus_updated {
                    self.update_focused();
                    self.focused_updated = true;
                }
                // add any logic here that should run on every tick
            }
            Action::PopulateEditJobForm(job) => {
                self.job = job;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(
        &mut self,
        frame: &mut Frame,
        area: ratatui::prelude::Rect,
    ) -> color_eyre::eyre::Result<()> {
        let root = Block::bordered()
            .padding(Padding::uniform(1))
            .title("Edit Job");
        let inner = root.inner(area);
        frame.render_widget(root, area);

        let layout_horizontal =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(inner);

        let layout_vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);

        // Order chunks: c1 r1, c1 r2, ..., c1 r6, c2 r1, c2 r2, ..., c2 r6
        let col1_chunks = layout_vertical.split(layout_horizontal[0]);
        let col2_chunks = layout_vertical.split(layout_horizontal[1]);
        let layout = col1_chunks
            .iter()
            .chain(col2_chunks.iter())
            .copied()
            .collect::<Vec<ratatui::layout::Rect>>();

        // Position + Category
        let position_chunk = layout[0];
        let position_chunk_split =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(position_chunk);
        frame.render_widget(
            self.text_fields.get(&Field::Position).unwrap(),
            position_chunk_split[0],
        );
        frame.render_widget(
            self.text_fields.get(&Field::PositionCategory).unwrap(),
            position_chunk_split[1],
        );

        // Company
        let company_chunk = layout[1];
        frame.render_widget(
            self.text_fields.get(&Field::CompanyName).unwrap(),
            company_chunk,
        );

        // Work Type
        let work_type_chunk = layout[2];
        frame.render_widget(
            self.text_fields.get(&Field::WorkType).unwrap(),
            work_type_chunk,
        );

        // Location + Location Type
        let location_chunk = layout[3];
        let location_chunk_split =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(location_chunk);
        frame.render_widget(
            self.text_fields.get(&Field::Location).unwrap(),
            location_chunk_split[0],
        );
        frame.render_widget(
            self.text_fields.get(&Field::LocationType).unwrap(),
            location_chunk_split[1],
        );

        // Date
        let date_chunk = layout[4];
        frame.render_widget(
            self.text_fields.get(&Field::ApplicationDate).unwrap(),
            date_chunk,
        );

        // Status
        let status_chunk = layout[5];
        frame.render_widget(self.text_fields.get(&Field::Status).unwrap(), status_chunk);

        // Contact Info
        let contact_info_chunk = layout[6];
        frame.render_widget(
            self.text_fields.get(&Field::ContactInfo).unwrap(),
            contact_info_chunk,
        );

        // Url
        let url_chunk = layout[7];
        frame.render_widget(self.text_fields.get(&Field::Url).unwrap(), url_chunk);

        // Files
        // TODO: needs custom rendering for options
        let files_chunk = layout[8];
        frame.render_widget(self.text_fields.get(&Field::Files).unwrap(), files_chunk);

        // Notes
        let notes_chunk = layout[9];
        frame.render_widget(self.text_fields.get(&Field::Notes).unwrap(), notes_chunk);

        Ok(())
    }
}
