//! Custom UI widgets for the Warp Terminal.

use iced::{widget::{Container, Text, Button, Checkbox, Radio, Slider, TextInput, Column, Row}, Element, Length, Color};
use iced::widget::button::Appearance;
use iced::widget::text_input;
use iced::widget::slider;
use iced::widget::checkbox;
use iced::widget::radio;
use iced::Alignment;

/// A custom styled button.
pub fn custom_button<'a, Message>(content: impl Into<Element<'a, Message>>, on_press: Message, background_color: Color, text_color: Color) -> Button<'a, Message> {
    Button::new(content)
        .on_press(on_press)
        .padding(10)
        .style(iced::theme::Button::Custom(Box::new(CustomButtonStyle {
            background: background_color,
            text_color,
        })))
}

struct CustomButtonStyle {
    background: Color,
    text_color: Color,
}

impl iced::widget::button::StyleSheet for CustomButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Some(self.background.into()),
            border_radius: 5.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: self.text_color,
            shadow_offset: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);
        Appearance {
            background: Some(Color {
                a: active.background.map_or(1.0, |bg| bg.a() * 0.9), // Slightly darker on hover
                ..self.background
            }.into()),
            ..active
        }
    }
}

/// A custom styled text input.
pub fn custom_text_input<'a, Message>(placeholder: &str, value: &str, on_input: impl Fn(String) -> Message + 'a, background_color: Color, text_color: Color, border_color: Color) -> TextInput<'a, Message> {
    text_input(placeholder, value)
        .on_input(on_input)
        .padding(10)
        .size(16)
        .style(iced::theme::TextInput::Custom(Box::new(CustomTextInputStyle {
            background: background_color,
            text_color,
            border_color,
        })))
}

struct CustomTextInputStyle {
    background: Color,
    text_color: Color,
    border_color: Color,
}

impl iced::widget::text_input::StyleSheet for CustomTextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: self.background.into(),
            border_radius: 5.0.into(),
            border_width: 1.0,
            border_color: self.border_color,
            icon_color: self.text_color,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        text_input::Appearance {
            border_color: Color::from_rgb(0.0, 0.5, 1.0), // Accent color on focus
            ..active
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.text_color.scale_alpha(0.7)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.text_color
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb(0.0, 0.5, 1.0).scale_alpha(0.3)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let active = self.active(style);
        text_input::Appearance {
            background: Color { a: active.background.map_or(1.0, |bg| bg.a() * 0.5), ..self.background }.into(),
            border_color: Color { a: active.border_color.a * 0.5, ..self.border_color },
            ..active
        }
    }
}

/// A custom styled checkbox.
pub fn custom_checkbox<'a, Message>(label: &str, is_checked: bool, on_toggle: impl Fn(bool) -> Message + 'a, active_color: Color, label_color: Color) -> Checkbox<'a, Message> {
    checkbox(label, is_checked, on_toggle)
        .text_color(label_color)
        .style(iced::theme::Checkbox::Custom(Box::new(CustomCheckboxStyle {
            active_color,
            label_color,
        })))
}

struct CustomCheckboxStyle {
    active_color: Color,
    label_color: Color,
}

impl iced::widget::checkbox::StyleSheet for CustomCheckboxStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: if is_checked { self.active_color.into() } else { Color::TRANSPARENT.into() },
            border_radius: 3.0.into(),
            border_width: 1.0,
            border_color: self.active_color,
            icon_color: self.label_color,
            text_color: Some(self.label_color),
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let active = self.active(style, is_checked);
        checkbox::Appearance {
            background: if is_checked {
                Color { a: active.background.map_or(1.0, |bg| bg.a() * 0.9), ..self.active_color }.into()
            } else {
                Color { a: 0.1, ..self.active_color }.into() // Light background on hover for unchecked
            },
            ..active
        }
    }
}

/// A custom styled radio button.
pub fn custom_radio<'a, T, Message>(label: &str, value: T, selected: Option<T>, on_click: impl Fn(T) -> Message + 'a, active_color: Color, label_color: Color) -> Radio<'a, T, Message>
where
    T: Copy + Eq + 'static,
{
    radio(label, value, selected, on_click)
        .text_color(label_color)
        .style(iced::theme::Radio::Custom(Box::new(CustomRadioStyle {
            active_color,
            label_color,
        })))
}

struct CustomRadioStyle {
    active_color: Color,
    label_color: Color,
}

impl iced::widget::radio::StyleSheet for CustomRadioStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style, is_selected: bool) -> radio::Appearance {
        radio::Appearance {
            background: if is_selected { self.active_color.into() } else { Color::TRANSPARENT.into() },
            border_radius: 10.0.into(), // Make it circular
            border_width: 1.0,
            border_color: self.active_color,
            dot_color: self.label_color,
            text_color: Some(self.label_color),
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        let active = self.active(style, is_selected);
        radio::Appearance {
            background: if is_selected {
                Color { a: active.background.map_or(1.0, |bg| bg.a() * 0.9), ..self.active_color }.into()
            } else {
                Color { a: 0.1, ..self.active_color }.into() // Light background on hover for unselected
            },
            ..active
        }
    }
}

/// A custom styled slider.
pub fn custom_slider<'a, T, Message>(range: std::ops::RangeInclusive<T>, value: T, on_change: impl Fn(T) -> Message + 'a, rail_color: Color, handle_color: Color) -> Slider<'a, T, Message>
where
    T: Copy + Into<f32> + std::fmt::Debug,
{
    slider(range, value, on_change)
        .step(0.1) // Example step
        .style(iced::theme::Slider::Custom(Box::new(CustomSliderStyle {
            rail_color,
            handle_color,
        })))
}

struct CustomSliderStyle {
    rail_color: Color,
    handle_color: Color,
}

impl iced::widget::slider::StyleSheet for CustomSliderStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail: slider::Rail {
                colors: (self.rail_color, self.rail_color.scale_alpha(0.5)),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 8.0 },
                color: self.handle_color,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                shadow_offset: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        let active = self.active(style);
        slider::Appearance {
            handle: slider::Handle {
                color: Color { a: active.handle.color.a * 0.9, ..self.handle_color }, // Slightly darker on hover
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        self.hovered(style)
    }
}
