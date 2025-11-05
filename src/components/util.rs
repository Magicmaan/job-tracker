use ratatui::style::Color;

use crate::{components::job_item::JobListingState, database::schema::ApplicationStatus};

pub fn status_colour(status: ApplicationStatus) -> ratatui::style::Color {
    match status {
        ApplicationStatus::Applied => ratatui::style::Color::Cyan,
        ApplicationStatus::Interviewing => ratatui::style::Color::Blue,
        ApplicationStatus::Offered => ratatui::style::Color::Green,
        ApplicationStatus::Rejected => ratatui::style::Color::Red,
        ApplicationStatus::Withdrawn => ratatui::style::Color::Magenta,
        ApplicationStatus::Accepted => ratatui::style::Color::Blue,
    }
}

pub fn is_focused_field_to_fg_color(
    state: &JobListingState,
    desired_field: i8,
    more_than: bool,
) -> Color {
    if state.focused {
        if state.focused_field as i8 == desired_field
            || (more_than && state.focused_field as i8 > desired_field)
        {
            Color::Blue
        } else {
            Color::White
        }
    } else {
        Color::DarkGray
    }
}

pub fn is_focused_field_to_bg_color(
    state: &JobListingState,
    desired_field: i8,
    more_than: bool,
) -> Color {
    if state.focused {
        if state.focused_field as i8 == desired_field
            || (more_than && state.focused_field as i8 > desired_field)
        {
            Color::DarkGray
        } else {
            Color::Reset
        }
    } else {
        Color::Reset
    }
}
