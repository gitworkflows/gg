use iced::{Element, widget::{
    button, column, container, text, text_input, row, scrollable, 
    color_picker, pick_list, slider
}};
use iced::{Alignment, Length, Color};

use crate::terminal::Message;
use crate::themes::{WarpTheme, ColorPalette, TerminalColors};
use crate::config::CustomThemeOverrides;

#[derive(Debug, Clone)]
pub enum ThemeCustomizerMessage {
    AccentColorChanged(Color),
    BackgroundColorChanged(Color),
    ForegroundColorChanged(Color),
    TerminalColorChanged(String, bool, Color), // color_name, is_bright, color
    BaseThemeSelected(String),
    SaveCustomTheme(String),
    ResetToBase,
    PreviewToggled(bool),
    ExportTheme,
    ImportTheme(String),
    ToggleVisibility,
    SelectBaseTheme(String),
    UpdateBackgroundColor(f32),
    UpdateForegroundColor(f32),
    UpdateAccentColor(f32),
    UpdateBorderColor(f32),
    UpdateBlockBackgroundColor(f32),
    ThemeNameChanged(String),
}

pub struct ThemeCustomizer {
    current_theme: WarpTheme,
    base_theme_name: String,
    available_base_themes: Vec<String>,
    custom_theme_name: String,
    is_visible: bool,
    preview_enabled: bool,
    color_picker_state: ColorPickerState,
    theme_name_input: String,
}

#[derive(Debug, Clone)]
struct ColorPickerState {
    active_picker: Option<ColorPickerType>,
}

#[derive(Debug, Clone, PartialEq)]
enum ColorPickerType {
    Accent,
    Background,
    Foreground,
    Terminal(String, bool), // color_name, is_bright
}

impl ThemeCustomizer {
    pub fn new(base_themes: Vec<String>, current_theme: WarpTheme) -> Self {
        ThemeCustomizer {
            current_theme,
            base_theme_name: "default".to_string(),
            available_base_themes: base_themes,
            custom_theme_name: "My Custom Theme".to_string(),
            is_visible: false,
            preview_enabled: true,
            color_picker_state: ColorPickerState {
                active_picker: None,
            },
            theme_name_input: format!("{}_custom", current_theme.name),
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn get_current_theme(&self) -> &WarpTheme {
        &self.current_theme
    }

    pub fn update(&mut self, message: ThemeCustomizerMessage) -> Option<Message> {
        match message {
            ThemeCustomizerMessage::AccentColorChanged(color) => {
                self.current_theme.accent = color_to_hex(color);
                if self.preview_enabled {
                    Some(Message::ThemePreviewUpdated(self.current_theme.clone()))
                } else {
                    None
                }
            }
            ThemeCustomizerMessage::BackgroundColorChanged(color) => {
                self.current_theme.background = color_to_hex(color);
                if self.preview_enabled {
                    Some(Message::ThemePreviewUpdated(self.current_theme.clone()))
                } else {
                    None
                }
            }
            ThemeCustomizerMessage::ForegroundColorChanged(color) => {
                self.current_theme.foreground = color_to_hex(color);
                if self.preview_enabled {
                    Some(Message::ThemePreviewUpdated(self.current_theme.clone()))
                } else {
                    None
                }
            }
            ThemeCustomizerMessage::TerminalColorChanged(color_name, is_bright, color) => {
                let hex_color = color_to_hex(color);
                let palette = if is_bright {
                    &mut self.current_theme.terminal_colors.bright
                } else {
                    &mut self.current_theme.terminal_colors.normal
                };

                match color_name.as_str() {
                    "black" => palette.black = hex_color,
                    "red" => palette.red = hex_color,
                    "green" => palette.green = hex_color,
                    "yellow" => palette.yellow = hex_color,
                    "blue" => palette.blue = hex_color,
                    "magenta" => palette.magenta = hex_color,
                    "cyan" => palette.cyan = hex_color,
                    "white" => palette.white = hex_color,
                    _ => {}
                }

                if self.preview_enabled {
                    Some(Message::ThemePreviewUpdated(self.current_theme.clone()))
                } else {
                    None
                }
            }
            ThemeCustomizerMessage::BaseThemeSelected(theme_name) => {
                self.base_theme_name = theme_name.clone();
                Some(Message::LoadBaseThemeForCustomization(theme_name))
            }
            ThemeCustomizerMessage::SaveCustomTheme(name) => {
                self.custom_theme_name = name.clone();
                Some(Message::SaveCustomTheme(name, self.current_theme.clone()))
            }
            ThemeCustomizerMessage::ResetToBase => {
                Some(Message::LoadBaseThemeForCustomization(self.base_theme_name.clone()))
            }
            ThemeCustomizerMessage::PreviewToggled(enabled) => {
                self.preview_enabled = enabled;
                if enabled {
                    Some(Message::ThemePreviewUpdated(self.current_theme.clone()))
                } else {
                    Some(Message::ThemePreviewDisabled)
                }
            }
            ThemeCustomizerMessage::ExportTheme => {
                Some(Message::ExportTheme(self.current_theme.clone()))
            }
            ThemeCustomizerMessage::ImportTheme(yaml_content) => {
                Some(Message::ImportTheme(yaml_content))
            }
            ThemeCustomizerMessage::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                None
            }
            ThemeCustomizerMessage::SelectBaseTheme(name) => {
                self.current_theme = WarpTheme::from_name(&name);
                self.theme_name_input = format!("{}_custom", name);
                None
            }
            ThemeCustomizerMessage::UpdateBackgroundColor(value) => {
                self.current_theme.colors.insert("background".to_string(), [value, value, value]);
                None
            }
            ThemeCustomizerMessage::UpdateForegroundColor(value) => {
                self.current_theme.colors.insert("foreground".to_string(), [value, value, value]);
                None
            }
            ThemeCustomizerMessage::UpdateAccentColor(value) => {
                self.current_theme.accent_color = [value, value, value];
                None
            }
            ThemeCustomizerMessage::UpdateBorderColor(value) => {
                self.current_theme.border_color = [value, value, value];
                None
            }
            ThemeCustomizerMessage::UpdateBlockBackgroundColor(value) => {
                self.current_theme.colors.insert("block_background_dark".to_string(), [value, value, value]);
                self.current_theme.colors.insert("block_background_light".to_string(), [1.0 - value, 1.0 - value, 1.0 - value]); // Invert for light
                None
            }
            ThemeCustomizerMessage::ThemeNameChanged(name) => {
                self.theme_name_input = name;
                None
            }
        }
    }

    pub fn set_theme(&mut self, theme: WarpTheme) {
        self.current_theme = theme;
    }

    pub fn view(&self) -> Element<ThemeCustomizerMessage> {
        if !self.is_visible {
            return container(text("")).into();
        }

        let current_bg = self.current_theme.colors.get("background").unwrap_or(&[0.0, 0.0, 0.0])[0];
        let current_fg = self.current_theme.colors.get("foreground").unwrap_or(&[0.0, 0.0, 0.0])[0];
        let current_accent = self.current_theme.accent_color[0];
        let current_border = self.current_theme.border_color[0];
        let current_block_bg = self.current_theme.colors.get("block_background_dark").unwrap_or(&[0.0, 0.0, 0.0])[0];

        let header = row![
            text("Theme Customizer").size(20),
            button("×")
                .on_press(ThemeCustomizerMessage::PreviewToggled(false))
        ]
        .align_items(Alignment::Center)
        .spacing(8);

        let base_theme_selector = row![
            text("Base Theme:").width(Length::Fixed(100.0)),
            pick_list(
                &self.available_base_themes[..],
                Some(self.base_theme_name.clone()),
                ThemeCustomizerMessage::BaseThemeSelected
            )
        ]
        .align_items(Alignment::Center)
        .spacing(8);

        let main_colors = self.create_main_colors_section();
        let terminal_colors = self.create_terminal_colors_section();
        let actions = self.create_actions_section();

        container(
            column![
                header,
                base_theme_selector,
                scrollable(
                    column![
                        main_colors,
                        terminal_colors,
                    ]
                    .spacing(16)
                ).height(Length::Fixed(400.0)),
                row![
                    text("Background Color:"),
                    slider(0.0..=1.0, current_bg, ThemeCustomizerMessage::UpdateBackgroundColor)
                        .width(Length::Fill),
                    text(format!("{:.2}", current_bg)),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                row![
                    text("Foreground Color:"),
                    slider(0.0..=1.0, current_fg, ThemeCustomizerMessage::UpdateForegroundColor)
                        .width(Length::Fill),
                    text(format!("{:.2}", current_fg)),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                row![
                    text("Accent Color:"),
                    slider(0.0..=1.0, current_accent, ThemeCustomizerMessage::UpdateAccentColor)
                        .width(Length::Fill),
                    text(format!("{:.2}", current_accent)),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                row![
                    text("Border Color:"),
                    slider(0.0..=1.0, current_border, ThemeCustomizerMessage::UpdateBorderColor)
                        .width(Length::Fill),
                    text(format!("{:.2}", current_border)),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                row![
                    text("Block Background:"),
                    slider(0.0..=1.0, current_block_bg, ThemeCustomizerMessage::UpdateBlockBackgroundColor)
                        .width(Length::Fill),
                    text(format!("{:.2}", current_block_bg)),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                actions,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(Length::Fixed(500.0))
        .height(Length::Fixed(600.0))
        .style(move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(self.current_theme.get_block_background_color(self.current_theme.is_dark))),
            border_radius: 8.0.into(),
            border_width: 2.0,
            border_color: self.current_theme.get_border_color(),
            ..Default::default()
        })
        .into()
    }

    fn create_main_colors_section(&self) -> Element<ThemeCustomizerMessage> {
        container(
            column![
                text("Main Colors").size(16),
                self.create_color_row("Accent", &self.current_theme.accent, ThemeCustomizerMessage::AccentColorChanged),
                self.create_color_row("Background", &self.current_theme.background, ThemeCustomizerMessage::BackgroundColorChanged),
                self.create_color_row("Foreground", &self.current_theme.foreground, ThemeCustomizerMessage::ForegroundColorChanged),
            ]
            .spacing(8)
        )
        .padding(8)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_terminal_colors_section(&self) -> Element<ThemeCustomizerMessage> {
        let normal_colors = self.create_color_palette_section("Normal Colors", &self.current_theme.terminal_colors.normal, false);
        let bright_colors = self.create_color_palette_section("Bright Colors", &self.current_theme.terminal_colors.bright, true);

        container(
            column![
                text("Terminal Colors").size(16),
                normal_colors,
                bright_colors,
            ]
            .spacing(8)
        )
        .padding(8)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_color_palette_section(&self, title: &str, palette: &ColorPalette, is_bright: bool) -> Element<ThemeCustomizerMessage> {
        column![
            text(title).size(14),
            row![
                self.create_terminal_color_button("Black", &palette.black, "black", is_bright),
                self.create_terminal_color_button("Red", &palette.red, "red", is_bright),
                self.create_terminal_color_button("Green", &palette.green, "green", is_bright),
                self.create_terminal_color_button("Yellow", &palette.yellow, "yellow", is_bright),
            ].spacing(4),
            row![
                self.create_terminal_color_button("Blue", &palette.blue, "blue", is_bright),
                self.create_terminal_color_button("Magenta", &palette.magenta, "magenta", is_bright),
                self.create_terminal_color_button("Cyan", &palette.cyan, "cyan", is_bright),
                self.create_terminal_color_button("White", &palette.white, "white", is_bright),
            ].spacing(4),
        ]
        .spacing(4)
        .into()
    }

    fn create_color_row(&self, label: &str, hex_color: &str, message: fn(Color) -> ThemeCustomizerMessage) -> Element<ThemeCustomizerMessage> {
        let color = hex_to_color(hex_color);
        
        row![
            text(label).width(Length::Fixed(80.0)),
            button("")
                .width(Length::Fixed(30.0))
                .height(Length::Fixed(30.0))
                .style(move |theme: &iced::Theme, status| {
                    button::Appearance {
                        background: Some(iced::Background::Color(color)),
                        border: iced::Border {
                            color: iced::Color::WHITE,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                }),
            text_input("", hex_color)
                .on_input(move |hex| {
                    if let Ok(color) = parse_hex_color(&hex) {
                        message(color)
                    } else {
                        message(Color::BLACK) // Fallback
                    }
                })
                .width(Length::Fixed(100.0)),
        ]
        .align_items(Alignment::Center)
        .spacing(8)
        .into()
    }

    fn create_terminal_color_button(&self, label: &str, hex_color: &str, color_name: &str, is_bright: bool) -> Element<ThemeCustomizerMessage> {
        let color = hex_to_color(hex_color);
        let color_name = color_name.to_string();
        
        column![
            text(label).size(10),
            button("")
                .width(Length::Fixed(25.0))
                .height(Length::Fixed(25.0))
                .style(move |theme: &iced::Theme, status| {
                    button::Appearance {
                        background: Some(iced::Background::Color(color)),
                        border: iced::Border {
                            color: iced::Color::WHITE,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    }
                })
        ]
        .align_items(Alignment::Center)
        .spacing(2)
        .into()
    }

    fn create_actions_section(&self) -> Element<ThemeCustomizerMessage> {
        row![
            text_input("Theme name", &self.theme_name_input)
                .width(Length::Fixed(150.0)),
            button("Save")
                .on_press(ThemeCustomizerMessage::SaveTheme),
            button("Reset")
                .on_press(ThemeCustomizerMessage::ResetToBase),
            button("Export")
                .on_press(ThemeCustomizerMessage::ExportTheme),
        ]
        .spacing(8)
        .align_items(Alignment::Center)
        .into()
    }
}

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Color::BLACK;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;

    Color::from_rgb(r, g, b)
}

fn color_to_hex(color: Color) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8
    )
}

fn parse_hex_color(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Err("Invalid hex color format".into());
    }

    let r = u8::from_str_radix(&hex[0..2], 16)? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16)? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16)? as f32 / 255.0;

    Ok(Color::from_rgb(r, g, b))
}
