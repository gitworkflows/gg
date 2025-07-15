use iced::{
    widget::{text_input, container},
    Element, Length, Command, Theme, Color,
};
use iced::widget::text_input::Appearance;

use crate::themes::WarpTheme;
use crate::prompt::PromptRenderer;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    InputChanged(String),
    Submit,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorHome,
    MoveCursorEnd,
    DeletePreviousChar,
    DeleteNextChar,
    // Add more editor-specific messages as needed
}

pub struct Editor {
    value: String,
    cursor_position: usize,
    font_size: u16,
    font_family: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor_position: 0,
            font_size: 16,
            font_family: "Fira Code".to_string(), // Default font
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn set_value(&mut self, new_value: String) {
        self.value = new_value;
        self.cursor_position = self.value.len();
    }

    pub fn set_font_size(&mut self, size: u16) {
        self.font_size = size;
    }

    pub fn set_font_family(&mut self, family: String) {
        self.font_family = family;
    }

    pub fn update(&mut self, message: EditorMessage) -> Command<EditorMessage> {
        match message {
            EditorMessage::InputChanged(new_value) => {
                self.value = new_value;
                self.cursor_position = self.value.len();
            }
            EditorMessage::Submit => {
                // Handled by the parent (Terminal)
            }
            EditorMessage::MoveCursorLeft => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            EditorMessage::MoveCursorRight => {
                if self.cursor_position < self.value.len() {
                    self.cursor_position += 1;
                }
            }
            EditorMessage::MoveCursorHome => {
                self.cursor_position = 0;
            }
            EditorMessage::MoveCursorEnd => {
                self.cursor_position = self.value.len();
            }
            EditorMessage::DeletePreviousChar => {
                if self.cursor_position > 0 {
                    self.value.remove(self.cursor_position - 1);
                    self.cursor_position -= 1;
                }
            }
            EditorMessage::DeleteNextChar => {
                if self.cursor_position < self.value.len() {
                    self.value.remove(self.cursor_position);
                }
            }
        }
        Command::none()
    }

    pub fn view<'a>(&'a self, theme: &WarpTheme, prompt_renderer: &'a PromptRenderer) -> Element<'a, EditorMessage> {
        let prompt_text = prompt_renderer.render_prompt_text();

        let input = text_input("", &self.value)
            .on_input(EditorMessage::InputChanged)
            .on_submit(EditorMessage::Submit)
            .padding(8)
            .size(self.font_size)
            .font(iced::Font::with_name(&self.font_family))
            .style(move |iced_theme: &Theme, status| {
                Appearance {
                    background: iced::Background::Color(theme.get_block_background_color(theme.is_dark_theme())),
                    border: iced::Border {
                        color: theme.get_border_color(),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    icon_color: theme.get_foreground_color(),
                    placeholder_color: theme.get_terminal_color("white", false),
                    value_color: theme.get_foreground_color(),
                    selection_color: theme.get_accent_color(),
                }
            });

        container(
            iced::widget::row![
                prompt_text,
                input,
            ]
            .align_items(iced::Alignment::Center)
            .spacing(5)
        )
        .width(Length::Fill)
        .padding(4)
        .into()
    }
}
