#![allow(dead_code)] // Remove this once you start using the code

use std::{collections::HashMap, env, path::PathBuf};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use derive_deref::{Deref, DerefMut};
use directories::ProjectDirs;
use lazy_static::lazy_static;
use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, de::Deserializer};
use tracing::error;
use tui_textarea::Key;

use crate::{action::Action, app::Mode};

const CONFIG: &str = include_str!("../.config/config.json5");

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub data_dir: PathBuf,
    #[serde(default)]
    pub config_dir: PathBuf,
}

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub keybindings: KeyBindings,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        Ok(Self {
            keybindings: KeyBindings::default(),
        })
    }
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME"))
}

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct KeyBindings(pub HashMap<Vec<KeyEvent>, Action>);
impl Default for KeyBindings {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            vec![KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: crossterm::event::KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            }],
            Action::Quit,
        );

        KeyBindings(map)
    }
}
