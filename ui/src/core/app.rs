//! Core application logic for the UI.

use iced::{Application, Command, Element, Settings, Theme};
use log::info;

pub struct CoreApp {
    value: i32,
}

#[derive(Debug, Clone)]
pub enum CoreMessage {
    IncrementPressed,
    DecrementPressed,
}

impl Application for CoreApp {
    type Executor = iced::executor::Default;
    type Message = CoreMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (CoreApp, Command<CoreMessage>) {
        (CoreApp { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Warp Terminal UI Core")
    }

    fn update(&mut self, message: CoreMessage) -> Command<CoreMessage> {
        info!("Core UI application is updating.");
        match message {
            CoreMessage::IncrementPressed => {
                self.value += 1;
            }
            CoreMessage::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<CoreMessage> {
        info!("Core UI application is rendering.");
        iced::widget::column![
            iced::widget::button("Increment").on_press(CoreMessage::IncrementPressed),
            iced::widget::text(self.value).size(50),
            iced::widget::button("Decrement").on_press(CoreMessage::DecrementPressed),
        ]
        .into()
    }
}

impl CoreApp {
    pub fn new() -> Self {
        CoreApp { value: 0 }
    }

    pub fn run(&self) {
        info!("Core UI application is running.");
        // Placeholder for core UI logic, e.g., initializing main window, event loop
    }
}
