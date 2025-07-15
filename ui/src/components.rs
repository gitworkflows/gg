//! Reusable UI components for the Warp Terminal.

use iced::{widget::{Container, Text, Button, Column, Row}, Element, Length, Color};

/// A simple card component.
pub struct Card;

impl Card {
    pub fn new<'a, Message>(content: Element<'a, Message>) -> Element<'a, Message> {
        Container::new(content)
            .width(Length::Fill)
            .padding(10)
            .style(iced::theme::Container::Box) // Use a default Iced theme style
            .into()
    }
}

/// A custom button with specific styling.
pub struct PrimaryButton;

impl PrimaryButton {
    pub fn new<'a, Message>(label: &str) -> Button<'a, Message> {
        Button::new(Text::new(label))
            .padding(10)
            .style(iced::theme::Button::Primary) // Use a default Iced theme style
    }
}

/// A text input field.
pub struct TextInput;

impl TextInput {
    pub fn new<'a, Message>(placeholder: &str, value: &str, on_change: impl Fn(String) -> Message + 'a) -> iced::widget::TextInput<'a, Message> {
        iced::widget::text_input(placeholder, value)
            .on_input(on_change)
            .padding(10)
            .size(16)
    }
}

/// A scrollable view.
pub struct Scrollable;

impl Scrollable {
    pub fn new<'a, Message>(content: Element<'a, Message>) -> iced::widget::Scrollable<'a, Message> {
        iced::widget::scrollable(content)
    }
}

/// A column layout.
pub struct VStack;

impl VStack {
    pub fn new<'a, Message>(elements: Vec<Element<'a, Message>>) -> Column<'a, Message> {
        Column::with_children(elements).spacing(10)
    }
}

/// A row layout.
pub struct HStack;

impl HStack {
    pub fn new<'a, Message>(elements: Vec<Element<'a, Message>>) -> Row<'a, Message> {
        Row::with_children(elements).spacing(10)
    }
}
