use iced::{
    widget::{column, text, pick_list},
    Element, Length,
};

use crate::themes::WarpTheme;
use crate::terminal::Message;

#[derive(Debug, Clone)]
pub enum ThemeSelectorMessage {
    ThemeSelected(String),
}

pub struct ThemeSelector {
    available_themes: Vec<String>,
    selected_theme: String,
}

impl ThemeSelector {
    pub fn new(available_themes: Vec<String>, current_theme_name: String) -> Self {
        Self {
            available_themes,
            selected_theme: current_theme_name,
        }
    }

    pub fn update(&mut self, message: ThemeSelectorMessage) -> Option<Message> {
        match message {
            ThemeSelectorMessage::ThemeSelected(theme_name) => {
                self.selected_theme = theme_name.clone();
                Some(Message::ThemeChanged(WarpTheme::from_name(&theme_name)))
            }
        }
    }

    pub fn view(&self) -> Element<ThemeSelectorMessage> {
        column![
            text("Select Theme:"),
            pick_list(
                self.available_themes.clone(),
                Some(self.selected_theme.clone()),
                ThemeSelectorMessage::ThemeSelected,
            )
            .width(Length::Fill),
        ]
        .spacing(10)
        .into()
    }
}
