use iced::{
    widget::{column, row, text, button, pick_list, Space},
    Alignment, Element, Length,
};
use crate::config::theme::WarpTheme;
use crate::config::yaml_theme_manager::ThemeManager;
use log::info;

#[derive(Debug, Clone)]
pub enum YamlThemeUIMessage {
    SelectTheme(String),
    ApplyTheme,
    OpenThemeCustomizer,
}

pub struct YamlThemeUI {
    theme_manager: ThemeManager,
    available_themes: Vec<String>,
    selected_theme_name: String,
}

impl YamlThemeUI {
    pub fn new(theme_manager: ThemeManager) -> Self {
        let available_themes = theme_manager.get_theme_names();
        let selected_theme_name = theme_manager.get_active_theme().name.clone();
        YamlThemeUI {
            theme_manager,
            available_themes,
            selected_theme_name,
        }
    }

    pub fn update(&mut self, message: YamlThemeUIMessage) {
        match message {
            YamlThemeUIMessage::SelectTheme(name) => {
                info!("Selected theme in UI: {}", name);
                self.selected_theme_name = name;
            }
            YamlThemeUIMessage::ApplyTheme => {
                info!("Applying theme: {}", self.selected_theme_name);
                self.theme_manager.set_active_theme(&self.selected_theme_name);
            }
            YamlThemeUIMessage::OpenThemeCustomizer => {
                info!("Opening theme customizer from YAML UI.");
                // This message would typically be handled by the main application loop
                // to switch to the ThemeEditor view.
            }
        }
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<YamlThemeUIMessage> {
        let foreground_color = theme.get_foreground_color();
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        container(
            column![
                text("Theme Selector").size(24).color(foreground_color),
                Space::with_height(Length::Fixed(20.0)),
                row![
                    text("Choose a theme:").size(18).color(foreground_color),
                    pick_list(
                        self.available_themes.clone(),
                        Some(self.selected_theme_name.clone()),
                        YamlThemeUIMessage::SelectTheme,
                    )
                    .width(Length::Fixed(250.0))
                    .padding(10)
                    .text_size(18)
                    .style(iced::theme::PickList::Default), // Use default theme for now
                ]
                .spacing(15)
                .align_items(Alignment::Center),
                Space::with_height(Length::Fixed(30.0)),
                row![
                    button(text("Apply Theme").color(foreground_color))
                        .on_press(YamlThemeUIMessage::ApplyTheme)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(accent_color)),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: foreground_color,
                        }),
                    button(text("Customize Themes").color(foreground_color))
                        .on_press(YamlThemeUIMessage::OpenThemeCustomizer)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(border_color)),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: foreground_color,
                        }),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
            ]
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
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
}
