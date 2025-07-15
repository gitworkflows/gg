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
// mod fuzzy; // Removed, now part of fuzzy_match
mod collaboration;
mod themes;
mod preferences;
mod theme_customizer;
mod config;
mod profiles;
mod profile_manager_ui;
mod profile_switcher;
mod workflows;
mod workflow_browser;
mod workflow_executor;

// New modules
mod agent_mode_eval;
mod asset_macro; // Empty mod.rs
mod command; // Empty mod.rs
mod fuzzy_match; // Now contains fuzzy logic
mod graphql; // Empty mod.rs
mod integration; // Empty mod.rs
mod languages; // Empty mod.rs
mod markdown_parser;
mod lpc; // Empty mod.rs
mod mcq; // Empty mod.rs
mod natural_language_detection;
mod resources; // Empty mod.rs
mod serve_wasm; // Empty mod.rs
mod string_offset; // Empty mod.rs
mod sum_tree; // Empty mod.rs
mod syntax_tree;
mod virtual_fs;
mod watcher;
mod drive; // Empty mod.rs
mod websocket; // Now contains websocket client

use terminal::WarpTerminal;

fn main() -> iced::Result {
    tracing_subscriber::init();
    
    WarpTerminal::run(Settings {
        antialiasing: true,
        default_font: iced::Font::MONOSPACE,
        ..Settings::default()
    })
}
