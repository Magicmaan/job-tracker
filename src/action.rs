use crate::database::schema::JobApplication;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    //
    DispatchJobSearch,
    JobResults(Vec<JobApplication>),
    //
    IndexNext,
    IndexPrevious,
    FocusNext,
    FocusPrevious,
    UnFocusField,
    ChangeMode(crate::app::Mode),
    PopulateEditJobForm(JobApplication),

    EnterPopup(&'static str),
    ExitPopup,
    DispatchNotesPopupData(&'static str),
    NotesPopupData(&'static str),
}
