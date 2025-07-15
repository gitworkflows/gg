use iced::{Element, widget::{
    button, column, container, text, text_input, row, scrollable, 
    pick_list, checkbox, tooltip
}};
use iced::{Alignment, Length, Color};
use uuid::Uuid;
use std::collections::HashMap;

use crate::terminal::Message;
use crate::workflows::{Workflow, WorkflowManager, Shell, WorkflowCollection};

#[derive(Debug, Clone)]
pub enum WorkflowBrowserMessage {
    SearchChanged(String),
    TagFilterChanged(Option<String>),
    ShellFilterChanged(Option<Shell>),
    CategoryChanged(WorkflowCategory),
    WorkflowSelected(Uuid),
    ExecuteWorkflow(Uuid),
    AddToFavorites(Uuid),
    RemoveFromFavorites(Uuid),
    EditWorkflow(Uuid),
    DeleteWorkflow(Uuid),
    CreateNewWorkflow,
    ImportWorkflow,
    ExportWorkflow(Uuid),
    RefreshWorkflows,
    SortChanged(WorkflowSortOrder),
    ViewModeChanged(ViewMode),
    Close,
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
    workflows: Vec<Workflow>,
    collections: Vec<WorkflowCollection>,
    favorites: Vec<Uuid>,
    recent_workflows: Vec<Uuid>,
    available_tags: Vec<String>,
    
    // UI state
    is_visible: bool,
    search_query: String,
    selected_tag_filter: Option<String>,
    selected_shell_filter: Option<Shell>,
    current_category: WorkflowCategory,
    sort_order: WorkflowSortOrder,
    view_mode: ViewMode,
    selected_workflow: Option<Uuid>,
}

impl WorkflowBrowser {
    pub fn new() -> Self {
        WorkflowBrowser {
            workflows: vec![],
            collections: vec![],
            favorites: vec![],
            recent_workflows: vec![],
            available_tags: vec![],
            
            is_visible: false,
            search_query: String::new(),
            selected_tag_filter: None,
            selected_shell_filter: None,
            current_category: WorkflowCategory::All,
            sort_order: WorkflowSortOrder::Name,
            view_mode: ViewMode::List,
            selected_workflow: None,
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

    pub fn update_workflows(&mut self, manager: &WorkflowManager) {
        self.workflows = manager.get_all_workflows().into_iter().cloned().collect();
        self.collections = manager.get_collections().clone();
        self.favorites = manager.get_favorites().into_iter().map(|w| w.id).collect();
        self.recent_workflows = manager.get_recent_workflows().into_iter().map(|w| w.id).collect();
        self.available_tags = manager.get_all_tags();
    }

    pub fn update(&mut self, message: WorkflowBrowserMessage) -> Option<Message> {
        match message {
            WorkflowBrowserMessage::SearchChanged(query) => {
                self.search_query = query;
                None
            }
            
            WorkflowBrowserMessage::TagFilterChanged(tag) => {
                self.selected_tag_filter = tag;
                None
            }
            
            WorkflowBrowserMessage::ShellFilterChanged(shell) => {
                self.selected_shell_filter = shell;
                None
            }
            
            WorkflowBrowserMessage::CategoryChanged(category) => {
                self.current_category = category;
                None
            }
            
            WorkflowBrowserMessage::WorkflowSelected(id) => {
                self.selected_workflow = Some(id);
                None
            }
            
            WorkflowBrowserMessage::ExecuteWorkflow(id) => {
                Some(Message::ExecuteWorkflow(id))
            }
            
            WorkflowBrowserMessage::AddToFavorites(id) => {
                Some(Message::AddWorkflowToFavorites(id))
            }
            
            WorkflowBrowserMessage::RemoveFromFavorites(id) => {
                Some(Message::RemoveWorkflowFromFavorites(id))
            }
            
            WorkflowBrowserMessage::EditWorkflow(id) => {
                Some(Message::EditWorkflow(id))
            }
            
            WorkflowBrowserMessage::DeleteWorkflow(id) => {
                Some(Message::DeleteWorkflow(id))
            }
            
            WorkflowBrowserMessage::CreateNewWorkflow => {
                Some(Message::CreateNewWorkflow)
            }
            
            WorkflowBrowserMessage::ImportWorkflow => {
                Some(Message::ImportWorkflow)
            }
            
            WorkflowBrowserMessage::ExportWorkflow(id) => {
                Some(Message::ExportWorkflow(id))
            }
            
            WorkflowBrowserMessage::RefreshWorkflows => {
                Some(Message::RefreshWorkflows)
            }
            
            WorkflowBrowserMessage::SortChanged(order) => {
                self.sort_order = order;
                None
            }
            
            WorkflowBrowserMessage::ViewModeChanged(mode) => {
                self.view_mode = mode;
                None
            }
            
            WorkflowBrowserMessage::Close => {
                self.hide();
                None
            }
        }
    }

    pub fn view(&self) -> Element<WorkflowBrowserMessage> {
        if !self.is_visible {
            return container(text("")).into();
        }

        let header = self.create_header();
        let filters = self.create_filters();
        let categories = self.create_categories();
        let workflows_view = self.create_workflows_view();
        let workflow_details = self.create_workflow_details();

        container(
            column![
                header,
                filters,
                categories,
                row![
                    scrollable(workflows_view)
                        .width(Length::FillPortion(2))
                        .height(Length::Fixed(500.0)),
                    workflow_details
                        .width(Length::FillPortion(1))
                ]
                .spacing(16)
            ]
            .spacing(16)
            .padding(16)
        )
        .width(Length::Fixed(1000.0))
        .height(Length::Fixed(700.0))
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.1))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.3, 0.3, 0.3),
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_header(&self) -> Element<WorkflowBrowserMessage> {
        row![
            text("Workflow Browser").size(20),
            row![
                button("New")
                    .on_press(WorkflowBrowserMessage::CreateNewWorkflow),
                button("Import")
                    .on_press(WorkflowBrowserMessage::ImportWorkflow),
                button("Refresh")
                    .on_press(WorkflowBrowserMessage::RefreshWorkflows),
                button("Close")
                    .on_press(WorkflowBrowserMessage::Close),
            ]
            .spacing(8)
        ]
        .align_items(Alignment::Center)
        .spacing(16)
        .into()
    }

    fn create_filters(&self) -> Element<WorkflowBrowserMessage> {
        let sort_options = vec![
            WorkflowSortOrder::Name,
            WorkflowSortOrder::LastUsed,
            WorkflowSortOrder::UsageCount,
            WorkflowSortOrder::Created,
        ];

        let view_modes = vec![
            ViewMode::List,
            ViewMode::Grid,
            ViewMode::Compact,
        ];

        let shell_options = vec![
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
        ];

        row![
            text_input("Search workflows...", &self.search_query)
                .on_input(WorkflowBrowserMessage::SearchChanged)
                .width(Length::Fixed(200.0)),
            
            text("Tag:"),
            pick_list(
                &self.available_tags[..],
                self.selected_tag_filter.clone(),
                WorkflowBrowserMessage::TagFilterChanged
            ).placeholder("All tags"),
            
            text("Shell:"),
            pick_list(
                shell_options,
                self.selected_shell_filter.clone(),
                WorkflowBrowserMessage::ShellFilterChanged
            ).placeholder("All shells"),
            
            text("Sort:"),
            pick_list(
                sort_options,
                Some(self.sort_order.clone()),
                WorkflowBrowserMessage::SortChanged
            ),
            
            text("View:"),
            pick_list(
                view_modes,
                Some(self.view_mode.clone()),
                WorkflowBrowserMessage::ViewModeChanged
            ),
        ]
        .spacing(8)
        .align_items(Alignment::Center)
        .into()
    }

    fn create_categories(&self) -> Element<WorkflowBrowserMessage> {
        let categories = vec![
            (WorkflowCategory::All, "All", self.workflows.len()),
            (WorkflowCategory::Favorites, "Favorites", self.favorites.len()),
            (WorkflowCategory::Recent, "Recent", self.recent_workflows.len()),
            (WorkflowCategory::Collections, "Collections", self.collections.len()),
            (WorkflowCategory::Custom, "Custom", self.get_custom_workflows().len()),
        ];

        let category_buttons: Vec<Element<WorkflowBrowserMessage>> = categories
            .into_iter()
            .map(|(category, name, count)| {
                let is_active = self.current_category == category;
                
                button(format!("{} ({})", name, count))
                    .on_press(WorkflowBrowserMessage::CategoryChanged(category))
                    .style(move |theme: &iced::Theme, status| {
                        button::Appearance {
                            background: Some(iced::Background::Color(
                                if is_active {
                                    Color::from_rgb(0.2, 0.4, 0.8)
                                } else {
                                    Color::from_rgb(0.3, 0.3, 0.3)
                                }
                            )),
                            border: iced::Border {
                                color: if is_active {
                                    Color::from_rgb(0.4, 0.6, 1.0)
                                } else {
                                    Color::from_rgb(0.5, 0.5, 0.5)
                                },
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            text_color: Color::WHITE,
                            ..Default::default()
                        }
                    })
                    .into()
            })
            .collect();

        row(category_buttons)
            .spacing(8)
            .into()
    }

    fn create_workflows_view(&self) -> Element<WorkflowBrowserMessage> {
        let filtered_workflows = self.get_filtered_workflows();
        
        match self.view_mode {
            ViewMode::List => self.create_list_view(filtered_workflows),
            ViewMode::Grid => self.create_grid_view(filtered_workflows),
            ViewMode::Compact => self.create_compact_view(filtered_workflows),
        }
    }

    fn create_list_view(&self, workflows: Vec<&Workflow>) -> Element<WorkflowBrowserMessage> {
        let workflow_items: Vec<Element<WorkflowBrowserMessage>> = workflows
            .into_iter()
            .map(|workflow| self.create_workflow_list_item(workflow))
            .collect();

        column(workflow_items)
            .spacing(8)
            .into()
    }

    fn create_grid_view(&self, workflows: Vec<&Workflow>) -> Element<WorkflowBrowserMessage> {
        let workflow_items: Vec<Element<WorkflowBrowserMessage>> = workflows
            .into_iter()
            .map(|workflow| self.create_workflow_grid_item(workflow))
            .collect();

        // Create a grid layout (simplified as rows of 3)
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        
        for (i, item) in workflow_items.into_iter().enumerate() {
            current_row.push(item);
            
            if current_row.len() == 3 || i == workflows.len() - 1 {
                rows.push(row(current_row).spacing(8).into());
                current_row = Vec::new();
            }
        }

        column(rows)
            .spacing(8)
            .into()
    }

    fn create_compact_view(&self, workflows: Vec<&Workflow>) -> Element<WorkflowBrowserMessage> {
        let workflow_items: Vec<Element<WorkflowBrowserMessage>> = workflows
            .into_iter()
            .map(|workflow| self.create_workflow_compact_item(workflow))
            .collect();

        column(workflow_items)
            .spacing(4)
            .into()
    }

    fn create_workflow_list_item(&self, workflow: &Workflow) -> Element<WorkflowBrowserMessage> {
        let is_favorite = self.favorites.contains(&workflow.id);
        let is_selected = self.selected_workflow == Some(workflow.id);

        let tags = if let Some(tags) = &workflow.tags {
            row(
                tags.iter()
                    .take(3)
                    .map(|tag| {
                        container(text(tag).size(10))
                            .padding(2)
                            .style(|theme: &iced::Theme| {
                                container::Appearance {
                                    background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.4, 0.8))),
                                    border: iced::Border {
                                        radius: 4.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                            })
                            .into()
                    })
                    .collect::<Vec<_>>()
            )
            .spacing(4)
        } else {
            row![].spacing(4)
        };

        let info = column![
            row![
                text(&workflow.name).size(16),
                if is_favorite {
                    text("★").color(Color::from_rgb(1.0, 0.8, 0.0))
                } else {
                    text("")
                }
            ]
            .spacing(8),
            if let Some(desc) = &workflow.description {
                text(desc).size(12).color(Color::from_rgb(0.7, 0.7, 0.7))
            } else {
                text("").size(12)
            },
            text(&workflow.command).size(11).color(Color::from_rgb(0.5, 0.8, 0.5)),
            tags,
        ]
        .spacing(4);

        let actions = row![
            button("Execute")
                .on_press(WorkflowBrowserMessage::ExecuteWorkflow(workflow.id)),
            if is_favorite {
                button("♥")
                    .on_press(WorkflowBrowserMessage::RemoveFromFavorites(workflow.id))
                    .style(|theme: &iced::Theme, status| {
                        button::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            ..Default::default()
                        }
                    })
            } else {
                button("♡")
                    .on_press(WorkflowBrowserMessage::AddToFavorites(workflow.id))
            },
            button("Edit")
                .on_press(WorkflowBrowserMessage::EditWorkflow(workflow.id)),
            button("Export")
                .on_press(WorkflowBrowserMessage::ExportWorkflow(workflow.id)),
        ]
        .spacing(4);

        container(
            row![
                info.width(Length::Fill),
                actions,
            ]
            .spacing(12)
            .align_items(Alignment::Center)
        )
        .padding(12)
        .style(move |theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(
                    if is_selected {
                        Color::from_rgb(0.15, 0.25, 0.35)
                    } else {
                        Color::from_rgb(0.05, 0.05, 0.05)
                    }
                )),
                border: iced::Border {
                    color: if is_selected {
                        Color::from_rgb(0.2, 0.4, 0.8)
                    } else {
                        Color::from_rgb(0.2, 0.2, 0.2)
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .into()
    }

    fn create_workflow_grid_item(&self, workflow: &Workflow) -> Element<WorkflowBrowserMessage> {
        let is_favorite = self.favorites.contains(&workflow.id);

        container(
            column![
                row![
                    text(&workflow.name).size(14),
                    if is_favorite {
                        text("★").color(Color::from_rgb(1.0, 0.8, 0.0))
                    } else {
                        text("")
                    }
                ]
                .spacing(4),
                if let Some(desc) = &workflow.description {
                    text(desc).size(10).color(Color::from_rgb(0.7, 0.7, 0.7))
                } else {
                    text("").size(10)
                },
                button("Execute")
                    .on_press(WorkflowBrowserMessage::ExecuteWorkflow(workflow.id))
                    .width(Length::Fill),
            ]
            .spacing(8)
            .align_items(Alignment::Center)
        )
        .padding(12)
        .width(Length::Fixed(200.0))
        .height(Length::Fixed(120.0))
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_workflow_compact_item(&self, workflow: &Workflow) -> Element<WorkflowBrowserMessage> {
        let is_favorite = self.favorites.contains(&workflow.id);

        container(
            row![
                text(&workflow.name).size(14).width(Length::Fixed(200.0)),
                text(&workflow.command).size(11).color(Color::from_rgb(0.5, 0.8, 0.5)).width(Length::Fill),
                if is_favorite {
                    text("★").color(Color::from_rgb(1.0, 0.8, 0.0))
                } else {
                    text("")
                },
                button("Execute")
                    .on_press(WorkflowBrowserMessage::ExecuteWorkflow(workflow.id)),
            ]
            .spacing(8)
            .align_items(Alignment::Center)
        )
        .padding(6)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 2.0.into(),
                },
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .into()
    }

    fn create_workflow_details(&self) -> Element<WorkflowBrowserMessage> {
        if let Some(workflow_id) = self.selected_workflow {
            if let Some(workflow) = self.workflows.iter().find(|w| w.id == workflow_id) {
                return self.create_workflow_detail_view(workflow);
            }
        }

        container(
            column![
                text("Workflow Details").size(16),
                text("Select a workflow to view details").color(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(8)
        )
        .padding(16)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_workflow_detail_view(&self, workflow: &Workflow) -> Element<WorkflowBrowserMessage> {
        let mut details = vec![
            text(&workflow.name).size(18).into(),
        ];

        if let Some(desc) = &workflow.description {
            details.push(text(desc).size(12).color(Color::from_rgb(0.7, 0.7, 0.7)).into());
        }

        details.push(text("Command:").size(14).into());
        details.push(
            container(text(&workflow.command).size(11))
                .padding(8)
                .style(|theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                        border: iced::Border {
                            color: Color::from_rgb(0.3, 0.3, 0.3),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })
                .into()
        );

        if let Some(tags) = &workflow.tags {
            details.push(text("Tags:").size(14).into());
            details.push(
                row(
                    tags.iter()
                        .map(|tag| {
                            container(text(tag).size(10))
                                .padding(4)
                                .style(|theme: &iced::Theme| {
                                    container::Appearance {
                                        background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.4, 0.8))),
                                        border: iced::Border {
                                            radius: 4.0.into(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }
                                })
                                .into()
                        })
                        .collect::<Vec<_>>()
                )
                .spacing(4)
                .into()
            );
        }

        if let Some(arguments) = &workflow.arguments {
            details.push(text("Arguments:").size(14).into());
            for arg in arguments {
                let arg_info = column![
                    text(&arg.name).size(12),
                    if let Some(desc) = &arg.description {
                        text(desc).size(10).color(Color::from_rgb(0.6, 0.6, 0.6))
                    } else {
                        text("").size(10)
                    },
                    if let Some(default) = &arg.default_value {
                        text(format!("Default: {}", default)).size(10).color(Color::from_rgb(0.5, 0.5, 0.5))
                    } else {
                        text("").size(10)
                    }
                ]
                .spacing(2);

                details.push(
                    container(arg_info)
                        .padding(8)
                        .style(|theme: &iced::Theme| {
                            container::Appearance {
                                background: Some(iced::Background::Color(Color::from_rgb(0.08, 0.08, 0.08))),
                                border: iced::Border {
                                    color: Color::from_rgb(0.2, 0.2, 0.2),
                                    width: 1.0,
                                    radius: 4.0.into(),
                                },
                                ..Default::default()
                            }
                        })
                        .into()
                );
            }
        }

        details.push(text(format!("Usage count: {}", workflow.usage_count)).size(10).into());
        
        if let Some(last_used) = workflow.last_used {
            details.push(text(format!("Last used: {}", last_used.format("%Y-%m-%d %H:%M"))).size(10).into());
        }

        container(
            scrollable(column(details).spacing(8))
        )
        .padding(16)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn get_filtered_workflows(&self) -> Vec<&Workflow> {
        let mut workflows: Vec<&Workflow> = match self.current_category {
            WorkflowCategory::All => self.workflows.iter().collect(),
            WorkflowCategory::Favorites => {
                self.workflows.iter()
                    .filter(|w| self.favorites.contains(&w.id))
                    .collect()
            }
            WorkflowCategory::Recent => {
                self.workflows.iter()
                    .filter(|w| self.recent_workflows.contains(&w.id))
                    .collect()
            }
            WorkflowCategory::Collections => {
                self.collections.iter()
                    .flat_map(|c| c.workflows.iter())
                    .collect()
            }
            WorkflowCategory::Custom => self.get_custom_workflows(),
        };

        // Apply search filter
        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            workflows.retain(|workflow| {
                workflow.name.to_lowercase().contains(&query) ||
                workflow.command.to_lowercase().contains(&query) ||
                workflow.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query)) ||
                workflow.tags.as_ref().map_or(false, |tags| {
                    tags.iter().any(|tag| tag.to_lowercase().contains(&query))
                })
            });
        }

        // Apply tag filter
        if let Some(tag_filter) = &self.selected_tag_filter {
            workflows.retain(|workflow| {
                workflow.tags.as_ref().map_or(false, |tags| {
                    tags.iter().any(|tag| tag.eq_ignore_ascii_case(tag_filter))
                })
            });
        }

        // Apply shell filter
        if let Some(shell_filter) = &self.selected_shell_filter {
            workflows.retain(|workflow| {
                workflow.shells.as_ref().map_or(true, |shells| shells.contains(shell_filter))
            });
        }

        // Sort workflows
        self.sort_workflows(&mut workflows);

        workflows
    }

    fn get_custom_workflows(&self) -> Vec<&Workflow> {
        self.workflows.iter()
            .filter(|w| w.file_path.is_some())
            .collect()
    }

    fn sort_workflows(&self, workflows: &mut Vec<&Workflow>) {
        match self.sort_order {
            WorkflowSortOrder::Name => {
                workflows.sort_by(|a, b| a.name.cmp(&b.name));
            }
            WorkflowSortOrder::LastUsed => {
                workflows.sort_by(|a, b| {
                    b.last_used.unwrap_or(DateTime::from_timestamp(0, 0).unwrap())
                        .cmp(&a.last_used.unwrap_or(DateTime::from_timestamp(0, 0).unwrap()))
                });
            }
            WorkflowSortOrder::UsageCount => {
                workflows.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
            }
            WorkflowSortOrder::Created => {
                workflows.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            }
        }
    }
}

impl std::fmt::Display for WorkflowSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowSortOrder::Name => write!(f, "Name"),
            WorkflowSortOrder::LastUsed => write!(f, "Last Used"),
            WorkflowSortOrder::UsageCount => write!(f, "Usage Count"),
            WorkflowSortOrder::Created => write!(f, "Created"),
        }
    }
}

impl std::fmt::Display for ViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewMode::List => write!(f, "List"),
            ViewMode::Grid => write!(f, "Grid"),
            ViewMode::Compact => write!(f, "Compact"),
        }
    }
}
