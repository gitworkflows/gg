use iced::{Element, widget::{button, row, container, text, tooltip}};
use iced::{Alignment, Length, Color};
use uuid::Uuid;

use crate::terminal::Message;
use crate::profiles::UserProfile;

#[derive(Debug, Clone)]
pub enum ProfileSwitcherMessage {
    SwitchProfile(Uuid),
    OpenProfileManager,
}

pub struct ProfileSwitcher {
    quick_switch_profiles: Vec<UserProfile>,
    active_profile_id: Uuid,
    is_visible: bool,
}

impl ProfileSwitcher {
    pub fn new() -> Self {
        ProfileSwitcher {
            quick_switch_profiles: vec![],
            active_profile_id: Uuid::new_v4(),
            is_visible: true,
        }
    }

    pub fn update_profiles(&mut self, profiles: Vec<UserProfile>, active_id: Uuid) {
        self.quick_switch_profiles = profiles;
        self.active_profile_id = active_id;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn view(&self) -> Element<ProfileSwitcherMessage> {
        if !self.is_visible || self.quick_switch_profiles.is_empty() {
            return container(text("")).into();
        }

        let profile_buttons: Vec<Element<ProfileSwitcherMessage>> = self.quick_switch_profiles
            .iter()
            .map(|profile| {
                let is_active = profile.id == self.active_profile_id;
                
                let button_content = if let Some(avatar_path) = &profile.avatar_path {
                    // In a real implementation, you'd load the image
                    text("👤")
                } else {
                    text(&profile.name.chars().take(2).collect::<String>().to_uppercase())
                };

                tooltip(
                    button(button_content)
                        .on_press(ProfileSwitcherMessage::SwitchProfile(profile.id))
                        .style(move |theme: &iced::Theme, status| {
                            button::Appearance {
                                background: Some(iced::Background::Color(
                                    if is_active {
                                        Color::from_rgb(0.2, 0.6, 0.2)
                                    } else {
                                        Color::from_rgb(0.3, 0.3, 0.3)
                                    }
                                )),
                                border: iced::Border {
                                    color: if is_active {
                                        Color::from_rgb(0.0, 0.8, 0.0)
                                    } else {
                                        Color::from_rgb(0.5, 0.5, 0.5)
                                    },
                                    width: if is_active { 2.0 } else { 1.0 },
                                    radius: 20.0.into(),
                                },
                                text_color: Color::WHITE,
                                ..Default::default()
                            }
                        })
                        .width(Length::Fixed(40.0))
                        .height(Length::Fixed(40.0)),
                    &profile.name,
                    tooltip::Position::Bottom
                ).into()
            })
            .collect();

        let manage_button = tooltip(
            button("⚙")
                .on_press(ProfileSwitcherMessage::OpenProfileManager)
                .style(|theme: &iced::Theme, status| {
                    button::Appearance {
                        background: Some(iced::Background::Color(Color::from_rgb(0.4, 0.4, 0.4))),
                        border: iced::Border {
                            color: Color::from_rgb(0.6, 0.6, 0.6),
                            width: 1.0,
                            radius: 20.0.into(),
                        },
                        text_color: Color::WHITE,
                        ..Default::default()
                    }
                })
                .width(Length::Fixed(40.0))
                .height(Length::Fixed(40.0)),
            "Manage Profiles",
            tooltip::Position::Bottom
        );

        container(
            row![
                row(profile_buttons).spacing(4),
                manage_button,
            ]
            .spacing(8)
            .align_items(Alignment::Center)
        )
        .padding(4)
        .style(|theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(Color::from_rgba(0.1, 0.1, 0.1, 0.9))),
                border: iced::Border {
                    color: Color::from_rgb(0.3, 0.3, 0.3),
                    width: 1.0,
                    radius: 25.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }
}
