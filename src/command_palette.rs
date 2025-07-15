use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Color,
};
use crate::config::theme::WarpTheme;
use crate::terminal::Message;
use crate::input::EditorMessage; // Use the new input module

#[derive(Debug, Clone)]
pub enum CommandPaletteMessage {
    ToggleVisibility,
    SearchInputChanged(String),
    InputSubmitted(String),
    ExecuteCommand(String),
    Editor(EditorMessage),
}

pub struct CommandPalette {
    is_visible: bool,
    search_input: String,
    commands: Vec<Command>,
    filtered_commands: Vec<Command>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub action: CommandAction,
}

#[derive(Debug, Clone)]
pub enum CommandAction {
    ClearTerminal,
    ToggleFullscreen,
    OpenPreferences,
    OpenThemeCustomizer,
    OpenProfileManager,
    OpenWorkflowBrowser,
    OpenWarpDrive,
    Custom(String), // For commands that trigger specific logic
}

impl CommandPalette {
    pub fn new() -> Self {
        let all_commands = vec![
            Command {
                name: "Clear Terminal".to_string(),
                description: "Clears all output from the terminal.".to_string(),
                action: CommandAction::ClearTerminal,
            },
            Command {
                name: "Toggle Fullscreen".to_string(),
                description: "Toggles fullscreen mode.".to_string(),
                action: CommandAction::ToggleFullscreen,
            },
            Command {
                name: "Open Preferences".to_string(),
                description: "Opens the application preferences window.".to_string(),
                action: CommandAction::OpenPreferences,
            },
            Command {
                name: "Open Theme Customizer".to_string(),
                description: "Opens the theme customization window.".to_string(),
                action: CommandAction::OpenThemeCustomizer,
            },
            Command {
                name: "Open Profile Manager".to_string(),
                description: "Opens the profile management window.".to_string(),
                action: CommandAction::OpenProfileManager,
            },
            Command {
                name: "Open Workflow Browser".to_string(),
                description: "Opens the workflow browser.".to_string(),
                action: CommandAction::OpenWorkflowBrowser,
            },
            Command {
                name: "Open Warp Drive".to_string(),
                description: "Opens the Warp Drive panel for workflows, notebooks, and env vars.".to_string(),
                action: CommandAction::OpenWarpDrive,
            },
            Command {
                name: "Run 'ls -la'".to_string(),
                description: "Executes the 'ls -la' command in the shell.".to_string(),
                action: CommandAction::Custom("ls -la".to_string()),
            },
            Command {
                name: "Show current directory".to_string(),
                description: "Displays the current working directory.".to_string(),
                action: CommandAction::Custom("pwd".to_string()),
            },
        ];

        Self {
            is_visible: false,
            search_input: String::new(),
            filtered_commands: all_commands.clone(),
            commands: all_commands,
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
        self.search_input.clear();
        self.filter_commands();
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn update(&mut self, message: CommandPaletteMessage) -> Option<Message> {
        match message {
            CommandPaletteMessage::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                if self.is_visible {
                    self.search_input.clear();
                    self.filter_commands();
                }
                None
            }
            CommandPaletteMessage::SearchInputChanged(input) => {
                self.search_input = input;
                self.filter_commands();
                None
            }
            CommandPaletteMessage::InputSubmitted(input) => {
                // If a command is submitted via Enter, try to execute the first filtered command
                if !self.filtered_commands.is_empty() {
                    let command_to_execute = self.filtered_commands[0].action.clone();
                    self.hide(); // Hide palette after submission
                    return Some(Message::CommandPalette(CommandPaletteMessage::ExecuteCommand(command_to_execute.to_string())));
                }
                None
            }
            CommandPaletteMessage::ExecuteCommand(command_str) => {
                // This message is typically handled by the main terminal update loop
                // to trigger the actual action.
                self.hide(); // Hide palette after execution
                match CommandAction::from_string(&command_str) {
                    Some(action) => Some(Message::ExecuteCommandAction(action)),
                    None => {
                        // If it's a custom command string, send it to the shell
                        Some(Message::ExecuteCommand(command_str))
                    }
                }
            }
            CommandPaletteMessage::Editor(editor_message) => {
                match editor_message {
                    EditorMessage::InputChanged(value) => {
                        self.search_input = value;
                        self.filter_commands();
                    }
                    EditorMessage::Submit => {
                        if !self.filtered_commands.is_empty() {
                            let command_to_execute = self.filtered_commands[0].action.clone();
                            self.hide();
                            return Some(Message::ExecuteCommandAction(command_to_execute));
                        }
                    }
                    _ => {} // Ignore other editor messages
                }
                None
            }
        }
    }

    fn filter_commands(&mut self) {
        let search_query = self.search_input.to_lowercase();
        self.filtered_commands = self.commands.iter()
            .filter(|cmd| {
                cmd.name.to_lowercase().contains(&search_query) ||
                cmd.description.to_lowercase().contains(&search_query)
            })
            .cloned()
            .collect();
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<CommandPaletteMessage> {
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();

        let command_list = scrollable(
            column(
                self.filtered_commands.iter().map(|cmd| {
                    button(
                        column![
                            text(&cmd.name).size(18).color(foreground_color),
                            text(&cmd.description).size(14).color(foreground_color.scale_rgb(0.7)),
                        ]
                        .align_items(Alignment::Start)
                        .spacing(2)
                    )
                    .on_press(CommandPaletteMessage::ExecuteCommand(cmd.action.to_string()))
                    .width(Length::Fill)
                    .padding(10)
                    .style(iced::widget::button::text::Appearance {
                        background: Some(iced::Background::Color(background_color)),
                        border_radius: 4.0.into(),
                        border_width: 1.0,
                        border_color: border_color,
                        text_color: foreground_color,
                    })
                    .into()
                }).collect()
            ).spacing(5)
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1));

        container(
            column![
                row![
                    text_input("Search commands...", &self.search_input)
                        .on_input(CommandPaletteMessage::SearchInputChanged)
                        .on_submit(CommandPaletteMessage::InputSubmitted(self.search_input.clone()))
                        .width(Length::Fill)
                        .padding(10)
                        .size(18)
                        .style(iced::theme::TextInput::Default), // Use default theme for now
                    button("Close").on_press(CommandPaletteMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: Color::WHITE,
                        }),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                Space::with_height(Length::Fixed(10.0)),
                command_list,
            ]
            .spacing(10)
            .padding(20)
        )
        .width(Length::Fixed(600.0))
        .height(Length::Fixed(400.0))
        .style(|_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                color: border_color,
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}

impl CommandAction {
    pub fn to_string(&self) -> String {
        match self {
            CommandAction::ClearTerminal => "ClearTerminal".to_string(),
            CommandAction::ToggleFullscreen => "ToggleFullscreen".to_string(),
            CommandAction::OpenPreferences => "OpenPreferences".to_string(),
            CommandAction::OpenThemeCustomizer => "OpenThemeCustomizer".to_string(),
            CommandAction::OpenProfileManager => "OpenProfileManager".to_string(),
            CommandAction::OpenWorkflowBrowser => "OpenWorkflowBrowser".to_string(),
            CommandAction::OpenWarpDrive => "OpenWarpDrive".to_string(),
            CommandAction::Custom(s) => s.clone(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "ClearTerminal" => Some(CommandAction::ClearTerminal),
            "ToggleFullscreen" => Some(CommandAction::ToggleFullscreen),
            "OpenPreferences" => Some(CommandAction::OpenPreferences),
            "OpenThemeCustomizer" => Some(CommandAction::OpenThemeCustomizer),
            "OpenProfileManager" => Some(CommandAction::OpenProfileManager),
            "OpenWorkflowBrowser" => Some(CommandAction::OpenWorkflowBrowser),
            "OpenWarpDrive" => Some(CommandAction::OpenWarpDrive),
            _ => Some(CommandAction::Custom(s.to_string())), // Assume it's a custom command if not a predefined action
        }
    }
}
