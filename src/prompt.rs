use iced::{
    widget::{column, row, text, text_input, Space},
    Alignment, Element, Length,
};
use crate::config::theme::WarpTheme;
use crate::config::PromptSettings;
use crate::input::EditorMessage; // Use the new input module

#[derive(Debug, Clone)]
pub enum PromptMessage {
    InputChanged(String),
    InputSubmitted,
    Editor(EditorMessage),
}

pub struct Prompt {
    input_value: String,
    settings: PromptSettings,
    // Potentially add state for current working directory, git status, etc.
}

impl Prompt {
    pub fn new(settings: PromptSettings) -> Self {
        Prompt {
            input_value: String::new(),
            settings,
        }
    }

    pub fn update(&mut self, message: PromptMessage) -> Option<String> {
        match message {
            PromptMessage::InputChanged(value) => {
                self.input_value = value;
                None
            }
            PromptMessage::InputSubmitted => {
                let submitted_command = self.input_value.clone();
                self.input_value.clear(); // Clear input after submission
                Some(submitted_command)
            }
            PromptMessage::Editor(editor_message) => {
                match editor_message {
                    EditorMessage::InputChanged(value) => {
                        self.input_value = value;
                    }
                    EditorMessage::Submit => {
                        let submitted_command = self.input_value.clone();
                        self.input_value.clear();
                        return Some(submitted_command);
                    }
                    _ => {} // Ignore other editor messages
                }
                None
            }
        }
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<PromptMessage> {
        let foreground_color = theme.get_foreground_color();
        let accent_color = theme.get_accent_color();

        let mut prompt_elements = Vec::new();

        if self.settings.show_user {
            prompt_elements.push(text(format!("{}user", self.settings.user_symbol)).color(foreground_color).size(16));
        }
        if self.settings.show_host {
            prompt_elements.push(text(format!("{}host", self.settings.host_symbol)).color(foreground_color).size(16));
        }
        if self.settings.show_cwd {
            prompt_elements.push(text(format!("{}~", self.settings.cwd_symbol)).color(foreground_color).size(16)); // Placeholder for actual CWD
        }
        if self.settings.show_git_status {
            // Placeholder for git status
            prompt_elements.push(text(format!("{}main*", self.settings.git_symbol)).color(Color::from_rgb(0.0, 0.7, 0.0)).size(16));
        }

        let prompt_line = row(prompt_elements).spacing(5).align_items(Alignment::Center);

        column![
            prompt_line,
            row![
                text(&self.settings.prompt_symbol).color(accent_color).size(18),
                Space::with_width(Length::Fixed(5.0)),
                text_input("", &self.input_value)
                    .on_input(PromptMessage::InputChanged)
                    .on_submit(PromptMessage::InputSubmitted)
                    .padding(5)
                    .size(18)
                    .width(Length::Fill)
                    .style(iced::theme::TextInput::Default), // Use default theme for now
            ]
            .align_items(Alignment::Center)
            .width(Length::Fill),
        ]
        .spacing(5)
        .width(Length::Fill)
        .into()
    }

    pub fn get_input_value(&self) -> &str {
        &self.input_value
    }
}
