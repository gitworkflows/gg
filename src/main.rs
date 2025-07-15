use iced::{executor, Application, Command, Element, Settings, Theme};
use iced::widget::{column, container, scrollable, text_input, button, row};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::mpsc;
use uuid::Uuid;

mod terminal;
mod blocks;
mod shell;
mod editor;
mod renderer;
mod fuzzy;
mod collaboration;
mod themes;
mod preferences;
mod theme_customizer;
mod config;
mod profiles;
mod profile_manager_ui;
mod profile_switcher;

use terminal::WarpTerminal;

fn main() -> iced::Result {
    tracing_subscriber::init();
    
    WarpTerminal::run(Settings {
        antialiasing: true,
        default_font: iced::Font::MONOSPACE,
        ..Settings::default()
    })
}
