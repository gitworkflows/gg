use iced::{
    widget::{column, row, text, button},
    Element, Length,
};
use uuid::Uuid;

use crate::profiles::UserProfile;
use crate::themes::WarpTheme; // Assuming WarpTheme is accessible

#[derive(Debug, Clone)]
pub enum ProfileSwitcherMessage {
    SwitchProfile(Uuid),
    OpenProfileManager,
}

pub struct ProfileSwitcher {
    quick_switch_profiles: Vec<UserProfile>,
    active_profile_id: Uuid,
}

impl ProfileSwitcher {
    pub fn new() -> Self {
        Self {
            quick_switch_profiles: Vec::new(),
            active_profile_id: Uuid::new_v4(), // Dummy initial
        }
    }

    pub fn update_profiles(&mut self, profiles: Vec<UserProfile>, active_id: Uuid) {
        self.quick_switch_profiles = profiles;
        self.active_profile_id = active_id;
    }

    pub fn view(&self) -> Element<ProfileSwitcherMessage> {
        let theme = WarpTheme::default_dark(); // Use a default theme for the switcher UI
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let accent_color = theme.get_accent_color();

        let profile_buttons = self.quick_switch_profiles.iter().map(|profile| {
            let is_active = profile.id == self.active_profile_id;
            button(text(&profile.name).color(if is_active { iced::Color::BLACK } else { foreground_color }))
                .on_press(ProfileSwitcherMessage::SwitchProfile(profile.id))
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(if is_active { accent_color } else { background_color })),
                    border_radius: 4.0.into(),
                    text_color: if is_active { iced::Color::BLACK } else { foreground_color },
                    ..Default::default()
                })
                .into()
        }).collect();

        row![
            column![
                text("Quick Switch Profiles:").size(16).color(foreground_color),
                row(profile_buttons).spacing(5),
            ]
            .spacing(5),
            button("Manage Profiles")
                .on_press(ProfileSwitcherMessage::OpenProfileManager)
                .style(iced::widget::button::text::Appearance {
                    background: Some(iced::Background::Color(background_color)),
                    border_radius: 4.0.into(),
                    text_color: foreground_color,
                    ..Default::default()
                }),
        ]
        .spacing(10)
        .align_items(iced::Alignment::Center)
        .into()
    }
}
