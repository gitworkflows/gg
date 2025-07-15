use iced::{
    widget::{column, row, text, button, text_input, pick_list, Space},
    Alignment, Element, Length, Color,
};
use crate::config::theme::WarpTheme;
use crate::config::yaml_theme_manager::ThemeManager;
use log::{info, error};

#[derive(Debug, Clone)]
pub enum ThemeEditorMessage {
    SelectTheme(String),
    EditColor(String, String), // (color_name, new_hex_value)
    SaveTheme,
    CancelChanges,
    ImportThemeClicked,
    ExportThemeClicked,
}

pub struct ThemeEditor {
    theme_manager: ThemeManager,
    available_themes: Vec<String>,
    selected_theme_name: String,
    current_theme_config: WarpTheme, // The theme being edited
    original_theme_config: WarpTheme, // To revert changes
}

impl ThemeEditor {
    pub fn new(theme_manager: ThemeManager) -> Self {
        let available_themes = theme_manager.get_theme_names();
        let selected_theme_name = theme_manager.get_active_theme().name.clone();
        let current_theme_config = theme_manager.get_active_theme();
        let original_theme_config = current_theme_config.clone();

        ThemeEditor {
            theme_manager,
            available_themes,
            selected_theme_name,
            current_theme_config,
            original_theme_config,
        }
    }

    pub fn update(&mut self, message: ThemeEditorMessage) {
        match message {
            ThemeEditorMessage::SelectTheme(name) => {
                info!("Selected theme: {}", name);
                if let Some(theme) = self.theme_manager.get_theme_by_name(&name) {
                    self.selected_theme_name = name;
                    self.current_theme_config = theme.clone();
                    self.original_theme_config = theme;
                } else {
                    error!("Selected theme '{}' not found.", name);
                }
            }
            ThemeEditorMessage::EditColor(color_name, new_hex) => {
                info!("Editing color {} to {}", color_name, new_hex);
                match color_name.as_str() {
                    "background" => self.current_theme_config.background = new_hex,
                    "foreground" => self.current_theme_config.foreground = new_hex,
                    "accent" => self.current_theme_config.accent = new_hex,
                    "border" => self.current_theme_config.border = new_hex,
                    "command_background" => self.current_theme_config.command_background = new_hex,
                    "output_background" => self.current_theme_config.output_background = new_hex,
                    _ => info!("Unknown color property: {}", color_name),
                }
            }
            ThemeEditorMessage::SaveTheme => {
                info!("Saving theme changes for: {}", self.current_theme_config.name);
                // In a real app, you'd need to convert WarpTheme back to YamlTheme
                // and save it via ThemeManager. For now, we'll just update the active theme.
                self.theme_manager.set_active_theme(&self.current_theme_config.name);
                // A more robust save would involve updating the YamlTheme in ThemeManager's internal map
                // and then persisting it to disk.
                self.original_theme_config = self.current_theme_config.clone();
            }
            ThemeEditorMessage::CancelChanges => {
                info!("Cancelling theme changes.");
                self.current_theme_config = self.original_theme_config.clone();
                self.selected_theme_name = self.original_theme_config.name.clone();
            }
            ThemeEditorMessage::ImportThemeClicked => {
                info!("Import theme clicked (not yet implemented file dialog).");
                // This would trigger a file dialog to select a .yaml theme file
            }
            ThemeEditorMessage::ExportThemeClicked => {
                info!("Export theme clicked (not yet implemented file dialog).");
                // This would trigger a file dialog to select a directory to save the .yaml theme file
            }
        }
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<ThemeEditorMessage> {
        let foreground_color = theme.get_foreground_color();
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let color_input_style = iced::widget::text_input::Appearance {
            background: theme.get_output_background_color(theme.is_dark_theme()).into(),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: theme.get_border_color(),
            icon_color: foreground_color,
        };

        let theme_selector = row![
            text("Select Theme:").size(18).color(foreground_color),
            pick_list(
                self.available_themes.clone(),
                Some(self.selected_theme_name.clone()),
                ThemeEditorMessage::SelectTheme,
            )
            .width(Length::Fixed(200.0))
            .padding(8)
            .text_size(16)
            .style(iced::theme::PickList::Default), // Use default theme for now
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let color_editor = column![
            self.color_row("Background", "background", &self.current_theme_config.background, foreground_color, color_input_style.clone()),
            self.color_row("Foreground", "foreground", &self.current_theme_config.foreground, foreground_color, color_input_style.clone()),
            self.color_row("Accent", "accent", &self.current_theme_config.accent, foreground_color, color_input_style.clone()),
            self.color_row("Border", "border", &self.current_theme_config.border, foreground_color, color_input_style.clone()),
            self.color_row("Command Background", "command_background", &self.current_theme_config.command_background, foreground_color, color_input_style.clone()),
            self.color_row("Output Background", "output_background", &self.current_theme_config.output_background, foreground_color, color_input_style.clone()),
        ]
        .spacing(10)
        .width(Length::Fill);

        let controls = row![
            button(text("Save").color(foreground_color))
                .on_press(ThemeEditorMessage::SaveTheme)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(accent_color)),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: foreground_color,
                }),
            button(text("Cancel").color(foreground_color))
                .on_press(ThemeEditorMessage::CancelChanges)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(border_color)),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: foreground_color,
                }),
            button(text("Import").color(foreground_color))
                .on_press(ThemeEditorMessage::ImportThemeClicked)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(border_color)),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: foreground_color,
                }),
            button(text("Export").color(foreground_color))
                .on_press(ThemeEditorMessage::ExportThemeClicked)
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
                text("Theme Customizer").size(24).color(foreground_color),
                Space::with_height(Length::Fixed(20.0)),
                theme_selector,
                Space::with_height(Length::Fixed(20.0)),
                color_editor,
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

    fn color_row<'a>(&self, label: &str, color_name: &str, hex_value: &str, text_color: Color, input_style: iced::widget::text_input::Appearance) -> Element<'a, ThemeEditorMessage> {
        row![
            text(label).size(16).color(text_color).width(Length::FillPortion(1)),
            text_input("#RRGGBB", hex_value)
                .on_input(move |s| ThemeEditorMessage::EditColor(color_name.to_string(), s))
                .padding(8)
                .size(16)
                .width(Length::FillPortion(2))
                .style(input_style),
            // A small color preview box
            container(Space::with_width(Length::Fixed(20.0)))
                .width(Length::Fixed(30.0))
                .height(Length::Fixed(30.0))
                .style(move |_theme: &iced::Theme| container::Appearance {
                    background: crate::config::theme::parse_hex_color(hex_value).map(|c| c.into()),
                    border: iced::Border {
                        color: text_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
}
