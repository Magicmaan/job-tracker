use crossterm::event::{KeyCode, MouseEventKind};
use ratatui::{
    Frame,
    layout::{Layout, Margin, Position, Rect},
    widgets::Paragraph,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    app::Mode,
    components::{
        component::Component,
        job_item::{JobItem, JobListingState},
    },
    config::Config,
    database::schema::JobApplication,
};
use color_eyre::Result;

#[derive(Default)]
pub struct JobListState {
    visible_start_index: usize,
    selected_index: usize,
    selected_job_state: JobListingState,
}

pub struct JobList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    jobs: Vec<JobApplication>,
    state: JobListState,
    area: Option<Rect>,
    notes_popup_visible: bool,
}

impl JobList {
    pub fn new() -> Self {
        Self {
            command_tx: None,
            config: Config::new().unwrap_or_default(),
            jobs: Vec::new(),
            state: JobListState::default(),
            area: None,
            notes_popup_visible: false,
        }
    }
    pub fn layout(&self, area: Rect) -> Layout {
        let num_rows = area.height / 8;
        Layout::vertical(
            std::iter::repeat(ratatui::layout::Constraint::Length(8))
                .take(num_rows as usize)
                .collect::<Vec<_>>(),
        )
    }
    fn set_visible_jobs(&mut self, start: usize) {
        self.state.visible_start_index = start;
    }
    pub fn get_visible_jobs(&self, area: Rect) -> usize {
        // let area = self.area.unwrap_or(Rect::default());
        let num_visible = (area.height / 8) as usize;
        let selected = self.state.selected_index;
        let jobs_len = self.jobs.len();

        // Center selected_index in the visible window if possible
        let mut start = if jobs_len > num_visible {
            if num_visible == 1 {
                selected
            } else if selected >= num_visible / 2 {
                std::cmp::min(
                    selected + 1 - num_visible / 2,
                    jobs_len.saturating_sub(num_visible),
                )
            } else {
                0
            }
        } else {
            0
        };
        start
    }
}

impl Component for JobList {
    fn mode(&self) -> Mode {
        Mode::Home
    }
    fn id(&self) -> String {
        "Job List".into()
    }
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        if self.jobs.len() == 0 {
            self.command_tx
                .clone()
                .unwrap()
                .send(Action::DispatchJobSearch)?;
        }
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            Action::JobResults(res) => {
                self.jobs = res;
            }
            Action::NotesPopupData(str) => {
                self.notes_popup_visible = false;
                self.jobs.get_mut(0).unwrap().notes = Some(str.into());
            }

            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let block = ratatui::widgets::Block::bordered()
            .border_type(ratatui::widgets::BorderType::Thick)
            .borders(ratatui::widgets::Borders::TOP)
            .title_top(ratatui::text::Line::from("Job Applications").centered());

        let region = area.inner(Margin::new(2, 2));
        let mut area = block.inner(region);
        self.area = Some(area);
        // Clamp height to a multiple of 8
        // inner.height = ((area.height / 8) * 8);
        // block.render(area, buf);

        frame.render_widget(block, region);

        // .padding(ratatui::layout::Padding::uniform(1));
        // Split the inner area into rows of height 8
        let layout = self.layout(area).split(area);

        let num_visible = (area.height / 8) as usize;
        let end = std::cmp::min(
            self.state.visible_start_index + num_visible,
            self.jobs.len(),
        );
        let visible_jobs = &self.jobs[self.state.visible_start_index..end];

        // Implementation for rendering the job list goes here
        for (chunk, job) in layout.iter().zip(visible_jobs.iter()) {
            let job_listing = JobItem::new(job.clone());
            let mut job_state = self.state.selected_job_state.clone();

            // Focus the first element in the visible jobs
            job_state.focused = layout.iter().position(|c| c == chunk) == Some(0);
            if self.state.selected_index < self.jobs.len()
                && self.jobs[self.state.selected_index].id == job.id
            {
                job_state.focused = true;
            } else {
                job_state.focused = false;
            }
            frame.render_stateful_widget(job_listing, *chunk, &mut job_state);
        }
        Ok(())
    }

    fn register_action_handler(
        &mut self,
        tx: UnboundedSender<Action>,
    ) -> color_eyre::eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::eyre::Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_events(
        &mut self,
        event: Option<crate::tui::Event>,
    ) -> color_eyre::eyre::Result<Option<Action>> {
        let action = match event {
            Some(crate::tui::Event::Key(key_event)) => self.handle_key_event(key_event)?,
            Some(crate::tui::Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event)?,
            _ => None,
        };
        Ok(action)
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Tab => {
                if self.state.selected_index + 1 < self.jobs.len() {
                    self.state.selected_index += 1;
                }
            }
            KeyCode::BackTab => {
                if self.state.selected_index > 0 {
                    self.state.selected_index -= 1;
                }
            }
            KeyCode::Enter => {
                // Send an action to edit the selected job
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::ChangeMode(Mode::EditJob))?;
                }
            }
            KeyCode::Esc => {
                // Send an action to go back to home mode
                if let Some(tx) = &self.command_tx {
                    tx.send(Action::ChangeMode(Mode::Popup("notes_popup")))?;

                    tx.send(Action::DispatchNotesPopupData(
                        "This is a test note popup.".into(),
                    ))?;
                }
            }
            KeyCode::Up => {
                if self.state.selected_index > 0 {
                    self.state.selected_index -= 1;
                    if self.area.is_some() {
                        self.state.visible_start_index = self.get_visible_jobs(self.area.unwrap());
                    }
                }
            }
            KeyCode::Down => {
                if self.state.selected_index + 1 < self.jobs.len() {
                    self.state.selected_index += 1;
                    if self.area.is_some() {
                        self.state.visible_start_index = self.get_visible_jobs(self.area.unwrap());
                    }
                }
            }
            KeyCode::Right => {
                if (self.state.selected_job_state.focused_field as i8) < 4 {
                    self.state.selected_job_state.focused_field += 1;
                }
            }
            KeyCode::Left => {
                if self.state.selected_job_state.focused_field as i8 > 0 {
                    self.state.selected_job_state.focused_field -= 1;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_mouse_event(
        &mut self,
        mouse: crossterm::event::MouseEvent,
    ) -> color_eyre::eyre::Result<Option<Action>> {
        let crossterm::event::MouseEvent {
            kind,
            column,
            row,
            modifiers,
        } = mouse;
        match kind {
            MouseEventKind::ScrollUp => {
                if self.state.selected_index > 0 {
                    self.state.selected_index -= 1;
                }
                if self.area.is_some() {
                    self.state.visible_start_index = self.get_visible_jobs(self.area.unwrap());
                }
            }
            MouseEventKind::ScrollDown => {
                if self.state.selected_index + 1 < self.jobs.len() {
                    self.state.selected_index += 1;
                }
                if self.area.is_some() {
                    self.state.visible_start_index = self.get_visible_jobs(self.area.unwrap());
                }
            }
            MouseEventKind::Moved => {
                let regions = self.layout(self.area.unwrap()).split(self.area.unwrap());
                for (i, region) in regions.iter().enumerate() {
                    let pos = Position::new(column, row);
                    if region.contains(pos) {
                        self.state.selected_index = self.state.visible_start_index + i;

                        let job_listing =
                            JobItem::new(self.jobs[self.state.selected_index].clone());
                        job_listing.handle_mouse_moved_in_region(
                            *region,
                            pos,
                            &mut self.state.selected_job_state,
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }
}
