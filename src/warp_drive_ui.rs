use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Color,
};
use uuid::Uuid;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::theme::WarpTheme; // Corrected import path
use crate::terminal::Message;
use crate::input::EditorMessage; // Use the new input module

#[derive(Debug, Clone, PartialEq)] // Added PartialEq for easier comparison in DriveManager
pub enum WarpDriveMessage {
    ToggleVisibility,
    SearchInputChanged(String),
    InputSubmitted(String), // For search input submission
    SelectWorkspace(String),
    SelectFolder(String),
    CreateFolderClicked,
    ImportClicked,
    ExportClicked(Uuid), // For exporting a specific item
    ItemClicked(Uuid), // For selecting/executing an item
    ExecuteWorkflow(Uuid), // Specific message for executing a workflow from Warp Drive
    ExportWorkflow(Uuid), // Specific message for exporting a workflow from Warp Drive
    Editor(EditorMessage), // To integrate with the editor's input handling
    OpenWorkflow(String),
    OpenNotebook(String),
    EditEnvironmentVariable(String),
    // Add messages for creating/editing/deleting items, drag-and-drop, etc.
}

#[derive(Debug, Clone, PartialEq)] // Added PartialEq for easier comparison in DriveManager
pub enum WarpDriveItem {
    Workflow { id: Uuid, name: String },
    Notebook { id: Uuid, name: String },
    Prompt { id: Uuid, name: String },
    EnvironmentVariables { id: Uuid, name: String },
}

impl WarpDriveItem {
    pub fn id(&self) -> Uuid {
        match self {
            WarpDriveItem::Workflow { id, .. } => *id,
            WarpDriveItem::Notebook { id, .. } => *id,
            WarpDriveItem::Prompt { id, .. } => *id,
            WarpDriveItem::EnvironmentVariables { id, .. } => *id,
        }
    }
}

pub struct WarpDriveUI {
    is_visible: bool,
    search_input: String,
    selected_workspace: String,
    selected_folder: String,
    workspaces: Vec<String>,
    folders: Vec<String>,
    items: Vec<WarpDriveItem>, // All items, filtered by selected workspace/folder
}

impl WarpDriveUI {
    pub fn new(initial_items: Vec<WarpDriveItem>, initial_folders: Vec<String>) -> Self {
        WarpDriveUI {
            is_visible: false,
            search_input: String::new(),
            selected_workspace: "Personal".to_string(),
            selected_folder: "All".to_string(),
            workspaces: vec!["Personal".to_string(), "Team A".to_string(), "Team B".to_string()],
            folders: initial_folders,
            items: initial_items,
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
        self.search_input.clear();
        // Re-filter items based on current selection if needed
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn set_items(&mut self, items: Vec<WarpDriveItem>) {
        self.items = items;
    }

    pub fn update(&mut self, message: WarpDriveMessage) -> Option<Message> {
        match message {
            WarpDriveMessage::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                None
            }
            WarpDriveMessage::SearchInputChanged(input) => {
                self.search_input = input;
                None
            }
            WarpDriveMessage::InputSubmitted(input) => {
                // Handle search submission if needed, for now just update search_input
                self.search_input = input;
                None
            }
            WarpDriveMessage::SelectWorkspace(workspace) => {
                self.selected_workspace = workspace;
                self.selected_folder = "All".to_string(); // Reset folder when changing workspace
                // TODO: Filter items based on selected workspace
                None
            }
            WarpDriveMessage::SelectFolder(folder) => {
                self.selected_folder = folder;
                // TODO: Filter items based on selected folder
                None
            }
            WarpDriveMessage::CreateFolderClicked => {
                println!("Create folder clicked (not yet implemented)");
                None
            }
            WarpDriveMessage::ImportClicked => {
                // This will be handled by the Terminal to open a file picker
                Some(Message::WarpDrive(WarpDriveMessage::ImportClicked))
            }
            WarpDriveMessage::ExportClicked(id) => {
                // This will be handled by the Terminal to open a file save dialog
                Some(Message::WarpDrive(WarpDriveMessage::ExportWorkflow(id)))
            }
            WarpDriveMessage::ItemClicked(id) => {
                // Determine item type and send appropriate message
                if let Some(item) = self.items.iter().find(|i| i.id() == id) {
                    match item {
                        WarpDriveItem::Workflow { id, .. } => {
                            return Some(Message::WarpDrive(WarpDriveMessage::ExecuteWorkflow(*id)));
                        }
                        _ => {
                            println!("Clicked on item: {:?}", item);
                        }
                    }
                }
                None
            }
            WarpDriveMessage::ExecuteWorkflow(id) => {
                // This message is passed through to the Terminal
                Some(Message::ExecuteWorkflow(id))
            }
            WarpDriveMessage::ExportWorkflow(id) => {
                // This message is passed through to the Terminal
                Some(Message::ExportWorkflowFile(id, PathBuf::from(format!("/tmp/exported_workflow_{}.yml", id))))
            }
            WarpDriveMessage::Editor(editor_message) => {
                // Handle editor messages for the search input
                match editor_message {
                    EditorMessage::InputChanged(value) => {
                        self.search_input = value;
                    }
                    EditorMessage::Submit => {
                        // Treat submit as search submission
                        return Some(Message::WarpDrive(WarpDriveMessage::InputSubmitted(self.search_input.clone())));
                    }
                    _ => {} // Ignore other editor messages
                }
                None
            }
            WarpDriveMessage::OpenWorkflow(name) => {
                println!("Opening workflow: {}", name);
                None
            }
            WarpDriveMessage::OpenNotebook(name) => {
                println!("Opening notebook: {}", name);
                None
            }
            WarpDriveMessage::EditEnvironmentVariable(name) => {
                println!("Editing environment variable: {}", name);
                None
            }
        }
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<WarpDriveMessage> {
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();

        let sidebar = column![
            text("Workspaces").size(18).color(foreground_color),
            column(
                self.workspaces.iter().map(|ws| {
                    button(text(ws).size(16).color(foreground_color))
                        .on_press(WarpDriveMessage::SelectWorkspace(ws.clone()))
                        .width(Length::Fill)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(if ws == &self.selected_workspace {
                                theme.get_accent_color()
                            } else {
                                background_color
                            })),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: if ws == &self.selected_workspace {
                                Color::BLACK
                            } else {
                                foreground_color
                            },
                        })
                        .into()
                }).collect()
            ).spacing(5),
            Space::with_height(Length::Fixed(20.0)),
            text("Folders").size(18).color(foreground_color),
            column(
                self.folders.iter().map(|folder| {
                    button(text(folder).size(16).color(foreground_color))
                        .on_press(WarpDriveMessage::SelectFolder(folder.clone()))
                        .width(Length::Fill)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(if folder == &self.selected_folder {
                                theme.get_accent_color()
                            } else {
                                background_color
                            })),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: if folder == &self.selected_folder {
                                Color::BLACK
                            } else {
                                foreground_color
                            },
                        })
                        .into()
                }).collect()
            ).spacing(5),
            button("New Folder").on_press(WarpDriveMessage::CreateFolderClicked)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(theme.get_accent_color())),
                    border_radius: 4.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    text_color: Color::BLACK,
                })
                .width(Length::Fill),
        ]
        .width(Length::Fixed(200.0))
        .padding(10)
        .spacing(10)
        .style(move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        let filtered_items: Vec<&WarpDriveItem> = self.items.iter()
            .filter(|item| {
                let item_name = match item {
                    WarpDriveItem::Workflow { name, .. } => name,
                    WarpDriveItem::Notebook { name, .. } => name,
                    WarpDriveItem::Prompt { name, .. } => name,
                    WarpDriveItem::EnvironmentVariables { name, .. } => name,
                };
                item_name.to_lowercase().contains(&self.search_input.to_lowercase())
            })
            .collect();

        let item_list = scrollable(
            column(
                filtered_items.into_iter().map(|item| {
                    let (id, name, item_type) = match item {
                        WarpDriveItem::Workflow { id, name } => (*id, name, "Workflow"),
                        WarpDriveItem::Notebook { id, name } => (*id, name, "Notebook"),
                        WarpDriveItem::Prompt { id, name } => (*id, name, "Prompt"),
                        WarpDriveItem::EnvironmentVariables { id, name } => (*id, name, "Env Vars"),
                    };
                    row![
                        text(format!("[{}] {}", item_type, name)).size(16).width(Length::Fill),
                        button("Open").on_press(WarpDriveMessage::ItemClicked(id)),
                        button("Export").on_press(WarpDriveMessage::ExportClicked(id)),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .padding(5)
                    .style(move |_theme: &iced::Theme| container::Appearance {
                        background: Some(iced::Background::Color(background_color)),
                        border: iced::Border {
                            color: border_color,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    })
                    .into()
                }).collect()
            ).spacing(5)
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1));

        let main_content = column![
            row![
                text_input("Search Warp Drive...", &self.search_input)
                    .on_input(WarpDriveMessage::SearchInputChanged)
                    .on_submit(WarpDriveMessage::InputSubmitted(self.search_input.clone()))
                    .width(Length::Fill),
                button("Import").on_press(WarpDriveMessage::ImportClicked)
                    .style(iced::widget::button::text::Appearance {
                        background: Some(iced::Background::Color(theme.get_accent_color())),
                        border_radius: 4.0.into(),
                        border_width: 0.0,
                        border_color: Color::TRANSPARENT,
                        text_color: Color::BLACK,
                    }),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            item_list,
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        container(
            column![
                row![
                    text("Warp Drive").size(28).width(Length::Fill),
                    button("Close").on_press(WarpDriveMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: Color::WHITE,
                        }),
                ]
                .align_items(Alignment::Center)
                .spacing(10),
                row![
                    sidebar,
                    main_content,
                ]
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill),
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fixed(1000.0))
        .height(Length::Fixed(700.0))
        .style(|_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
            border: iced::Border {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        })
        .into()
    }
}
