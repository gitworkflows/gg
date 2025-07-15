use iced::{Element, widget::{column, container, text, row, button}};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::terminal::Message;
use crate::themes::WarpTheme;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub content: BlockContent,
    pub timestamp: DateTime<Utc>,
    pub exit_code: Option<i32>,
    pub execution_time: Option<std::time::Duration>,
}

#[derive(Debug, Clone)]
pub enum BlockContent {
    Command {
        input: String,
        output: String,
        is_running: bool,
    },
    Markdown(String),
    FilePreview {
        path: PathBuf,
        content: String,
        file_type: FileType,
    },
    Image {
        path: PathBuf,
        data: Vec<u8>,
    },
    Error {
        message: String,
        details: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum FileType {
    Text,
    Code(String), // Language
    Binary,
    Image,
}

#[derive(Debug, Clone)]
pub enum BlockMessage {
    Copy(String),
    Edit(Uuid),
    Delete(Uuid),
    Expand(Uuid),
    Collapse(Uuid),
}

impl Block {
    pub fn new_command(id: Uuid, input: String) -> Self {
        Block {
            id,
            content: BlockContent::Command {
                input,
                output: String::new(),
                is_running: true,
            },
            timestamp: Utc::now(),
            exit_code: None,
            execution_time: None,
        }
    }

    pub fn new_markdown(id: Uuid, content: String) -> Self {
        Block {
            id,
            content: BlockContent::Markdown(content),
            timestamp: Utc::now(),
            exit_code: None,
            execution_time: None,
        }
    }

    pub fn append_output(&mut self, output: String) {
        if let BlockContent::Command { output: ref mut current_output, .. } = &mut self.content {
            current_output.push_str(&output);
        }
    }

    pub fn complete_execution(&mut self, exit_code: i32, duration: std::time::Duration) {
        self.exit_code = Some(exit_code);
        self.execution_time = Some(duration);
        
        if let BlockContent::Command { is_running, .. } = &mut self.content {
            *is_running = false;
        }
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<Message> {
        let header = self.create_header(theme);
        let content = self.create_content(theme);
        
        container(
            column![
                header,
                content
            ]
            .spacing(4)
        )
        .padding(8)
        .style(move |_theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(
                    theme.get_block_background_color(theme.is_dark_theme())
                )),
                border: iced::Border {
                    color: theme.get_border_color(),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .width(iced::Length::Fill)
        .into()
    }

    fn create_header(&self, theme: &WarpTheme) -> Element<Message> {
        let timestamp_text = text(
            self.timestamp.format("%H:%M:%S").to_string()
        )
        .size(12)
        .color(theme.get_terminal_color("white", false));

        let status_indicator = match (&self.content, self.exit_code) {
            (BlockContent::Command { is_running: true, .. }, _) => {
                text("●").color(theme.get_terminal_color("yellow", true)) // Yellow for running
            }
            (_, Some(0)) => {
                text("●").color(theme.get_terminal_color("green", true)) // Green for success
            }
            (_, Some(_)) => {
                text("●").color(theme.get_terminal_color("red", true)) // Red for error
            }
            _ => {
                text("●").color(theme.get_terminal_color("white", false)) // Gray for unknown
            }
        };

        row![
            status_indicator,
            timestamp_text,
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn create_content(&self, theme: &WarpTheme) -> Element<Message> {
        match &self.content {
            BlockContent::Command { input, output, is_running } => {
                let input_text = text(format!("$ {}", input))
                    .size(14)
                    .color(theme.get_terminal_color("blue", true));

                let output_text = if output.is_empty() && *is_running {
                    text("Running...").color(theme.get_terminal_color("yellow", false))
                } else {
                    text(output).size(14).color(theme.get_foreground_color())
                };

                column![
                    input_text,
                    output_text
                ]
                .spacing(4)
                .into()
            }
            
            BlockContent::Markdown(content) => {
                text(content).color(theme.get_foreground_color()).into()
            }
            
            BlockContent::FilePreview { path, content, .. } => {
                column![
                    text(format!("File: {}", path.display()))
                        .size(12)
                        .color(theme.get_terminal_color("cyan", false)),
                    text(content).size(14).color(theme.get_foreground_color())
                ]
                .spacing(4)
                .into()
            }
            
            BlockContent::Error { message, details } => {
                let mut col = column![
                    text(message).color(theme.get_terminal_color("red", true))
                ];
                
                if let Some(details) = details {
                    col = col.push(
                        text(details)
                            .size(12)
                            .color(theme.get_terminal_color("white", false))
                    );
                }
                
                col.spacing(4).into()
            }
            
            _ => text("Unsupported block type").color(theme.get_foreground_color()).into(),
        }
    }
}
