use iced::{Element, widget::{text, row}};
use iced::{Color, Length};
use crate::config::{PromptSettings, PromptStyle};
use crate::themes::WarpTheme;
use std::path::PathBuf;
use chrono::{Local, Timelike}; // For time chip

pub struct PromptRenderer {
    // Add any state needed for rendering, e.g., current working directory, git status
}

impl PromptRenderer {
    pub fn new() -> Self {
        PromptRenderer {}
    }

    pub fn render_prompt(&self, settings: &PromptSettings, theme: &WarpTheme, current_dir: &PathBuf) -> Element<'static, crate::terminal::Message> {
        let mut prompt_elements = row![];

        match settings.style {
            PromptStyle::Warp => {
                for chip in &settings.context_chips {
                    match chip.as_str() {
                        "cwd" => {
                            if let Some(dir_name) = current_dir.file_name().and_then(|s| s.to_str()) {
                                prompt_elements = prompt_elements.push(
                                    text(format!(" {} ", dir_name))
                                        .size(14)
                                        .color(theme.get_terminal_color("blue", true))
                                );
                            }
                        }
                        "git" => {
                            // Placeholder for Git status
                            prompt_elements = prompt_elements.push(
                                text(" (git:main*) ")
                                    .size(14)
                                    .color(theme.get_terminal_color("green", true))
                            );
                        }
                        "time" => {
                            let now = Local::now();
                            prompt_elements = prompt_elements.push(
                                text(format!(" {} ", now.format("%H:%M").to_string()))
                                    .size(14)
                                    .color(theme.get_terminal_color("white", false))
                            );
                        }
                        "kubernetes" => {
                            // Placeholder for Kubernetes context
                            prompt_elements = prompt_elements.push(
                                text(" (kube:dev) ")
                                    .size(14)
                                    .color(theme.get_terminal_color("cyan", true))
                            );
                        }
                        // Add more chips here
                        _ => {}
                    }
                }
                prompt_elements = prompt_elements.push(
                    text(" $ ")
                        .size(14)
                        .color(theme.get_foreground_color())
                );
            }
            PromptStyle::Shell => {
                // For shell prompt, we'd typically rely on the shell's PS1.
                // For now, we'll just show a generic indicator.
                prompt_elements = prompt_elements.push(
                    text(" (Shell Prompt) $ ")
                        .size(14)
                        .color(theme.get_foreground_color())
                );
            }
        }

        prompt_elements
            .align_items(iced::Alignment::Center)
            .spacing(2)
            .into()
    }
}
