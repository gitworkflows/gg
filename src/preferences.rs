use iced::{
    widget::{column, row, text, button, checkbox, text_input, Space},
    Element, Length, Color,
};
use crate::config::{UserPreferences, KeyBindings, PromptSettings};
use crate::terminal::Message;
use crate::themes::WarpTheme; // Assuming WarpTheme is accessible

#[derive(Debug, Clone)]
pub enum PreferencesMessage {
    ToggleVisibility,
    // User Preferences
    ToggleFuzzySearch(bool),
    ToggleCollaboration(bool),
    ToggleWelcomeMessage(bool),
    MaxHistorySizeChanged(String),
    ToggleAutoUpdate(bool),
    // Font Settings
    FontSizeChanged(String),
    FontFamilyChanged(String),
    // Shell Settings
    ShellPathChanged(String),
    // Keybindings
    SubmitInputKeyChanged(String),
    HistoryUpKeyChanged(String),
    HistoryDownKeyChanged(String),
    ClearTerminalKeyChanged(String),
    ToggleFullscreenKeyChanged(String),
    OpenCommandPaletteKeyChanged(String),
    OpenPreferencesKeyChanged(String),
    OpenThemeCustomizerKeyChanged(String),
    OpenProfileManagerKeyChanged(String),
    OpenWorkflowBrowserKeyChanged(String),
    OpenWarpDriveKeyChanged(String), // New message for Warp Drive keybinding
    // Prompt Settings
    ToggleShowUser(bool),
    ToggleShowHost(bool),
    ToggleShowCwd(bool),
    ToggleShowGitStatus(bool),
    UserSymbolChanged(String),
    HostSymbolChanged(String),
    CwdSymbolChanged(String),
    GitSymbolChanged(String),
    PromptSymbolChanged(String),
    // Save
    SavePreferences,
}

pub struct PreferencesWindow {
    is_visible: bool,
    preferences: UserPreferences,
    font_family: String,
    font_size: String, // Stored as String for text input
    shell: String,
    keybindings: KeyBindings,
    prompt_settings: PromptSettings,
}

impl PreferencesWindow {
    pub fn new(
        preferences: UserPreferences,
        font_family: String,
        font_size: u16,
        shell: String,
        keybindings: KeyBindings,
        prompt_settings: PromptSettings,
    ) -> Self {
        Self {
            is_visible: false,
            preferences,
            font_family,
            font_size: font_size.to_string(),
            shell,
            keybindings,
            prompt_settings,
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

    pub fn update(&mut self, message: PreferencesMessage) -> Option<Message> {
        match message {
            PreferencesMessage::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                None
            }
            PreferencesMessage::ToggleFuzzySearch(b) => {
                self.preferences.enable_fuzzy_search = b;
                None
            }
            PreferencesMessage::ToggleCollaboration(b) => {
                self.preferences.enable_collaboration = b;
                None
            }
            PreferencesMessage::ToggleWelcomeMessage(b) => {
                self.preferences.show_welcome_message = b;
                None
            }
            PreferencesMessage::MaxHistorySizeChanged(s) => {
                if let Ok(size) = s.parse::<usize>() {
                    self.preferences.max_history_size = size;
                }
                None
            }
            PreferencesMessage::ToggleAutoUpdate(b) => {
                self.preferences.enable_auto_update = b;
                None
            }
            PreferencesMessage::FontSizeChanged(s) => {
                self.font_size = s;
                None
            }
            PreferencesMessage::FontFamilyChanged(s) => {
                self.font_family = s;
                None
            }
            PreferencesMessage::ShellPathChanged(s) => {
                self.shell = s;
                None
            }
            PreferencesMessage::SubmitInputKeyChanged(s) => {
                self.keybindings.submit_input = s;
                None
            }
            PreferencesMessage::HistoryUpKeyChanged(s) => {
                self.keybindings.history_up = s;
                None
            }
            PreferencesMessage::HistoryDownKeyChanged(s) => {
                self.keybindings.history_down = s;
                None
            }
            PreferencesMessage::ClearTerminalKeyChanged(s) => {
                self.keybindings.clear_terminal = s;
                None
            }
            PreferencesMessage::ToggleFullscreenKeyChanged(s) => {
                self.keybindings.toggle_fullscreen = s;
                None
            }
            PreferencesMessage::OpenCommandPaletteKeyChanged(s) => {
                self.keybindings.open_command_palette = s;
                None
            }
            PreferencesMessage::OpenPreferencesKeyChanged(s) => {
                self.keybindings.open_preferences = s;
                None
            }
            PreferencesMessage::OpenThemeCustomizerKeyChanged(s) => {
                self.keybindings.open_theme_customizer = s;
                None
            }
            PreferencesMessage::OpenProfileManagerKeyChanged(s) => {
                self.keybindings.open_profile_manager = s;
                None
            }
            PreferencesMessage::OpenWorkflowBrowserKeyChanged(s) => {
                self.keybindings.open_workflow_browser = s;
                None
            }
            PreferencesMessage::OpenWarpDriveKeyChanged(s) => {
                self.keybindings.open_warp_drive = s;
                None
            }
            PreferencesMessage::ToggleShowUser(b) => {
                self.prompt_settings.show_user = b;
                None
            }
            PreferencesMessage::ToggleShowHost(b) => {
                self.prompt_settings.show_host = b;
                None
            }
            PreferencesMessage::ToggleShowCwd(b) => {
                self.prompt_settings.show_cwd = b;
                None
            }
            PreferencesMessage::ToggleShowGitStatus(b) => {
                self.prompt_settings.show_git_status = b;
                None
            }
            PreferencesMessage::UserSymbolChanged(s) => {
                self.prompt_settings.user_symbol = s;
                None
            }
            PreferencesMessage::HostSymbolChanged(s) => {
                self.prompt_settings.host_symbol = s;
                None
            }
            PreferencesMessage::CwdSymbolChanged(s) => {
                self.prompt_settings.cwd_symbol = s;
                None
            }
            PreferencesMessage::GitSymbolChanged(s) => {
                self.prompt_settings.git_symbol = s;
                None
            }
            PreferencesMessage::PromptSymbolChanged(s) => {
                self.prompt_settings.prompt_symbol = s;
                None
            }
            PreferencesMessage::SavePreferences => {
                let font_size_parsed = self.font_size.parse::<u16>().unwrap_or(16);
                Some(Message::PreferencesSaved {
                    preferences: self.preferences.clone(),
                    font_family: self.font_family.clone(),
                    font_size: font_size_parsed,
                    shell: self.shell.clone(),
                    keybindings: self.keybindings.clone(),
                    prompt_settings: self.prompt_settings.clone(),
                })
            }
        }
    }

    pub fn view(&self) -> Element<PreferencesMessage> {
        let theme = WarpTheme::default_dark(); // Use a default theme for the preferences UI
        let background_color = theme.get_block_background_color(theme.is_dark_theme());
        let foreground_color = theme.get_foreground_color();
        let border_color = theme.get_border_color();
        let accent_color = theme.get_accent_color();

        let input_style = iced::widget::text_input::Appearance {
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
        };

        let checkbox_style = iced::widget::checkbox::Appearance {
            background: iced::Background::Color(background_color),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color,
            text_color: foreground_color,
        };

        let section_title = |text: &str| text::text(text).size(20).color(foreground_color);

        let preferences_section = column![
            section_title("General Preferences"),
            checkbox("Enable Fuzzy Search", self.preferences.enable_fuzzy_search)
                .on_toggle(PreferencesMessage::ToggleFuzzySearch)
                .style(checkbox_style),
            checkbox("Enable Collaboration", self.preferences.enable_collaboration)
                .on_toggle(PreferencesMessage::ToggleCollaboration)
                .style(checkbox_style),
            checkbox("Show Welcome Message", self.preferences.show_welcome_message)
                .on_toggle(PreferencesMessage::ToggleWelcomeMessage)
                .style(checkbox_style),
            row![
                text("Max History Size:").color(foreground_color),
                text_input("1000", &self.preferences.max_history_size.to_string())
                    .on_input(PreferencesMessage::MaxHistorySizeChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(100.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            checkbox("Enable Auto Update", self.preferences.enable_auto_update)
                .on_toggle(PreferencesMessage::ToggleAutoUpdate)
                .style(checkbox_style),
        ]
        .spacing(10);

        let font_section = column![
            section_title("Font Settings"),
            row![
                text("Font Family:").color(foreground_color),
                text_input("Fira Code", &self.font_family)
                    .on_input(PreferencesMessage::FontFamilyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Font Size:").color(foreground_color),
                text_input("16", &self.font_size)
                    .on_input(PreferencesMessage::FontSizeChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(100.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(10);

        let shell_section = column![
            section_title("Shell Settings"),
            row![
                text("Shell Path:").color(foreground_color),
                text_input("/bin/bash", &self.shell)
                    .on_input(PreferencesMessage::ShellPathChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(10);

        let keybindings_section = column![
            section_title("Keybindings"),
            row![
                text("Submit Input:").color(foreground_color),
                text_input("Enter", &self.keybindings.submit_input)
                    .on_input(PreferencesMessage::SubmitInputKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("History Up:").color(foreground_color),
                text_input("Up", &self.keybindings.history_up)
                    .on_input(PreferencesMessage::HistoryUpKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("History Down:").color(foreground_color),
                text_input("Down", &self.keybindings.history_down)
                    .on_input(PreferencesMessage::HistoryDownKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Clear Terminal:").color(foreground_color),
                text_input("Ctrl+L", &self.keybindings.clear_terminal)
                    .on_input(PreferencesMessage::ClearTerminalKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Toggle Fullscreen:").color(foreground_color),
                text_input("F11", &self.keybindings.toggle_fullscreen)
                    .on_input(PreferencesMessage::ToggleFullscreenKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Open Command Palette:").color(foreground_color),
                text_input("Ctrl+P", &self.keybindings.open_command_palette)
                    .on_input(PreferencesMessage::OpenCommandPaletteKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Open Preferences:").color(foreground_color),
                text_input("Ctrl+,", &self.keybindings.open_preferences)
                    .on_input(PreferencesMessage::OpenPreferencesKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Open Theme Customizer:").color(foreground_color),
                text_input("Ctrl+T", &self.keybindings.open_theme_customizer)
                    .on_input(PreferencesMessage::OpenThemeCustomizerKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Open Profile Manager:").color(foreground_color),
                text_input("Ctrl+Shift+P", &self.keybindings.open_profile_manager)
                    .on_input(PreferencesMessage::OpenProfileManagerKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Open Workflow Browser:").color(foreground_color),
                text_input("Ctrl+W", &self.keybindings.open_workflow_browser)
                    .on_input(PreferencesMessage::OpenWorkflowBrowserKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Open Warp Drive:").color(foreground_color),
                text_input("Ctrl+Shift+D", &self.keybindings.open_warp_drive)
                    .on_input(PreferencesMessage::OpenWarpDriveKeyChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(10);

        let prompt_section = column![
            section_title("Prompt Settings"),
            checkbox("Show User", self.prompt_settings.show_user)
                .on_toggle(PreferencesMessage::ToggleShowUser)
                .style(checkbox_style),
            row![
                text("User Symbol:").color(foreground_color),
                text_input("👤", &self.prompt_settings.user_symbol)
                    .on_input(PreferencesMessage::UserSymbolChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            checkbox("Show Host", self.prompt_settings.show_host)
                .on_toggle(PreferencesMessage::ToggleShowHost)
                .style(checkbox_style),
            row![
                text("Host Symbol:").color(foreground_color),
                text_input("💻", &self.prompt_settings.host_symbol)
                    .on_input(PreferencesMessage::HostSymbolChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            checkbox("Show CWD", self.prompt_settings.show_cwd)
                .on_toggle(PreferencesMessage::ToggleShowCwd)
                .style(checkbox_style),
            row![
                text("CWD Symbol:").color(foreground_color),
                text_input("📁", &self.prompt_settings.cwd_symbol)
                    .on_input(PreferencesMessage::CwdSymbolChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            checkbox("Show Git Status", self.prompt_settings.show_git_status)
                .on_toggle(PreferencesMessage::ToggleShowGitStatus)
                .style(checkbox_style),
            row![
                text("Git Symbol:").color(foreground_color),
                text_input("🌿", &self.prompt_settings.git_symbol)
                    .on_input(PreferencesMessage::GitSymbolChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
            row![
                text("Prompt Symbol:").color(foreground_color),
                text_input("❯", &self.prompt_settings.prompt_symbol)
                    .on_input(PreferencesMessage::PromptSymbolChanged)
                    .padding(8)
                    .size(16)
                    .style(input_style)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(10);


        container(
            column![
                row![
                    text("Preferences").size(28).width(Length::Fill).color(foreground_color),
                    button("Close").on_press(PreferencesMessage::ToggleVisibility)
                        .style(iced::widget::button::text::Appearance {
                            background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                            border_radius: 4.0.into(),
                            text_color: Color::WHITE,
                            ..Default::default()
                        }),
                ]
                .spacing(10)
                .align_items(iced::Alignment::Center),
                scrollable(
                    column![
                        preferences_section,
                        Space::with_height(Length::Fixed(20.0)),
                        font_section,
                        Space::with_height(Length::Fixed(20.0)),
                        shell_section,
                        Space::with_height(Length::Fixed(20.0)),
                        keybindings_section,
                        Space::with_height(Length::Fixed(20.0)),
                        prompt_section,
                    ]
                    .spacing(20)
                    .padding(10)
                )
                .width(Length::Fill)
                .height(Length::FillPortion(1)),
                button("Save").on_press(PreferencesMessage::SavePreferences)
                    .style(iced::widget::button::text::Appearance {
                        background: Some(iced::Background::Color(accent_color)),
                        border_radius: 4.0.into(),
                        text_color: Color::BLACK,
                        ..Default::default()
                    })
                    .width(Length::Fill),
            ]
            .spacing(20)
            .padding(20)
        )
        .width(Length::Fixed(800.0))
        .height(Length::Fixed(700.0))
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
