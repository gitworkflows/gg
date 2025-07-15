use iced::{Element, widget::{button, column, container, text, pick_list}};
use crate::terminal::Message;
use crate::themes::ThemeManager;

pub struct ThemeSelector {
    available_themes: Vec<String>,
    selected_theme: Option<String>,
}

impl ThemeSelector {
    pub fn new(theme_manager: &ThemeManager) -> Self {
        let available_themes = theme_manager.get_available_themes();
        let current_theme = theme_manager.get_current_theme();
        
        ThemeSelector {
            available_themes: available_themes.clone(),
            selected_theme: available_themes.first().cloned(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let theme_picker = pick_list(
            &self.available_themes[..],
            self.selected_theme.as_ref(),
            |theme| Message::ThemeChanged(theme.clone())
        )
        .placeholder("Select a theme...");

        let reload_button = button("Reload Themes")
            .on_press(Message::ThemeReloaded);

        container(
            column![
                text("Theme Selection").size(16),
                theme_picker,
                reload_button,
            ]
            .spacing(8)
        )
        .padding(16)
        .into()
    }
}
