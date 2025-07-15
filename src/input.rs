use iced::{
    widget::{text_input, container, column, Text},
    Element, Length, Command,
};
use crate::terminal::Message;
use crate::themes::WarpTheme;
use crate::prompt::PromptRenderer;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    InputChanged(String),
    Submit,
    HistoryUp,
    HistoryDown,
    // Add more editor-specific messages like CursorMoved, SelectText, etc.
}

pub struct Editor {
    input_value: String,
    history: Vec<String>,
    history_index: Option<usize>,
    font_size: u16,
    font_family: String,
    input_handler: InputHandler,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            input_value: String::new(),
            history: Vec::new(),
            history_index: None,
            font_size: 16,
            font_family: "Fira Code".to_string(),
            input_handler: InputHandler::new(),
        }
    }

    pub fn update(&mut self, message: EditorMessage) -> Command<Message> {
        match message {
            EditorMessage::InputChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            EditorMessage::Submit => {
                let submitted_value = self.input_value.clone();
                if !submitted_value.trim().is_empty() {
                    self.history.push(submitted_value.clone());
                    self.input_value.clear();
                    self.history_index = None;
                }
                Command::perform(async {}, move |_| Message::InputSubmitted(submitted_value))
            }
            EditorMessage::HistoryUp => {
                if let Some(idx) = self.history_index {
                    if idx > 0 {
                        self.history_index = Some(idx - 1);
                        self.input_value = self.history[idx - 1].clone();
                    }
                } else if !self.history.is_empty() {
                    self.history_index = Some(self.history.len() - 1);
                    self.input_value = self.history[self.history.len() - 1].clone();
                }
                Command::none()
            }
            EditorMessage::HistoryDown => {
                if let Some(idx) = self.history_index {
                    if idx < self.history.len() - 1 {
                        self.history_index = Some(idx + 1);
                        self.input_value = self.history[idx + 1].clone();
                    } else {
                        self.history_index = None;
                        self.input_value.clear();
                    }
                }
                Command::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &WarpTheme, prompt_renderer: &'a PromptRenderer) -> Element<'a, EditorMessage> {
        let input_style = iced::widget::text_input::Appearance {
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
        };

        let prompt_view = prompt_renderer.render_prompt(theme);

        container(
            column![
                prompt_view,
                text_input("", &self.input_value)
                    .on_input(EditorMessage::InputChanged)
                    .on_submit(EditorMessage::Submit)
                    .padding(8)
                    .size(self.font_size)
                    .font(iced::Font::with_name(&self.font_family))
                    .style(input_style),
            ]
            .spacing(4)
            .padding(8)
        )
        .width(Length::Fill)
        .into()
    }

    pub fn get_input_value(&self) -> &str {
        &self.input_value
    }

    pub fn set_input_value(&mut self, value: String) {
        self.input_value = value;
        self.history_index = None; // Reset history index when input is manually set
    }

    pub fn clear(&mut self) {
        self.input_value.clear();
        self.history_index = None;
        self.input_handler.clear_input();
    }

    pub fn set_font_size(&mut self, size: u16) {
        self.font_size = size;
    }

    pub fn set_font_family(&mut self, family: String) {
        self.font_family = family;
    }

    pub fn handle_key_event(&mut self, key: char) {
        self.input_handler.handle_key_event(key);
        self.input_value = self.input_handler.current_input.clone();
    }
}

pub struct InputHandler {
    pub current_input: String,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            current_input: String::new(),
        }
    }

    pub fn handle_key_event(&mut self, key: char) {
        self.current_input.push(key);
        println!("Input: {}", self.current_input);
    }

    pub fn clear_input(&mut self) {
        self.current_input.clear();
    }
}
