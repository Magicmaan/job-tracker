use ratatui::{
    layout::Margin,
    style::{Color, Style},
    widgets,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, app::Mode, components::component::Component};

pub struct NotesPopup {
    command_tx: Option<UnboundedSender<Action>>,
    notes: String,
}
impl NotesPopup {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            notes: String::new(),
        }
    }
}

impl Component for NotesPopup {
    fn mode(&self) -> Mode {
        Mode::Popup("notes_popup")
    }
    fn id(&self) -> String {
        "Notes Popup".into()
    }
    fn register_action_handler(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<Action>,
    ) -> color_eyre::eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }
    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> color_eyre::eyre::Result<Option<crate::action::Action>> {
        match action {
            Action::DispatchNotesPopupData(notes) => {
                self.notes = notes.into();
            }
            _ => {}
        }
        Ok(None)
    }
    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<Action>> {
        match key.code {
            crossterm::event::KeyCode::Esc => {
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::ChangeMode(Mode::Home))?;
                    tx.send(Action::NotesPopupData("Cunt".into()))?;
                }
            }
            _ => {}
        }
        Ok(None)
    }
    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
    ) -> color_eyre::eyre::Result<()> {
        let area = area.inner(Margin {
            vertical: 4,
            horizontal: 8,
        });

        frame.render_widget(widgets::Clear, area);

        let block = ratatui::widgets::Block::bordered()
            .style(Style::default().bg(Color::Red).fg(Color::White))
            .title("Notes");
        let paragraph = widgets::Paragraph::new(self.notes.clone()).block(block);
        frame.render_widget(paragraph, area);
        Ok(())
    }
}
