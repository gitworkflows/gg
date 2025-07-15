use iced::{Element, widget::{
    button, column, container, text, text_input, row, scrollable, 
    pick_list, checkbox, tooltip, Space
}};
use iced::{Alignment, Length, Color};
use uuid::Uuid;
use std::collections::HashMap;

use crate::terminal::Message;
use crate::workflows::{Workflow, WorkflowManager, Shell, WorkflowCollection};
use crate::themes::WarpTheme; // Assuming WarpTheme is accessible

#[derive(Debug, Clone)]
pub enum WorkflowBrowserMessage {
    ToggleVisibility,
    SearchInputChanged(String),
    ExecuteWorkflowClicked(Uuid),
    AddFavoriteClicked(Uuid),
    RemoveFavoriteClicked(Uuid),
    EditWorkflowClicked(Uuid),
    DeleteWorkflowClicked(Uuid),
    CreateNewWorkflowClicked,
    ImportWorkflowClicked,
    ExportWorkflowClicked(Uuid),
    RefreshWorkflowsClicked,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowCategory {
    All,
    Favorites,
    Recent,
    Collections,
    Custom,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowSortOrder {
    Name,
    LastUsed,
    UsageCount,
    Created,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    List,
    Grid,
    Compact,
}

pub struct WorkflowBrowser {
    is_visible: bool,
    search_query: String,
    workflows: Vec<Workflow>,
    favorite_workflow_ids: Vec<Uuid>,
}

impl WorkflowBrowser {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            search_query: String::new(),
            workflows: Vec::new(),
            favorite_workflow_ids: Vec::new(),
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn update_workflows(&mut self, workflow_manager: &WorkflowManager) {
        self.workflows = workflow_manager.get_all_workflows().into_iter().cloned().collect();
        self.favorite_workflow_ids = workflow_manager.get_favorite_workflow_ids();
    }

    pub fn update(&mut self, message: WorkflowBrowserMessage) -> Option<Message> {
        match message {
            WorkflowBrowserMessage::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                None
            }
            WorkflowBrowserMessage::SearchInputChanged(query) => {
                self.search_query = query;
                None
            }
            WorkflowBrowserMessage::ExecuteWorkflowClicked(id) => {
                Some(Message::ExecuteWorkflow(id))
            }
            WorkflowBrowserMessage::AddFavoriteClicked(id) => {
                Some(Message::AddWorkflowToFavorites(id))
            }
            WorkflowBrowserMessage::RemoveFavoriteClicked(id) => {
                Some(Message::RemoveWorkflowFromFavorites(id))
            }
            WorkflowBrowserMessage::EditWorkflowClicked(id) => {
                println!("Edit workflow: {:?}", id); // Placeholder
                None
            }
            WorkflowBrowserMessage::DeleteWorkflowClicked(id) => {
                println!("Delete workflow: {:?}", id); // Placeholder
                None
            }
            WorkflowBrowserMessage::CreateNewWorkflowClicked => {
                println!("Create new workflow (not yet implemented)"); // Placeholder
                None
            }
            WorkflowBrowserMessage::ImportWorkflowClicked => {
                println!("Import workflow (not yet implemented)"); // Placeholder
                None
            }
            WorkflowBrowserMessage::ExportWorkflowClicked(id) => {
                println!("Export workflow: {:?}", id); // Placeholder
                None
            }
            WorkflowBrowserMessage::RefreshWorkflowsClicked => {
                Some(Message::RefreshWorkflows)
            }
        }
    }

    pub fn view(&self) -> Element<WorkflowBrowserMessage> {
        let theme = WarpTheme::default_dark(); // Use a default theme for the browser UI
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let filtered_workflows: Vec<&Workflow> = self.workflows.iter()
            .filter(|w| w.name.to_lowercase().contains(&self.search_query.to_lowercase()))
            .collect();

        let workflow_list = scrollable(
            column(
                filtered_workflows.into_iter().map(|workflow| {
                    let is_favorite = self.favorite_workflow_ids.contains(&workflow.id);
                    row![
                        text(&workflow.name).size(16).color(foreground_color).width(Length::Fill),
                        button(text(if is_favorite { "Unfavorite" } else { "Favorite" }).color(foreground_color))
                            .on_press(if is_favorite {
                                WorkflowBrowserMessage::RemoveFavoriteClicked(workflow.id)
                            } else {
                                WorkflowBrowserMessage::AddFavoriteClicked(workflow.id)
                            })
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(background_color)),
                                border_radius: 4.0.into(),
                                text_color: foreground_color,
                                ..Default::default()
                            }),
                        button(text("Edit").color(foreground_color))
                            .on_press(WorkflowBrowserMessage::EditWorkflowClicked(workflow.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(background_color)),
                                border_radius: 4.0.into(),
                                text_color: foreground_color,
                                ..Default::default()
                            }),
                        button(text("Delete").color(foreground_color))
                            .on_press(WorkflowBrowserMessage::DeleteWorkflowClicked(workflow.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(background_color)),
                                border_radius: 4.0.into(),
                                text_color: foreground_color,
                                ..Default::default()
                            }),
                        button(text("Execute").color(Color::BLACK))
                            .on_press(WorkflowBrowserMessage::ExecuteWorkflowClicked(workflow.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(accent_color)),
                                border_radius: 4.0.into(),
                                text_color: Color::BLACK,
                                ..Default::default()
                            }),
                    ]
                    .spacing(5)
                    .into()
                }).collect()
            )
            .spacing(8)
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1));

        container(
            column![
                row![
                    text("Workflow Browser").size(28).width(Length::Fill).color(foreground_color),
                    button("Close").on_press(WorkflowBrowserMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            border_radius: 4.0.into(),
                            text_color: Color::WHITE,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                row![
                    text_input("Search workflows...", &self.search_query)
                        .on_input(WorkflowBrowserMessage::SearchInputChanged)
                        .padding(8)
                        .size(16)
                        .style(iced::widget::text_input::Appearance {
                            background: iced::Background::Color(background_color),
                            border: iced::Border {
                                color: border_color,
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            icon_color: foreground_color,
                            placeholder_color: theme.get_terminal_color("white", false),
                            value_color: foreground_color,
                            selection_color: accent_color,
                        })
                        .width(Length::Fill),
                    button("Refresh").on_press(WorkflowBrowserMessage::RefreshWorkflowsClicked)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(background_color)),
                            border_radius: 4.0.into(),
                            text_color: foreground_color,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                workflow_list,
                Space::with_height(Length::Fixed(20.0)),
                row![
                    button("Create New").on_press(WorkflowBrowserMessage::CreateNewWorkflowClicked)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(accent_color)),
                            border_radius: 4.0.into(),
                            text_color: Color::BLACK,
                            ..Default::default()
                        }),
                    button("Import").on_press(WorkflowBrowserMessage::ImportWorkflowClicked)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(background_color)),
                            border_radius: 4.0.into(),
                            text_color: foreground_color,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .width(Length::Fill),
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fixed(800.0))
        .height(Length::Fixed(600.0))
        .center_x()
        .center_y()
        .style(move |_theme: &iced::Theme| iced::widget::container::Appearance {
            background: Some(theme.get_background_color()),
            border_color: theme.get_border_color(),
            border_width: 2.0,
            border_radius: 8.0.into(),
            ..Default::default()
        })
        .into()
    }
}
