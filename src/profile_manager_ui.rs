use iced::{Element, widget::{
    button, column, container, text, text_input, row, scrollable, 
    pick_list, checkbox, image, tooltip, Space
}};
use iced::{Alignment, Length, Color};
use uuid::Uuid;
use std::path::PathBuf;

use crate::terminal::Message;
use crate::profiles::{UserProfile};
use crate::themes::WarpTheme; // Assuming WarpTheme is accessible

#[derive(Debug, Clone)]
pub enum ProfileManagerMessage {
    ToggleVisibility,
    NewProfileNameChanged(String),
    NewProfileDescriptionChanged(String),
    CreateProfileClicked,
    SelectProfile(Uuid),
    DuplicateProfileClicked(Uuid),
    DeleteProfileClicked(Uuid),
    ToggleQuickSwitch(Uuid),
    // Add messages for editing profile details, auto-switch rules, etc.
}

pub struct ProfileManagerUI {
    is_visible: bool,
    profiles: Vec<UserProfile>,
    active_profile_id: Uuid,
    quick_switch_profile_ids: Vec<Uuid>,
    new_profile_name: String,
    new_profile_description: String,
    // Add state for editing selected profile
}

impl ProfileManagerUI {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            profiles: Vec::new(),
            active_profile_id: Uuid::new_v4(), // Dummy initial
            quick_switch_profile_ids: Vec::new(),
            new_profile_name: String::new(),
            new_profile_description: String::new(),
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

    pub fn update_profiles(&mut self, profiles: Vec<UserProfile>, active_id: Uuid, quick_switch_ids: Vec<Uuid>) {
        self.profiles = profiles;
        self.active_profile_id = active_id;
        self.quick_switch_profile_ids = quick_switch_ids;
    }

    pub fn update(&mut self, message: ProfileManagerMessage) -> Option<Message> {
        match message {
            ProfileManagerMessage::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                None
            }
            ProfileManagerMessage::NewProfileNameChanged(name) => {
                self.new_profile_name = name;
                None
            }
            ProfileManagerMessage::NewProfileDescriptionChanged(desc) => {
                self.new_profile_description = desc;
                None
            }
            ProfileManagerMessage::CreateProfileClicked => {
                if !self.new_profile_name.is_empty() {
                    let name = self.new_profile_name.clone();
                    let description = if self.new_profile_description.is_empty() {
                        None
                    } else {
                        Some(self.new_profile_description.clone())
                    };
                    self.new_profile_name.clear();
                    self.new_profile_description.clear();
                    return Some(Message::CreateProfile(name, description));
                }
                None
            }
            ProfileManagerMessage::SelectProfile(id) => {
                Some(Message::SwitchProfile(id))
            }
            ProfileManagerMessage::DuplicateProfileClicked(id) => {
                Some(Message::DuplicateProfile(id))
            }
            ProfileManagerMessage::DeleteProfileClicked(id) => {
                Some(Message::DeleteProfile(id))
            }
            ProfileManagerMessage::ToggleQuickSwitch(id) => {
                if self.quick_switch_profile_ids.contains(&id) {
                    Some(Message::RemoveFromQuickSwitch(id))
                } else {
                    Some(Message::AddToQuickSwitch(id))
                }
            }
        }
    }

    pub fn view(&self) -> Element<ProfileManagerMessage> {
        let theme = WarpTheme::default_dark(); // Use a default theme for the manager UI
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let profile_list = scrollable(
            column(
                self.profiles.iter().map(|profile| {
                    let is_active = profile.id == self.active_profile_id;
                    let is_quick_switch = self.quick_switch_profile_ids.contains(&profile.id);

                    row![
                        button(text(&profile.name).color(foreground_color))
                            .on_press(ProfileManagerMessage::SelectProfile(profile.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(if is_active { accent_color } else { background_color })),
                                border_radius: 4.0.into(),
                                text_color: if is_active { Color::BLACK } else { foreground_color },
                                ..Default::default()
                            })
                            .width(Length::Fill),
                        button(text(if is_quick_switch { "Remove from Quick Switch" } else { "Add to Quick Switch" }).color(foreground_color))
                            .on_press(ProfileManagerMessage::ToggleQuickSwitch(profile.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(background_color)),
                                border_radius: 4.0.into(),
                                text_color: foreground_color,
                                ..Default::default()
                            }),
                        button(text("Duplicate").color(foreground_color))
                            .on_press(ProfileManagerMessage::DuplicateProfileClicked(profile.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(background_color)),
                                border_radius: 4.0.into(),
                                text_color: foreground_color,
                                ..Default::default()
                            }),
                        button(text("Delete").color(foreground_color))
                            .on_press(ProfileManagerMessage::DeleteProfileClicked(profile.id))
                            .style(iced::widget::button::text::Appearance {
                                background: Some(iced::Background::Color(background_color)),
                                border_radius: 4.0.into(),
                                text_color: foreground_color,
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

        let new_profile_section = column![
            text("Create New Profile").size(20).color(foreground_color),
            text_input("Profile Name", &self.new_profile_name)
                .on_input(ProfileManagerMessage::NewProfileNameChanged)
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
            text_input("Description (Optional)", &self.new_profile_description)
                .on_input(ProfileManagerMessage::NewProfileDescriptionChanged)
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
            button("Create Profile")
                .on_press(ProfileManagerMessage::CreateProfileClicked)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(accent_color)),
                    border_radius: 4.0.into(),
                    text_color: Color::BLACK,
                    ..Default::default()
                })
                .width(Length::Fill),
        ]
        .spacing(10)
        .padding(10)
        .style(move |_theme: &iced::Theme| iced::widget::container::Appearance {
            background: Some(iced::Background::Color(background_color)),
            border_color,
            border_width: 1.0,
            border_radius: 4.0.into(),
            ..Default::default()
        });

        container(
            column![
                row![
                    text("Profile Manager").size(28).width(Length::Fill).color(foreground_color),
                    button("Close").on_press(ProfileManagerMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            border_radius: 4.0.into(),
                            text_color: Color::WHITE,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                profile_list,
                Space::with_height(Length::Fixed(20.0)),
                new_profile_section,
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fixed(800.0))
        .height(Length::Fixed(600.0))
        .center_x()
        .center_y()
        .style(move |_theme: &iced::Theme| iced::widget::container::Appearance {
            background: Some(iced::Background::Color(theme.get_background_color())),
            border_color: theme.get_border_color(),
            border_width: 2.0,
            border_radius: 8.0.into(),
            ..Default::default()
        })
        .into()
    }
}
