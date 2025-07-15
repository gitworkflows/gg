use iced::{
    widget::{column, row, text, button, text_input, Space},
    Element, Length, Color,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::workflows::Workflow;
use crate::terminal::Message;
use crate::themes::WarpTheme; // Assuming WarpTheme is accessible

#[derive(Debug, Clone)]
pub enum WorkflowExecutorMessage {
    ToggleVisibility,
    ArgumentInputChanged(String, String), // (argument_name, new_value)
    ExecuteWorkflow,
}

pub struct WorkflowExecutor {
    is_visible: bool,
    workflow: Option<Workflow>,
    argument_values: HashMap<String, String>,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            workflow: None,
            argument_values: HashMap::new(),
        }
    }

    pub fn show_workflow(&mut self, workflow: Workflow) {
        self.workflow = Some(workflow.clone());
        self.argument_values.clear();
        if let Some(args) = workflow.arguments {
            for arg_name in args.keys() {
                self.argument_values.insert(arg_name.clone(), String::new());
            }
        }
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.workflow = None;
        self.argument_values.clear();
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn update(&mut self, message: WorkflowExecutorMessage) -> Option<Message> {
        match message {
            WorkflowExecutorMessage::ToggleVisibility => {
                self.hide();
                None
            }
            WorkflowExecutorMessage::ArgumentInputChanged(name, value) => {
                self.argument_values.insert(name, value);
                None
            }
            WorkflowExecutorMessage::ExecuteWorkflow => {
                if let Some(workflow) = &self.workflow {
                    let workflow_id = workflow.id;
                    let arguments = self.argument_values.clone();
                    self.hide();
                    return Some(Message::ExecuteWorkflowWithArguments(workflow_id, arguments));
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<WorkflowExecutorMessage> {
        let theme = WarpTheme::default_dark(); // Use a default theme for the executor UI
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let mut content = column![];

        if let Some(workflow) = &self.workflow {
            content = content.push(text(format!("Execute Workflow: {}", workflow.name)).size(24).color(foreground_color));
            content = content.push(text(format!("Command: {}", workflow.command)).size(16).color(foreground_color));
            content = content.push(Space::with_height(Length::Fixed(20.0)));

            if let Some(args) = &workflow.arguments {
                for (arg_name, arg_description) in args {
                    let current_value = self.argument_values.get(arg_name).unwrap_or(&String::new()).clone();
                    content = content.push(
                        column![
                            text(format!("{}: {}", arg_name, arg_description)).size(16).color(foreground_color),
                            text_input("Enter value...", &current_value)
                                .on_input(move |s| WorkflowExecutorMessage::ArgumentInputChanged(arg_name.clone(), s))
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
                                }),
                        ]
                        .spacing(5)
                    );
                }
            }

            content = content.push(Space::with_height(Length::Fixed(20.0)));
            content = content.push(
                row![
                    button("Execute").on_press(WorkflowExecutorMessage::ExecuteWorkflow)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(accent_color)),
                            border_radius: 4.0.into(),
                            text_color: Color::BLACK,
                            ..Default::default()
                        }),
                    button("Cancel").on_press(WorkflowExecutorMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(background_color)),
                            border_radius: 4.0.into(),
                            text_color: foreground_color,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
            );
        } else {
            content = content.push(text("No workflow selected for execution.").size(20).color(foreground_color));
        }

        container(
            column![
                row![
                    text("Workflow Executor").size(28).width(Length::Fill).color(foreground_color),
                    button("Close").on_press(WorkflowExecutorMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            border_radius: 4.0.into(),
                            text_color: Color::WHITE,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                content.spacing(10),
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fixed(600.0))
        .height(Length::Shrink)
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
