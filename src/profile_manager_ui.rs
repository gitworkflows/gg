use iced::{Element, widget::{
    button, column, container, text, text_input, row, scrollable, 
    pick_list, checkbox, image, tooltip
}};
use iced::{Alignment, Length, Color};
use uuid::Uuid;
use std::path::PathBuf;

use crate::terminal::Message;
use crate::profiles::{UserProfile, AutoSwitchRule, SwitchCondition};

#[derive(Debug, Clone)]
pub enum ProfileManagerMessage {
    ProfileSelected(Uuid),
    CreateProfile,
    DuplicateProfile(Uuid),
    DeleteProfile(Uuid),
    EditProfile(Uuid),
    ExportProfile(Uuid),
    ImportProfile,
    ProfileNameChanged(String),
    ProfileDescriptionChanged(String),
    ProfileTagsChanged(String),
    AvatarSelected(PathBuf),
    SaveProfile,
    CancelEdit,
    AddToQuickSwitch(Uuid),
    RemoveFromQuickSwitch(Uuid),
    CreateAutoSwitchRule,
    EditAutoSwitchRule(Uuid),
    DeleteAutoSwitchRule(Uuid),
    ToggleAutoSwitchRule(Uuid, bool),
    SearchChanged(String),
    FilterByTag(String),
    SortChanged(ProfileSortOrder),
    Close,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProfileSortOrder {
    Name,
    LastUsed,
    Created,
}

pub struct ProfileManagerUI {
    profiles: Vec<UserProfile>,
    active_profile_id: Uuid,
    quick_switch_profiles: Vec<Uuid>,
    auto_switch_rules: Vec<AutoSwitchRule>,
    
    // UI state
    is_visible: bool,
    editing_profile: Option<UserProfile>,
    search_query: String,
    selected_tag_filter: Option<String>,
    sort_order: ProfileSortOrder,
    
    // Form state
    profile_name: String,
    profile_description: String,
    profile_tags: String,
    selected_avatar: Option<PathBuf>,
}

impl ProfileManagerUI {
    pub fn new() -> Self {
        ProfileManagerUI {
            profiles: vec![],
            active_profile_id: Uuid::new_v4(),
            quick_switch_profiles: vec![],
            auto_switch_rules: vec![],
            
            is_visible: false,
            editing_profile: None,
            search_query: String::new(),
            selected_tag_filter: None,
            sort_order: ProfileSortOrder::LastUsed,
            
            profile_name: String::new(),
            profile_description: String::new(),
            profile_tags: String::new(),
            selected_avatar: None,
        }
    }

    pub fn show(&mut self) {
        self.is_visible = true;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.editing_profile = None;
        self.clear_form();
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn update_profiles(&mut self, profiles: Vec<UserProfile>, active_id: Uuid, quick_switch: Vec<Uuid>) {
        self.profiles = profiles;
        self.active_profile_id = active_id;
        self.quick_switch_profiles = quick_switch;
    }

    pub fn update(&mut self, message: ProfileManagerMessage) -> Option<Message> {
        match message {
            ProfileManagerMessage::ProfileSelected(id) => {
                Some(Message::SwitchProfile(id))
            }
            
            ProfileManagerMessage::CreateProfile => {
                self.editing_profile = Some(UserProfile::default());
                self.clear_form();
                None
            }
            
            ProfileManagerMessage::DuplicateProfile(id) => {
                Some(Message::DuplicateProfile(id))
            }
            
            ProfileManagerMessage::DeleteProfile(id) => {
                Some(Message::DeleteProfile(id))
            }
            
            ProfileManagerMessage::EditProfile(id) => {
                if let Some(profile) = self.profiles.iter().find(|p| p.id == id) {
                    self.editing_profile = Some(profile.clone());
                    self.profile_name = profile.name.clone();
                    self.profile_description = profile.description.clone().unwrap_or_default();
                    self.profile_tags = profile.tags.join(", ");
                    self.selected_avatar = profile.avatar_path.clone();
                }
                None
            }
            
            ProfileManagerMessage::ExportProfile(id) => {
                Some(Message::ExportProfile(id))
            }
            
            ProfileManagerMessage::ImportProfile => {
                Some(Message::ImportProfile)
            }
            
            ProfileManagerMessage::ProfileNameChanged(name) => {
                self.profile_name = name;
                None
            }
            
            ProfileManagerMessage::ProfileDescriptionChanged(desc) => {
                self.profile_description = desc;
                None
            }
            
            ProfileManagerMessage::ProfileTagsChanged(tags) => {
                self.profile_tags = tags;
                None
            }
            
            ProfileManagerMessage::AvatarSelected(path) => {
                self.selected_avatar = Some(path);
                None
            }
            
            ProfileManagerMessage::SaveProfile => {
                if let Some(mut profile) = self.editing_profile.take() {
                    profile.name = self.profile_name.clone();
                    profile.description = if self.profile_description.is_empty() {
                        None
                    } else {
                        Some(self.profile_description.clone())
                    };
                    profile.tags = self.profile_tags
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    profile.avatar_path = self.selected_avatar.clone();
                    
                    self.clear_form();
                    return Some(Message::SaveProfile(profile));
                }
                None
            }
            
            ProfileManagerMessage::CancelEdit => {
                self.editing_profile = None;
                self.clear_form();
                None
            }
            
            ProfileManagerMessage::AddToQuickSwitch(id) => {
                Some(Message::AddToQuickSwitch(id))
            }
            
            ProfileManagerMessage::RemoveFromQuickSwitch(id) => {
                Some(Message::RemoveFromQuickSwitch(id))
            }
            
            ProfileManagerMessage::SearchChanged(query) => {
                self.search_query = query;
                None
            }
            
            ProfileManagerMessage::FilterByTag(tag) => {
                self.selected_tag_filter = if tag.is_empty() { None } else { Some(tag) };
                None
            }
            
            ProfileManagerMessage::SortChanged(order) => {
                self.sort_order = order;
                None
            }
            
            ProfileManagerMessage::Close => {
                self.hide();
                None
            }
            
            _ => None,
        }
    }

    pub fn view(&self) -> Element<ProfileManagerMessage> {
        if !self.is_visible {
            return container(text("")).into();
        }

        if self.editing_profile.is_some() {
            return self.edit_profile_view();
        }

        let header = self.create_header();
        let filters = self.create_filters();
        let profiles_list = self.create_profiles_list();
        let quick_switch = self.create_quick_switch_section();

        container(
            column![
                header,
                filters,
                scrollable(profiles_list).height(Length::Fixed(400.0)),
                quick_switch,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(Length::Fixed(800.0))
        .height(Length::Fixed(600.0))
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

    fn create_header(&self) -> Element<ProfileManagerMessage> {
        row![
            text("Profile Manager").size(20),
            row![
                button("New Profile")
                    .on_press(ProfileManagerMessage::CreateProfile),
                button("Import")
                    .on_press(ProfileManagerMessage::ImportProfile),
                button("Close")
                    .on_press(ProfileManagerMessage::Close),
            ]
            .spacing(8)
        ]
        .align_items(Alignment::Center)
        .spacing(16)
        .into()
    }

    fn create_filters(&self) -> Element<ProfileManagerMessage> {
        let sort_options = vec![
            ProfileSortOrder::Name,
            ProfileSortOrder::LastUsed,
            ProfileSortOrder::Created,
        ];

        row![
            text_input("Search profiles...", &self.search_query)
                .on_input(ProfileManagerMessage::SearchChanged)
                .width(Length::Fixed(200.0)),
            text("Sort by:"),
            pick_list(
                sort_options,
                Some(self.sort_order.clone()),
                ProfileManagerMessage::SortChanged
            ),
        ]
        .spacing(8)
        .align_items(Alignment::Center)
        .into()
    }

    fn create_profiles_list(&self) -> Element<ProfileManagerMessage> {
        let mut filtered_profiles = self.get_filtered_profiles();
        self.sort_profiles(&mut filtered_profiles);

        let profile_items: Vec<Element<ProfileManagerMessage>> = filtered_profiles
            .into_iter()
            .map(|profile| self.create_profile_item(profile))
            .collect();

        column(profile_items)
            .spacing(8)
            .into()
    }

    fn create_profile_item(&self, profile: &UserProfile) -> Element<ProfileManagerMessage> {
        let is_active = profile.id == self.active_profile_id;
        let is_quick_switch = self.quick_switch_profiles.contains(&profile.id);

        let avatar = if let Some(avatar_path) = &profile.avatar_path {
            // In a real implementation, you'd load the image
            container(text("👤")).width(Length::Fixed(40.0)).height(Length::Fixed(40.0))
        } else {
            container(text("👤")).width(Length::Fixed(40.0)).height(Length::Fixed(40.0))
        };

        let profile_info = column![
            row![
                text(&profile.name).size(16),
                if is_active {
                    text("(Active)").size(12).color(Color::from_rgb(0.0, 0.8, 0.0))
                } else {
                    text("").size(12)
                }
            ]
            .spacing(8),
            if let Some(desc) = &profile.description {
                text(desc).size(12).color(Color::from_rgb(0.7, 0.7, 0.7))
            } else {
                text("").size(12)
            },
            text(format!("Last used: {}", profile.last_used.format("%Y-%m-%d %H:%M")))
                .size(10)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
        ]
        .spacing(2);

        let tags = if !profile.tags.is_empty() {
            row(
                profile.tags
                    .iter()
                    .map(|tag| {
                        container(text(tag).size(10))
                            .padding(2)
                            .style(|theme: &iced::Theme| {
                                container::Appearance {
                                    background: Some(iced::Background::Color(iced::Color::from_rgb(0.2, 0.4, 0.8))),
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

        let actions = row![
            if !is_active {
                button("Switch")
                    .on_press(ProfileManagerMessage::ProfileSelected(profile.id))
            } else {
                button("Switch").style(|theme: &iced::Theme, status| {
                    button::Appearance {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.3, 0.3, 0.3))),
                        ..Default::default()
                    }
                })
            },
            button("Edit")
                .on_press(ProfileManagerMessage::EditProfile(profile.id)),
            button("Duplicate")
                .on_press(ProfileManagerMessage::DuplicateProfile(profile.id)),
            if !profile.is_default {
                button("Delete")
                    .on_press(ProfileManagerMessage::DeleteProfile(profile.id))
                    .style(|theme: &iced::Theme, status| {
                        button::Appearance {
                            background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))),
                            ..Default::default()
                        }
                    })
            } else {
                button("Delete").style(|theme: &iced::Theme, status| {
                    button::Appearance {
                        background: Some(iced::Background::Color(iced::Color::from_rgb(0.3, 0.3, 0.3))),
                        ..Default::default()
                    }
                })
            },
            button("Export")
                .on_press(ProfileManagerMessage::ExportProfile(profile.id)),
            if is_quick_switch {
                button("Remove from Quick")
                    .on_press(ProfileManagerMessage::RemoveFromQuickSwitch(profile.id))
            } else {
                button("Add to Quick")
                    .on_press(ProfileManagerMessage::AddToQuickSwitch(profile.id))
            },
        ]
        .spacing(4);

        container(
            row![
                avatar,
                column![
                    profile_info,
                    tags,
                ]
                .spacing(4)
                .width(Length::Fill),
                actions,
            ]
            .spacing(12)
            .align_items(Alignment::Center)
        )
        .padding(12)
        .style(move |theme: &iced::Theme| {
            container::Appearance {
                background: Some(iced::Background::Color(
                    if is_active {
                        iced::Color::from_rgb(0.15, 0.25, 0.15)
                    } else {
                        iced::Color::from_rgb(0.05, 0.05, 0.05)
                    }
                )),
                border: iced::Border {
                    color: if is_active {
                        iced::Color::from_rgb(0.0, 0.8, 0.0)
                    } else {
                        iced::Color::from_rgb(0.2, 0.2, 0.2)
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

    fn create_quick_switch_section(&self) -> Element<ProfileManagerMessage> {
        let quick_profiles: Vec<Element<ProfileManagerMessage>> = self.quick_switch_profiles
            .iter()
            .filter_map(|id| self.profiles.iter().find(|p| p.id == *id))
            .map(|profile| {
                button(&profile.name)
                    .on_press(ProfileManagerMessage::ProfileSelected(profile.id))
                    .into()
            })
            .collect();

        container(
            column![
                text("Quick Switch").size(16),
                if quick_profiles.is_empty() {
                    text("No profiles in quick switch").color(Color::from_rgb(0.5, 0.5, 0.5)).into()
                } else {
                    row(quick_profiles).spacing(8).into()
                }
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

    fn edit_profile_view(&self) -> Element<ProfileManagerMessage> {
        let form = column![
            text("Edit Profile").size(18),
            
            text("Name:"),
            text_input("Profile name", &self.profile_name)
                .on_input(ProfileManagerMessage::ProfileNameChanged),
            
            text("Description:"),
            text_input("Profile description", &self.profile_description)
                .on_input(ProfileManagerMessage::ProfileDescriptionChanged),
            
            text("Tags (comma-separated):"),
            text_input("work, development, personal", &self.profile_tags)
                .on_input(ProfileManagerMessage::ProfileTagsChanged),
            
            row![
                button("Save")
                    .on_press(ProfileManagerMessage::SaveProfile),
                button("Cancel")
                    .on_press(ProfileManagerMessage::CancelEdit),
            ]
            .spacing(8)
        ]
        .spacing(8);

        container(form)
            .padding(16)
            .width(Length::Fixed(400.0))
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

    fn get_filtered_profiles(&self) -> Vec<&UserProfile> {
        self.profiles
            .iter()
            .filter(|profile| {
                // Search filter
                if !self.search_query.is_empty() {
                    let query = self.search_query.to_lowercase();
                    let matches_name = profile.name.to_lowercase().contains(&query);
                    let matches_desc = profile.description
                        .as_ref()
                        .map_or(false, |desc| desc.to_lowercase().contains(&query));
                    let matches_tags = profile.tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query));
                    
                    if !matches_name && !matches_desc && !matches_tags {
                        return false;
                    }
                }

                // Tag filter
                if let Some(tag_filter) = &self.selected_tag_filter {
                    if !profile.tags.contains(tag_filter) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    fn sort_profiles(&self, profiles: &mut Vec<&UserProfile>) {
        match self.sort_order {
            ProfileSortOrder::Name => {
                profiles.sort_by(|a, b| a.name.cmp(&b.name));
            }
            ProfileSortOrder::LastUsed => {
                profiles.sort_by(|a, b| b.last_used.cmp(&a.last_used));
            }
            ProfileSortOrder::Created => {
                profiles.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            }
        }
    }

    fn clear_form(&mut self) {
        self.profile_name.clear();
        self.profile_description.clear();
        self.profile_tags.clear();
        self.selected_avatar = None;
    }
}

impl std::fmt::Display for ProfileSortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileSortOrder::Name => write!(f, "Name"),
            ProfileSortOrder::LastUsed => write!(f, "Last Used"),
            ProfileSortOrder::Created => write!(f, "Created"),
        }
    }
}
