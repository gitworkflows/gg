use iced::{
    widget::{column, container, text, Space},
    Alignment, Element, Length,
};

// Re-export the `WarpTheme` from the `config` module
use crate::config::theme::WarpTheme;

/// Represents a single block of output in the terminal.
#[derive(Debug, Clone)]
pub struct Block {
    id: usize,
    content: String,
    is_command: bool,
}

impl Block {
    pub fn new(id: usize, content: String, is_command: bool) -> Self {
        Block {
            id,
            content,
            is_command,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<crate::terminal::Message> {
        let background_color = if self.is_command {
            theme.get_command_background_color(theme.is_dark_theme())
        } else {
            theme.get_output_background_color(theme.is_dark_theme())
        };
        let foreground_color = theme.get_foreground_color();

        container(
            column![
                text(self.content.clone())
                    .size(16)
                    .color(foreground_color)
            ]
            .width(Length::Fill)
            .align_items(Alignment::Start)
            .spacing(5),
        )
        .width(Length::Fill)
        .padding(10)
        .style(move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                color: theme.get_border_color(),
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}
