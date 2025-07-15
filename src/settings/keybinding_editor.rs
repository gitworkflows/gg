use iced::{
    widget::{column, row, text, button, text_input, scrollable, Space},
    Alignment, Element, Length,
};
use crate::config::{KeyBindings, ConfigManager};
use crate::config::theme::WarpTheme;
use log::info;

#[derive(Debug, Clone)]
pub enum KeybindingEditorMessage {
    KeybindingChanged(String, String), // (key_name, new_value)
    SaveClicked,
    CancelClicked,
    ResetToDefaults,
}

pub struct KeybindingEditor {
    current_keybindings: KeyBindings,
    original_keybindings: KeyBindings,
    config_manager: ConfigManager,
}

impl KeybindingEditor {
    pub fn new(config_manager: ConfigManager) -> Self {
        let current_keybindings = config_manager.get_keybindings().clone();
        let original_keybindings = current_keybindings.clone();
        KeybindingEditor {
            current_keybindings,
            original_keybindings,
            config_manager,
        }
    }

    pub fn update(&mut self, message: KeybindingEditorMessage) {
        match message {
            KeybindingEditorMessage::KeybindingChanged(key_name, new_value) => {
                info!("Keybinding changed: {} to {}", key_name, new_value);
                match key_name.as_str() {
                    "submit_input" => self.current_keybindings.submit_input = new_value,
                    "history_up" => self.current_keybindings.history_up = new_value,
                    "history_down" => self.current_keybindings.history_down = new_value,
                    "clear_terminal" => self.current_keybindings.clear_terminal = new_value,
                    "toggle_fullscreen" => self.current_keybindings.toggle_fullscreen = new_value,
                    "open_command_palette" => self.current_keybindings.open_command_palette = new_value,
                    "open_preferences" => self.current_keybindings.open_preferences = new_value,
                    "open_theme_customizer" => self.current_keybindings.open_theme_customizer = new_value,
                    "open_profile_manager" => self.current_keybindings.open_profile_manager = new_value,
                    "open_workflow_browser" => self.current_keybindings.open_workflow_browser = new_value,
                    "open_warp_drive" => self.current_keybindings.open_warp_drive = new_value,
                    _ => info!("Unknown keybinding: {}", key_name),
                }
            }
            KeybindingEditorMessage::SaveClicked => {
                info!("Saving keybindings...");
                if let Err(e) = self.config_manager.update_keybindings(self.current_keybindings.clone()) {
                    error!("Failed to save keybindings: {}", e);
                } else {
                    self.original_keybindings = self.current_keybindings.clone(); // Update original after successful save
                }
            }
            KeybindingEditorMessage::CancelClicked => {
                info!("Cancelling keybinding changes.");
                self.current_keybindings = self.original_keybindings.clone(); // Revert to original
            }
            KeybindingEditorMessage::ResetToDefaults => {
                info!("Resetting keybindings to defaults.");
                self.current_keybindings = KeyBindings::default();
            }
        }
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<KeybindingEditorMessage> {
        let foreground_color = theme.get_foreground_color();
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let keybinding_rows = column![
            self.keybinding_row("Submit Input", "submit_input", &self.current_keybindings.submit_input, theme),
            self.keybinding_row("History Up", "history_up", &self.current_keybindings.history_up, theme),
            self.keybinding_row("History Down", "history_down", &self.current_keybindings.history_down, theme),
            self.keybinding_row("Clear Terminal", "clear_terminal", &self.current_keybindings.clear_terminal, theme),
            self.keybinding_row("Toggle Fullscreen", "toggle_fullscreen", &self.current_keybindings.toggle_fullscreen, theme),
            self.keybinding_row("Open Command Palette", "open_command_palette", &self.current_keybindings.open_command_palette, theme),
            self.keybinding_row("Open Preferences", "open_preferences", &self.current_keybindings.open_preferences, theme),
            self.keybinding_row("Open Theme Customizer", "open_theme_customizer", &self.current_keybindings.open_theme_customizer, theme),
            self.keybinding_row("Open Profile Manager", "open_profile_manager", &self.current_keybindings.open_profile_manager, theme),
            self.keybinding_row("Open Workflow Browser", "open_workflow_browser", &self.current_keybindings.open_workflow_browser, theme),
            self.keybinding_row("Open Warp Drive", "open_warp_drive", &self.current_keybindings.open_warp_drive, theme),
        ]
        .spacing(10)
        .width(Length::Fill);

        let controls = row![
            button(text("Save").color(foreground_color))
                .on_press(KeybindingEditorMessage::SaveClicked)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(accent_color)),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: foreground_color,
                }),
            button(text("Cancel").color(foreground_color))
                .on_press(KeybindingEditorMessage::CancelClicked)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(border_color)),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: foreground_color,
                }),
            button(text("Reset to Defaults").color(foreground_color))
                .on_press(KeybindingEditorMessage::ResetToDefaults)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(border_color)),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: foreground_color,
                }),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        container(
            column![
                text("Keybinding Editor").size(24).color(foreground_color),
                Space::with_height(Length::Fixed(20.0)),
                scrollable(keybinding_rows)
                    .width(Length::Fill)
                    .height(Length::FillPortion(1)),
                Space::with_height(Length::Fixed(20.0)),
                controls,
            ]
            .spacing(15)
            .padding(20)
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                color: border_color,
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn keybinding_row<'a>(&self, label: &str, key_name: &str, value: &str, theme: &WarpTheme) -> Element<'a, KeybindingEditorMessage> {
        let foreground_color = theme.get_foreground_color();
        let text_input_bg = theme.get_output_background_color(theme.is_dark_theme());
        let text_input_border = theme.get_border_color();

        row![
            text(label).size(16).color(foreground_color).width(Length::FillPortion(1)),
            text_input("Press new key combination...", value)
                .on_input(move |s| KeybindingEditorMessage::KeybindingChanged(key_name.to_string(), s))
                .padding(8)
                .size(16)
                .width(Length::FillPortion(2))
                .style(iced::widget::text_input::Appearance {
                    background: text_input_bg.into(),
                    border_radius: 4.0.into(),
                    border_width: 1.0,
                    border_color: text_input_border,
                    icon_color: foreground_color,
                }),
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
}
