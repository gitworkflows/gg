use iced::{
    widget::{button, column, container, row, scrollable, text, text_input, Space},
    Alignment, Element, Length, Color,
};
use uuid::Uuid;
use log::info;

use crate::config::theme::WarpTheme;
use crate::workflows::manager::WorkflowManager;
use crate::workflows::executor::Workflow; // Import Workflow struct
use crate::terminal::Message; // For sending messages back to the main app

#[derive(Debug, Clone)]
pub enum WorkflowBrowserMessage {
    SearchInputChanged(String),
    ExecuteWorkflow(Uuid),
    EditWorkflow(Uuid),
    DeleteWorkflow(Uuid),
    ImportWorkflowClicked,
    CreateWorkflowClicked,
    // Add messages for pagination, sorting, etc.
}

pub struct WorkflowBrowser {
    workflow_manager: WorkflowManager,
    search_input: String,
    filtered_workflows: Vec<Workflow>,
}

impl WorkflowBrowser {
    pub fn new(workflow_manager: WorkflowManager) -> Self {
        let all_workflows = workflow_manager.get_all_workflows().into_iter().cloned().collect();
        WorkflowBrowser {
            workflow_manager,
            search_input: String::new(),
            filtered_workflows: all_workflows,
        }
    }

    pub fn update(&mut self, message: WorkflowBrowserMessage) -> Option<Message> {
        match message {
            WorkflowBrowserMessage::SearchInputChanged(input) => {
                self.search_input = input;
                self.filter_workflows();
                None
            }
            WorkflowBrowserMessage::ExecuteWorkflow(id) => {
                info!("WorkflowBrowser: Request to execute workflow ID: {}", id);
                // This message is passed up to the main terminal update loop
                Some(Message::ExecuteWorkflow(id))
            }
            WorkflowBrowserMessage::EditWorkflow(id) => {
                info!("WorkflowBrowser: Request to edit workflow ID: {}", id);
                // This would typically open a new view/modal for editing
                None
            }
            WorkflowBrowserMessage::DeleteWorkflow(id) => {
                info!("WorkflowBrowser: Request to delete workflow ID: {}", id);
                if self.workflow_manager.remove_workflow(&id).is_some() {
                    info!("Workflow deleted successfully.");
                    self.filter_workflows(); // Re-filter after deletion
                } else {
                    info!("Workflow not found for deletion.");
                }
                None
            }
            WorkflowBrowserMessage::ImportWorkflowClicked => {
                info!("WorkflowBrowser: Import workflow clicked (trigger file dialog).");
                // This would trigger a file dialog in the main app
                None
            }
            WorkflowBrowserMessage::CreateWorkflowClicked => {
                info!("WorkflowBrowser: Create new workflow clicked.");
                // This would typically open a new view/modal for creating a workflow
                None
            }
        }
    }

    fn filter_workflows(&mut self) {
        let query = self.search_input.to_lowercase();
        self.filtered_workflows = self.workflow_manager.get_all_workflows().into_iter()
            .filter(|wf| wf.name.to_lowercase().contains(&query))
            .cloned()
            .collect();
    }

    pub fn view(&self, theme: &WarpTheme) -> Element<WorkflowBrowserMessage> {
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let workflow_list = scrollable(
            column(
                self.filtered_workflows.iter().map(|workflow| {
                    row![
                        text(&workflow.name).size(18).color(foreground_color).width(Length::Fill),
                        button("Execute").on_press(WorkflowBrowserMessage::ExecuteWorkflow(workflow.id)),
                        button("Edit").on_press(WorkflowBrowserMessage::EditWorkflow(workflow.id)),
                        button("Delete").on_press(WorkflowBrowserMessage::DeleteWorkflow(workflow.id)),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .padding(8)
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

        container(
            column![
                text("Workflow Browser").size(28).color(foreground_color),
                Space::with_height(Length::Fixed(20.0)),
                row![
                    text_input("Search workflows...", &self.search_input)
                        .on_input(WorkflowBrowserMessage::SearchInputChanged)
                        .width(Length::Fill)
                        .padding(10)
                        .size(18)
                        .style(iced::theme::TextInput::Default), // Use default theme for now
                    button("Create New").on_press(WorkflowBrowserMessage::CreateWorkflowClicked)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(accent_color)),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: Color::BLACK,
                        }),
                    button("Import").on_press(WorkflowBrowserMessage::ImportWorkflowClicked)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(accent_color)),
                            border_radius: 4.0.into(),
                            border_width: 0.0,
                            border_color: Color::TRANSPARENT,
                            text_color: Color::BLACK,
                        }),
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                Space::with_height(Length::Fixed(10.0)),
                workflow_list,
            ]
            .spacing(15)
            .padding(20)
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &iced::Theme| container::Appearance {
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
