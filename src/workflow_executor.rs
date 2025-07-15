use iced::{Element, widget::{
    button, column, container, text, text_input, row, scrollable, 
    pick_list, checkbox
}};
use iced::{Alignment, Length, Color};
use uuid::Uuid;
use std::collections::HashMap;

use crate::terminal::Message;
use crate::workflows::{Workflow, WorkflowArgument, ArgumentType};

#[derive(Debug, Clone)]
pub enum WorkflowExecutorMessage {
    ArgumentChanged(String, String),
    ArgumentTypeChanged(String, String),
    Execute,
    Cancel,
    SaveAsTemplate,
    LoadTemplate(String),
}

pub struct WorkflowExecutor {
    workflow: Option<Workflow>,
    argument_values: HashMap<String, String>,
    resolved_command: String,
    is_visible: bool,
    validation_errors: HashMap<String, String>,
    saved_templates: HashMap<String, HashMap<String, String>>,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        WorkflowExecutor {
            workflow: None,
            argument_values: HashMap::new(),
            resolved_command: String::new(),
            is_visible: false,
            validation_errors: HashMap::new(),
            saved_templates: HashMap::new(),
        }
    }

    pub fn show_workflow(&mut self, workflow: Workflow) {
        self.workflow = Some(workflow.clone());
        self.argument_values.clear();
        self.validation_errors.clear();
        
        // Initialize with default values
        if let Some(arguments) = &workflow.arguments {
            for arg in arguments {
                if let Some(default_value) = &arg.default_value {
                    self.argument_values.insert(arg.name.clone(), default_value.clone());
                }
            }
        }
        
        self.update_resolved_command();
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.workflow = None;
        self.argument_values.clear();
        self.validation_errors.clear();
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn update(&mut self, message: WorkflowExecutorMessage) -> Option<Message> {
        match message {
            WorkflowExecutorMessage::ArgumentChanged(name, value) => {
                self.argument_values.insert(name.clone(), value);
                self.validate_argument(&name);
                self.update_resolved_command();
                None
            }
            
            WorkflowExecutorMessage::Execute => {
                if self.validate_all_arguments() {
                    if let Some(workflow) = &self.workflow {
                        self.hide();
                        return Some(Message::ExecuteWorkflowWithArguments(
                            workflow.id,
                            self.argument_values.clone()
                        ));
                    }
                }
                None
            }
            
            WorkflowExecutorMessage::Cancel => {
                self.hide();
                None
            }
            
            WorkflowExecutorMessage::SaveAsTemplate => {
                if let Some(workflow) = &self.workflow {
                    let template_name = format!("{}_template", workflow.name);
                    self.saved_templates.insert(template_name, self.argument_values.clone());
                }
                None
            }
            
            WorkflowExecutorMessage::LoadTemplate(template_name) => {
                if let Some(template) = self.saved_templates.get(&template_name) {
                    self.argument_values = template.clone();
                    self.update_resolved_command();
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<WorkflowExecutorMessage> {
        if !self.is_visible || self.workflow.is_none() {
            return container(text("")).into();
        }

        let workflow = self.workflow.as_ref().unwrap();

        let header = column![
            text(&workflow.name).size(20),
            if let Some(desc) = &workflow.description {
                text(desc).size(12).color(Color::from_rgb(0.7, 0.7, 0.7))
            } else {
                text("").size(12)
            }
        ]
        .spacing(4);

        let arguments_section = if let Some(arguments) = &workflow.arguments {
            self.create_arguments_section(arguments)
        } else {
            column![text("No arguments required")].into()
        };

        let command_preview = self.create_command_preview();
        let actions = self.create_actions();

        container(
            column![
                header,
                arguments_section,
                command_preview,
                actions,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(Length::Fixed(600.0))
        .height(Length::Fixed(500.0))
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

    fn create_arguments_section(&self, arguments: &[WorkflowArgument]) -> Element<WorkflowExecutorMessage> {
        let argument_inputs: Vec<Element<WorkflowExecutorMessage>> = arguments
            .iter()
            .map(|arg| self.create_argument_input(arg))
            .collect();

        container(
            column![
                text("Arguments:").size(16),
                column(argument_inputs).spacing(8)
            ]
            .spacing(8)
        )
        .padding(8)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_argument_input(&self, arg: &WorkflowArgument) -> Element<WorkflowExecutorMessage> {
        let current_value = self.argument_values.get(&arg.name).cloned().unwrap_or_default();
        let has_error = self.validation_errors.contains_key(&arg.name);
        
        let label = row![
            text(&arg.name).size(14),
            if arg.required.unwrap_or(false) {
                text("*").color(Color::from_rgb(1.0, 0.3, 0.3))
            } else {
                text("")
            }
        ]
        .spacing(4);

        let input = match &arg.argument_type {
            Some(ArgumentType::Choice(choices)) => {
                pick_list(
                    choices.clone(),
                    if current_value.is_empty() { None } else { Some(current_value.clone()) },
                    move |value| WorkflowExecutorMessage::ArgumentChanged(arg.name.clone(), value)
                )
                .placeholder("Select an option")
                .into()
            }
            Some(ArgumentType::Boolean) => {
                let is_checked = current_value == "true";
                checkbox("", is_checked)
                    .on_toggle(move |checked| {
                        WorkflowExecutorMessage::ArgumentChanged(
                            arg.name.clone(),
                            checked.to_string()
                        )
                    })
                    .into()
            }
            _ => {
                let placeholder = match &arg.argument_type {
                    Some(ArgumentType::File) => "Select file...",
                    Some(ArgumentType::Directory) => "Select directory...",
                    Some(ArgumentType::Url) => "https://example.com",
                    Some(ArgumentType::Email) => "user@example.com",
                    Some(ArgumentType::Number) => "123",
                    _ => "Enter value...",
                };

                text_input(placeholder, &current_value)
                    .on_input(move |value| WorkflowExecutorMessage::ArgumentChanged(arg.name.clone(), value))
                    .style(move |theme: &iced::Theme, status| {
                        text_input::Appearance {
                            background: iced::Background::Color(
                                if has_error {
                                    Color::from_rgb(0.3, 0.1, 0.1)
                                } else {
                                    Color::from_rgb(0.1, 0.1, 0.1)
                                }
                            ),
                            border: iced::Border {
                                color: if has_error {
                                    Color::from_rgb(1.0, 0.3, 0.3)
                                } else {
                                    Color::from_rgb(0.3, 0.3, 0.3)
                                },
                                width: 1.0,
                                radius: 4.0.into(),
                            },
                            icon_color: Color::WHITE,
                            placeholder_color: Color::from_rgb(0.5, 0.5, 0.5),
                            value_color: Color::WHITE,
                            selection_color: Color::from_rgb(0.2, 0.4, 0.8),
                        }
                    })
                    .into()
            }
        };

        let mut elements = vec![
            label.into(),
            input,
        ];

        if let Some(desc) = &arg.description {
            elements.push(
                text(desc)
                    .size(10)
                    .color(Color::from_rgb(0.6, 0.6, 0.6))
                    .into()
            );
        }

        if let Some(error) = self.validation_errors.get(&arg.name) {
            elements.push(
                text(error)
                    .size(10)
                    .color(Color::from_rgb(1.0, 0.3, 0.3))
                    .into()
            );
        }

        column(elements)
            .spacing(4)
            .into()
    }

    fn create_command_preview(&self) -> Element<WorkflowExecutorMessage> {
        container(
            column![
                text("Command Preview:").size(14),
                container(
                    scrollable(
                        text(&self.resolved_command)
                            .size(12)
                            .color(Color::from_rgb(0.5, 0.8, 0.5))
                    )
                )
                .padding(8)
                .height(Length::Fixed(80.0))
                .style(|theme: &iced::Theme| {
                    container::Appearance {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.02, 0.02, 0.02))),
                        border: iced::Border {
                            color: iced::Color::from_rgb(0.3, 0.3, 0.3),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })
            ]
            .spacing(8)
        )
        .padding(8)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.05, 0.05, 0.05))),
                border: iced::Border {
                    color: iced::Color::from_rgb(0.2, 0.2, 0.2),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn create_actions(&self) -> Element<WorkflowExecutorMessage> {
        let can_execute = self.validation_errors.is_empty() && self.all_required_arguments_filled();

        row![
            button("Save as Template")
                .on_press(WorkflowExecutorMessage::SaveAsTemplate),
            button("Cancel")
                .on_press(WorkflowExecutorMessage::Cancel),
            button("Execute")
                .on_press(WorkflowExecutorMessage::Execute)
                .style(move |theme: &iced::Theme, status| {
                    button::Appearance {
                        background: Some(iced::Background::Color(
                            if can_execute {
                                Color::from_rgb(0.2, 0.6, 0.2)
                            } else {
                                Color::from_rgb(0.3, 0.3, 0.3)
                            }
                        )),
                        text_color: if can_execute {
                            Color::WHITE
                        } else {
                            Color::from_rgb(0.6, 0.6, 0.6)
                        },
                        ..Default::default()
                    }
                }),
        ]
        .spacing(8)
        .align_items(Alignment::Center)
        .into()
    }

    fn update_resolved_command(&mut self) {
        if let Some(workflow) = &self.workflow {
            self.resolved_command = workflow.command.clone();
            
            if let Some(arguments) = &workflow.arguments {
                for arg in arguments {
                    let placeholder = format!("{{{{{}}}}}", arg.name);
                    let value = self.argument_values.get(&arg.name)
                        .or(arg.default_value.as_ref())
                        .unwrap_or(&format!("{{{{ {} }}}}", arg.name));
                    
                    self.resolved_command = self.resolved_command.replace(&placeholder, value);
                }
            }
        }
    }

    fn validate_argument(&mut self, arg_name: &str) {
        if let Some(workflow) = &self.workflow {
            if let Some(arguments) = &workflow.arguments {
                if let Some(arg) = arguments.iter().find(|a| a.name == arg_name) {
                    let value = self.argument_values.get(arg_name).unwrap_or(&String::new());
                    
                    // Check if required argument is empty
                    if arg.required.unwrap_or(false) && value.is_empty() {
                        self.validation_errors.insert(arg_name.to_string(), "This field is required".to_string());
                        return;
                    }
                    
                    // Type-specific validation
                    if let Some(arg_type) = &arg.argument_type {
                        match arg_type {
                            ArgumentType::Number => {
                                if !value.is_empty() && value.parse::<f64>().is_err() {
                                    self.validation_errors.insert(arg_name.to_string(), "Must be a valid number".to_string());
                                    return;
                                }
                            }
                            ArgumentType::Email => {
                                if !value.is_empty() && !value.contains('@') {
                                    self.validation_errors.insert(arg_name.to_string(), "Must be a valid email address".to_string());
                                    return;
                                }
                            }
                            ArgumentType::Url => {
                                if !value.is_empty() && !value.starts_with("http") {
                                    self.validation_errors.insert(arg_name.to_string(), "Must be a valid URL".to_string());
                                    return;
                                }
                            }
                            _ => {}
                        }
                    }
                    
                    // Validation rules
                    if let Some(validation) = &arg.validation {
                        if let Some(min_len) = validation.min_length {
                            if value.len() < min_len {
                                self.validation_errors.insert(arg_name.to_string(), format!("Must be at least {} characters", min_len));
                                return;
                            }
                        }
                        
                        if let Some(max_len) = validation.max_length {
                            if value.len() > max_len {
                                self.validation_errors.insert(arg_name.to_string(), format!("Must be at most {} characters", max_len));
                                return;
                            }
                        }
                    }
                    
                    // Remove error if validation passes
                    self.validation_errors.remove(arg_name);
                }
            }
        }
    }

    fn validate_all_arguments(&mut self) -> bool {
        if let Some(workflow) = &self.workflow {
            if let Some(arguments) = &workflow.arguments {
                for arg in arguments {
                    self.validate_argument(&arg.name);
                }
            }
        }
        
        self.validation_errors.is_empty()
    }

    fn all_required_arguments_filled(&self) -> bool {
        if let Some(workflow) = &self.workflow {
            if let Some(arguments) = &workflow.arguments {
                for arg in arguments {
                    if arg.required.unwrap_or(false) {
                        let value = self.argument_values.get(&arg.name).unwrap_or(&String::new());
                        if value.is_empty() {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
