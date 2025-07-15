use iced::{Element, widget::{column, container, text, row, button, Space}, Length};
use iced::{Color, alignment};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Local};

use crate::terminal::Message;
use crate::themes::WarpTheme;
use crate::shell::ShellOutput;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Uuid,
    pub content: BlockContent,
}

#[derive(Debug, Clone)]
pub enum BlockContent {
    Command {
        input: String,
        output: String,
        prompt: String, // The prompt string at the time of command execution
        exit_code: Option<i32>,
        execution_time_ms: Option<u128>,
        is_running: bool,
        timestamp: DateTime<Local>,
    },
    Markdown {
        content: String,
        timestamp: DateTime<Local>,
    },
    // Add other content types like images, tables, etc.
}

#[derive(Debug, Clone)]
pub enum BlockMessage {
    ToggleContextMenu,
    CopyCommand,
    CopyOutput,
    CopyBoth,
    ShareBlock,
    ReinputCommand,
    // Add more block-specific actions
}

impl Block {
    pub fn new_command(id: Uuid, input: String, prompt: String) -> Self {
        Self {
            id,
            content: BlockContent::Command {
                input,
                output: String::new(),
                prompt,
                exit_code: None,
                execution_time_ms: None,
                is_running: true,
                timestamp: Local::now(),
            },
        }
    }

    pub fn new_markdown(id: Uuid, content: String) -> Self {
        Self {
            id,
            content: BlockContent::Markdown {
                content,
                timestamp: Local::now(),
            },
        }
    }

    pub fn append_output(&mut self, output: ShellOutput) {
        if let BlockContent::Command { output: current_output, .. } = &mut self.content {
            match output {
                ShellOutput::Stdout(s) => {
                    current_output.push_str(&s);
                    current_output.push('\n');
                },
                ShellOutput::Stderr(s) => {
                    current_output.push_str(&format!("[ERROR] {}\n", s));
                }
            }
        }
    }

    pub fn complete_execution(&mut self, exit_code: i32, execution_time_ms: u128) {
        if let BlockContent::Command { is_running, exit_code: ec, execution_time_ms: et, .. } = &mut self.content {
            *is_running = false;
            *ec = Some(exit_code);
            *et = Some(execution_time_ms);
        }
    }

    pub fn view<'a>(&'a self, theme: &WarpTheme, show_context_menu: bool) -> Element<'a, BlockMessage> {
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let timestamp_text = text(self.content.timestamp().format("%H:%M:%S").to_string())
            .size(12)
            .color(theme.get_terminal_color("white", false));

        let content_view: Element<BlockMessage> = match &self.content {
            BlockContent::Command { input, output, prompt, exit_code, execution_time_ms, is_running, .. } => {
                let exit_status_text = if *is_running {
                    text("RUNNING...").color(theme.get_terminal_color("yellow", true))
                } else {
                    match exit_code {
                        Some(0) => text(format!("SUCCESS ({}ms)", execution_time_ms.unwrap_or(0))).color(theme.get_terminal_color("green", true)),
                        Some(code) => text(format!("FAILED ({}, {}ms)", code, execution_time_ms.unwrap_or(0))).color(theme.get_terminal_color("red", true)),
                        None => text("UNKNOWN").color(theme.get_terminal_color("white", false)),
                    }
                };

                column![
                    row![
                        text(prompt).color(foreground_color),
                        text(input).color(foreground_color).width(Length::Fill),
                        timestamp_text,
                    ]
                    .align_items(alignment::Vertical::Center)
                    .spacing(5),
                    text(output).color(foreground_color).size(14),
                    row![
                        Space::with_width(Length::Fill),
                        exit_status_text,
                    ]
                    .align_items(alignment::Vertical::Center)
                ]
                .spacing(5)
                .into()
            }
            BlockContent::Markdown { content, .. } => {
                column![
                    row![
                        text("Markdown").color(foreground_color).width(Length::Fill),
                        timestamp_text,
                    ]
                    .align_items(alignment::Vertical::Center)
                    .spacing(5),
                    text(content).color(foreground_color).size(14),
                ]
                .spacing(5)
                .into()
            }
        };

        let context_menu = if show_context_menu {
            Some(
                column![
                    button("Copy Command").on_press(BlockMessage::CopyCommand),
                    button("Copy Output").on_press(BlockMessage::CopyOutput),
                    button("Copy Both").on_press(BlockMessage::CopyBoth),
                    button("Share Block").on_press(BlockMessage::ShareBlock),
                    button("Re-input Command").on_press(BlockMessage::ReinputCommand),
                ]
                .spacing(5)
                .padding(5)
                .style(move |_theme: &iced::Theme| container::Appearance {
                    background: Some(iced::Background::Color(background_color)),
                    border_color,
                    border_width: 1.0,
                    border_radius: 4.0.into(),
                    ..Default::default()
                })
                .into()
            )
        } else {
            None
        };

        container(
            column![
                row![
                    content_view,
                    button(text("...").size(16).color(foreground_color))
                        .on_press(BlockMessage::ToggleContextMenu)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(background_color)),
                            border_radius: 4.0.into(),
                            text_color: foreground_color,
                            ..Default::default()
                        }),
                ]
                .spacing(5)
                .align_items(alignment::Vertical::Start), // Align items to start to prevent button from stretching
                if let Some(menu) = context_menu {
                    row![
                        Space::with_width(Length::Fill),
                        menu,
                    ].into()
                } else {
                    Space::with_height(Length::Fixed(0.0)).into()
                }
            ]
        )
        .width(Length::Fill)
        .padding(8)
        .style(move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border_color,
            border_width: 1.0,
            border_radius: 4.0.into(),
            ..Default::default()
        })
        .into()
    }
}

impl BlockContent {
    pub fn timestamp(&self) -> DateTime<Local> {
        match self {
            BlockContent::Command { timestamp, .. } => *timestamp,
            BlockContent::Markdown { timestamp, .. } => *timestamp,
        }
    }
}
