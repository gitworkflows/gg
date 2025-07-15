use iced::{Element, widget::{text_input, column, container}};
use std::collections::VecDeque;

use crate::terminal::Message;
use crate::fuzzy::Suggestion;

pub struct EnhancedTextInput {
    value: String,
    suggestions: Vec<Suggestion>,
    selected_suggestion: Option<usize>,
    show_suggestions: bool,
}

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ValueChanged(String),
    SuggestionSelected(usize),
    SuggestionsToggled(bool),
}

impl EnhancedTextInput {
    pub fn new() -> Self {
        EnhancedTextInput {
            value: String::new(),
            suggestions: Vec::new(),
            selected_suggestion: None,
            show_suggestions: false,
        }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<Suggestion>) -> Self {
        self.suggestions = suggestions;
        self.show_suggestions = !suggestions.is_empty();
        self
    }

    pub fn view(&self) -> Element<Message> {
        let input = text_input("Enter command...", &self.value)
            .padding(8)
            .size(16);

        if self.show_suggestions && !self.suggestions.is_empty() {
            let suggestions_view = self.create_suggestions_view();
            
            column![
                input,
                suggestions_view
            ]
            .spacing(2)
            .into()
        } else {
            input.into()
        }
    }

    fn create_suggestions_view(&self) -> Element<Message> {
        let suggestions: Vec<Element<Message>> = self.suggestions
            .iter()
            .enumerate()
            .take(5) // Limit to 5 suggestions
            .map(|(index, suggestion)| {
                let is_selected = self.selected_suggestion == Some(index);
                
                container(
                    iced::widget::text(&suggestion.text)
                        .size(14)
                )
                .padding(4)
                .style(move |theme: &iced::Theme| {
                    container::Appearance {
                        background: if is_selected {
                            Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.4, 0.8)))
                        } else {
                            Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.1)))
                        },
                        ..Default::default()
                    }
                })
                .width(iced::Length::Fill)
                .into()
            })
            .collect();

        container(
            column(suggestions)
                .spacing(1)
        )
        .style(|_theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.3, 0.3, 0.3),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .padding(4)
        .width(iced::Length::Fill)
        .into()
    }
}
