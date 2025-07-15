use iced::{Element, widget::{column, container, text, row, button}};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::terminal::Message;

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

    pub fn view(&self) -> Element<Message> {
        let header = self.create_header();
        let content = self.create_content();
        
        container(
            column![
                header,
                content
            ]
            .spacing(4)
        )
        .padding(8)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(
                    if matches!(theme, iced::Theme::Dark) {
                        iced::Color::from_rgb(0.1, 0.1, 0.1)
                    } else {
                        iced::Color::from_rgb(0.95, 0.95, 0.95)
                    }
                )),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.3, 0.3, 0.3),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .width(iced::Length::Fill)
        .into()
    }

    fn create_header(&self) -> Element<Message> {
        let timestamp_text = text(
            self.timestamp.format("%H:%M:%S").to_string()
        )
        .size(12)
        .color(iced::Color::from_rgb(0.6, 0.6, 0.6));

        let status_indicator = match (&self.content, self.exit_code) {
            (BlockContent::Command { is_running: true, .. }, _) => {
                text("●").color(iced::Color::from_rgb(1.0, 0.8, 0.0)) // Yellow for running
            }
            (_, Some(0)) => {
                text("●").color(iced::Color::from_rgb(0.0, 0.8, 0.0)) // Green for success
            }
            (_, Some(_)) => {
                text("●").color(iced::Color::from_rgb(0.8, 0.0, 0.0)) // Red for error
            }
            _ => {
                text("●").color(iced::Color::from_rgb(0.5, 0.5, 0.5)) // Gray for unknown
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

    fn create_content(&self) -> Element<Message> {
        match &self.content {
            BlockContent::Command { input, output, is_running } => {
                let input_text = text(format!("$ {}", input))
                    .size(14)
                    .color(iced::Color::from_rgb(0.8, 0.8, 1.0));

                let output_text = if output.is_empty() && *is_running {
                    text("Running...").color(iced::Color::from_rgb(0.6, 0.6, 0.6))
                } else {
                    text(output).size(14)
                };

                column![
                    input_text,
                    output_text
                ]
                .spacing(4)
                .into()
            }
            
            BlockContent::Markdown(content) => {
                // TODO: Implement markdown rendering
                text(content).into()
            }
            
            BlockContent::FilePreview { path, content, .. } => {
                column![
                    text(format!("File: {}", path.display()))
                        .size(12)
                        .color(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                    text(content).size(14)
                ]
                .spacing(4)
                .into()
            }
            
            BlockContent::Error { message, details } => {
                let mut col = column![
                    text(message).color(iced::Color::from_rgb(0.8, 0.0, 0.0))
                ];
                
                if let Some(details) = details {
                    col = col.push(
                        text(details)
                            .size(12)
                            .color(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    );
                }
                
                col.spacing(4).into()
            }
            
            _ => text("Unsupported block type").into(),
        }
    }
}
