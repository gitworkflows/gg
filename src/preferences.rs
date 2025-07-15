use iced::{Element, widget::{
    button, column, container, text, slider, checkbox, text_input, 
    pick_list, row, scrollable, tabs, Tab
}};
use iced::{Alignment, Length};

use crate::terminal::Message;
use crate::config::{UserPreferences, CursorStyle, KeyBindings, NotificationSettings, PerformanceSettings, PromptSettings, PromptStyle};

#[derive(Debug, Clone)]
pub enum PreferencesMessage {
    TabSelected(PreferencesTab),
    FontSizeChanged(u16),
    FontFamilyChanged(String),
    ShellChanged(String),
    AutoSaveToggled(bool),
    ShowTimestampsToggled(bool),
    FuzzySearchToggled(bool),
    HistorySizeChanged(usize),
    ScrollSensitivityChanged(f32),
    AnimationSpeedChanged(f32),
    BlurBackgroundToggled(bool),
    TransparencyChanged(f32),
    CursorStyleChanged(CursorStyle),
    NotificationToggled(bool),
    CommandCompletionToggled(bool),
    ErrorNotificationToggled(bool),
    SoundToggled(bool),
    MaxFpsChanged(u32),
    GpuAccelerationToggled(bool),
    MemoryLimitChanged(usize),
    LazyRenderingToggled(bool),
    BufferSizeChanged(usize),
    KeyBindingChanged(String, String),
    ResetToDefaults,
    Save,
    Cancel,
    PromptTabSelected(PromptTab), // New
    PromptStyleChanged(PromptStyle), // New
    SameLinePromptToggled(bool), // New
    ContextChipToggled(String, bool), // New (chip name, enabled)
    ContextChipMoved(String, usize, usize), // New (chip name, from index, to index)
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreferencesTab {
    General,
    Appearance,
    Performance,
    KeyBindings,
    Notifications,
    Prompt, // Add this line
}

// Define a new enum for Prompt sub-tabs (if needed, for now just one)
#[derive(Debug, Clone, PartialEq)]
pub enum PromptTab {
    General,
    ContextChips,
}

pub struct PreferencesWindow {
    active_tab: PreferencesTab,
    preferences: UserPreferences,
    font_family: String,
    font_size: u16,
    shell: String,
    keybindings: KeyBindings,
    prompt_settings: PromptSettings, // Add this line
    is_visible: bool,
}

impl PreferencesWindow {
    pub fn new(
        preferences: UserPreferences,
        font_family: String,
        font_size: u16,
        shell: String,
        keybindings: KeyBindings,
        prompt_settings: PromptSettings, // Add this line
    ) -> Self {
        PreferencesWindow {
            active_tab: PreferencesTab::General,
            preferences,
            font_family,
            font_size,
            shell,
            keybindings,
            prompt_settings, // Add this line
            is_visible: false,
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
            PreferencesMessage::TabSelected(tab) => {
                self.active_tab = tab;
                None
            }
            PreferencesMessage::FontSizeChanged(size) => {
                self.font_size = size;
                None
            }
            PreferencesMessage::FontFamilyChanged(family) => {
                self.font_family = family;
                None
            }
            PreferencesMessage::ShellChanged(shell) => {
                self.shell = shell;
                None
            }
            PreferencesMessage::AutoSaveToggled(enabled) => {
                self.preferences.auto_save_session = enabled;
                None
            }
            PreferencesMessage::ShowTimestampsToggled(enabled) => {
                self.preferences.show_timestamps = enabled;
                None
            }
            PreferencesMessage::FuzzySearchToggled(enabled) => {
                self.preferences.enable_fuzzy_search = enabled;
                None
            }
            PreferencesMessage::HistorySizeChanged(size) => {
                self.preferences.max_history_size = size;
                None
            }
            PreferencesMessage::ScrollSensitivityChanged(sensitivity) => {
                self.preferences.scroll_sensitivity = sensitivity;
                None
            }
            PreferencesMessage::AnimationSpeedChanged(speed) => {
                self.preferences.animation_speed = speed;
                None
            }
            PreferencesMessage::BlurBackgroundToggled(enabled) => {
                self.preferences.blur_background = enabled;
                None
            }
            PreferencesMessage::TransparencyChanged(transparency) => {
                self.preferences.transparency = transparency;
                None
            }
            PreferencesMessage::CursorStyleChanged(style) => {
                self.preferences.cursor_style = style;
                None
            }
            PreferencesMessage::NotificationToggled(enabled) => {
                self.preferences.notification_settings.enable_notifications = enabled;
                None
            }
            PreferencesMessage::CommandCompletionToggled(enabled) => {
                self.preferences.notification_settings.command_completion = enabled;
                None
            }
            PreferencesMessage::ErrorNotificationToggled(enabled) => {
                self.preferences.notification_settings.error_notifications = enabled;
                None
            }
            PreferencesMessage::SoundToggled(enabled) => {
                self.preferences.notification_settings.sound_enabled = enabled;
                None
            }
            PreferencesMessage::MaxFpsChanged(fps) => {
                self.preferences.performance.max_fps = fps;
                None
            }
            PreferencesMessage::GpuAccelerationToggled(enabled) => {
                self.preferences.performance.gpu_acceleration = enabled;
                None
            }
            PreferencesMessage::MemoryLimitChanged(limit) => {
                self.preferences.performance.memory_limit_mb = limit;
                None
            }
            PreferencesMessage::LazyRenderingToggled(enabled) => {
                self.preferences.performance.lazy_rendering = enabled;
                None
            }
            PreferencesMessage::BufferSizeChanged(size) => {
                self.preferences.performance.buffer_size = size;
                None
            }
            PreferencesMessage::KeyBindingChanged(action, binding) => {
                // Update keybinding based on action
                match action.as_str() {
                    "new_tab" => self.keybindings.new_tab = binding,
                    "close_tab" => self.keybindings.close_tab = binding,
                    "next_tab" => self.keybindings.next_tab = binding,
                    "prev_tab" => self.keybindings.prev_tab = binding,
                    "clear_screen" => self.keybindings.clear_screen = binding,
                    "copy" => self.keybindings.copy = binding,
                    "paste" => self.keybindings.paste = binding,
                    "search" => self.keybindings.search = binding,
                    "preferences" => self.keybindings.preferences = binding,
                    "theme_selector" => self.keybindings.theme_selector = binding,
                    _ => {}
                }
                None
            }
            PreferencesMessage::ResetToDefaults => {
                self.preferences = UserPreferences::default();
                self.keybindings = KeyBindings::default();
                self.font_size = 14;
                self.font_family = "JetBrains Mono".to_string();
                self.shell = "zsh".to_string();
                self.prompt_settings = PromptSettings::default(); // Reset prompt settings
                None
            }
            PreferencesMessage::Save => {
                self.is_visible = false;
                Some(Message::PreferencesSaved {
                    preferences: self.preferences.clone(),
                    font_family: self.font_family.clone(),
                    font_size: self.font_size,
                    shell: self.shell.clone(),
                    keybindings: self.keybindings.clone(),
                    prompt_settings: self.prompt_settings.clone(), // Pass prompt settings
                })
            }
            PreferencesMessage::Cancel => {
                self.is_visible = false;
                None
            }
            // New prompt message handling
            PreferencesMessage::PromptTabSelected(tab) => {
                self.active_tab = PreferencesTab::Prompt; // Ensure we are on the Prompt tab
                // If you had sub-tabs within Prompt, you'd update a sub-tab state here
                None
            }
            PreferencesMessage::PromptStyleChanged(style) => {
                self.prompt_settings.style = style;
                None
            }
            PreferencesMessage::SameLinePromptToggled(enabled) => {
                self.prompt_settings.same_line_prompt = enabled;
                None
            }
            PreferencesMessage::ContextChipToggled(chip_name, enabled) => {
                if enabled {
                    if !self.prompt_settings.context_chips.contains(&chip_name) {
                        self.prompt_settings.context_chips.push(chip_name);
                    }
                } else {
                    self.prompt_settings.context_chips.retain(|c| c != &chip_name);
                }
                None
            }
            PreferencesMessage::ContextChipMoved(chip_name, from_idx, to_idx) => {
                // This is a simplified move. For a real drag-and-drop, you'd need more complex logic.
                if let Some(index) = self.prompt_settings.context_chips.iter().position(|c| c == &chip_name) {
                    if index == from_idx { // Only move if the chip is at the expected 'from' position
                        let chip = self.prompt_settings.context_chips.remove(from_idx);
                        self.prompt_settings.context_chips.insert(to_idx, chip);
                    }
                }
                None
            }
        }
    }

    pub fn view(&self) -> Element<PreferencesMessage> {
        if !self.is_visible {
            return container(text("")).into();
        }

        let tabs = tabs(
            self.active_tab.clone(),
            vec![
                (PreferencesTab::General, Tab::new("General")),
                (PreferencesTab::Appearance, Tab::new("Appearance")),
                (PreferencesTab::Performance, Tab::new("Performance")),
                (PreferencesTab::KeyBindings, Tab::new("Key Bindings")),
                (PreferencesTab::Notifications, Tab::new("Notifications")),
                (PreferencesTab::Prompt, Tab::new("Prompt")), // Add this line
            ],
            PreferencesMessage::TabSelected,
        );

        let content = match self.active_tab {
            PreferencesTab::General => self.general_tab(),
            PreferencesTab::Appearance => self.appearance_tab(),
            PreferencesTab::Performance => self.performance_tab(),
            PreferencesTab::KeyBindings => self.keybindings_tab(),
            PreferencesTab::Notifications => self.notifications_tab(),
            PreferencesTab::Prompt => self.prompt_tab(), // Add this line
        };

        let buttons = row![
            button("Reset to Defaults")
                .on_press(PreferencesMessage::ResetToDefaults),
            button("Cancel")
                .on_press(PreferencesMessage::Cancel),
            button("Save")
                .on_press(PreferencesMessage::Save),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        container(
            column![
                tabs,
                scrollable(content).height(Length::Fill),
                buttons,
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

    fn general_tab(&self) -> Element<PreferencesMessage> {
        column![
            self.create_section("Terminal Settings", vec![
                self.create_text_input("Font Family", &self.font_family, PreferencesMessage::FontFamilyChanged),
                self.create_slider("Font Size", self.font_size as f32, 8.0, 32.0, |v| PreferencesMessage::FontSizeChanged(v as u16)),
                self.create_text_input("Default Shell", &self.shell, PreferencesMessage::ShellChanged),
            ]),
            self.create_section("Session Settings", vec![
                self.create_checkbox("Auto-save session", self.preferences.auto_save_session, PreferencesMessage::AutoSaveToggled),
                self.create_checkbox("Show timestamps", self.preferences.show_timestamps, PreferencesMessage::ShowTimestampsToggled),
                self.create_checkbox("Enable fuzzy search", self.preferences.enable_fuzzy_search, PreferencesMessage::FuzzySearchToggled),
            ]),
            self.create_section("History Settings", vec![
                self.create_slider("Max history size", self.preferences.max_history_size as f32, 100.0, 10000.0, |v| PreferencesMessage::HistorySizeChanged(v as usize)),
            ]),
        ]
        .spacing(16)
        .into()
    }

    fn appearance_tab(&self) -> Element<PreferencesMessage> {
        let cursor_styles = vec![CursorStyle::Block, CursorStyle::Underline, CursorStyle::Beam];
        
        column![
            self.create_section("Visual Effects", vec![
                self.create_checkbox("Blur background", self.preferences.blur_background, PreferencesMessage::BlurBackgroundToggled),
                self.create_slider("Transparency", self.preferences.transparency, 0.1, 1.0, PreferencesMessage::TransparencyChanged),
                self.create_slider("Animation speed", self.preferences.animation_speed, 0.1, 3.0, PreferencesMessage::AnimationSpeedChanged),
                self.create_slider("Scroll sensitivity", self.preferences.scroll_sensitivity, 0.1, 5.0, PreferencesMessage::ScrollSensitivityChanged),
            ]),
            self.create_section("Cursor", vec![
                container(
                    row![
                        text("Cursor Style:").width(Length::Fixed(120.0)),
                        pick_list(
                            cursor_styles,
                            Some(self.preferences.cursor_style.clone()),
                            PreferencesMessage::CursorStyleChanged
                        )
                    ]
                    .align_items(Alignment::Center)
                ).into()
            ]),
        ]
        .spacing(16)
        .into()
    }

    fn performance_tab(&self) -> Element<PreferencesMessage> {
        column![
            self.create_section("Rendering", vec![
                self.create_checkbox("GPU acceleration", self.preferences.performance.gpu_acceleration, PreferencesMessage::GpuAccelerationToggled),
                self.create_checkbox("Lazy rendering", self.preferences.performance.lazy_rendering, PreferencesMessage::LazyRenderingToggled),
                self.create_slider("Max FPS", self.preferences.performance.max_fps as f32, 30.0, 144.0, |v| PreferencesMessage::MaxFpsChanged(v as u32)),
            ]),
            self.create_section("Memory", vec![
                self.create_slider("Memory limit (MB)", self.preferences.performance.memory_limit_mb as f32, 128.0, 2048.0, |v| PreferencesMessage::MemoryLimitChanged(v as usize)),
                self.create_slider("Buffer size", self.preferences.performance.buffer_size as f32, 1000.0, 50000.0, |v| PreferencesMessage::BufferSizeChanged(v as usize)),
            ]),
        ]
        .spacing(16)
        .into()
    }

    fn keybindings_tab(&self) -> Element<PreferencesMessage> {
        let keybindings = vec![
            ("New Tab", "new_tab", &self.keybindings.new_tab),
            ("Close Tab", "close_tab", &self.keybindings.close_tab),
            ("Next Tab", "next_tab", &self.keybindings.next_tab),
            ("Previous Tab", "prev_tab", &self.keybindings.prev_tab),
            ("Clear Screen", "clear_screen", &self.keybindings.clear_screen),
            ("Copy", "copy", &self.keybindings.copy),
            ("Paste", "paste", &self.keybindings.paste),
            ("Search", "search", &self.keybindings.search),
            ("Preferences", "preferences", &self.keybindings.preferences),
            ("Theme Selector", "theme_selector", &self.keybindings.theme_selector),
        ];

        let keybinding_inputs: Vec<Element<PreferencesMessage>> = keybindings
            .into_iter()
            .map(|(label, action, binding)| {
                container(
                    row![
                        text(label).width(Length::Fixed(150.0)),
                        text_input("", binding)
                            .on_input(move |new_binding| PreferencesMessage::KeyBindingChanged(action.to_string(), new_binding))
                            .width(Length::Fixed(200.0)),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(8)
                ).into()
            })
            .collect();

        column(keybinding_inputs)
            .spacing(8)
            .into()
    }

    fn notifications_tab(&self) -> Element<PreferencesMessage> {
        column![
            self.create_section("Notification Settings", vec![
                self.create_checkbox("Enable notifications", self.preferences.notification_settings.enable_notifications, PreferencesMessage::NotificationToggled),
                self.create_checkbox("Command completion notifications", self.preferences.notification_settings.command_completion, PreferencesMessage::CommandCompletionToggled),
                self.create_checkbox("Error notifications", self.preferences.notification_settings.error_notifications, PreferencesMessage::ErrorNotificationToggled),
                self.create_checkbox("Sound notifications", self.preferences.notification_settings.sound_enabled, PreferencesMessage::SoundToggled),
            ]),
        ]
        .spacing(16)
        .into()
    }

    fn create_section(&self, title: &str, items: Vec<Element<PreferencesMessage>>) -> Element<PreferencesMessage> {
        container(
            column![
                text(title).size(18),
                column(items).spacing(8)
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

    fn create_checkbox(&self, label: &str, checked: bool, message: fn(bool) -> PreferencesMessage) -> Element<PreferencesMessage> {
        container(
            checkbox(label, checked)
                .on_toggle(message)
        ).into()
    }

    fn create_slider<F>(&self, label: &str, value: f32, min: f32, max: f32, message: F) -> Element<PreferencesMessage>
    where
        F: Fn(f32) -> PreferencesMessage + 'static,
    {
        container(
            column![
                row![
                    text(label).width(Length::Fixed(120.0)),
                    text(format!("{:.1}", value)).width(Length::Fixed(50.0)),
                ]
                .align_items(Alignment::Center),
                slider(min..=max, value, message)
                    .width(Length::Fixed(200.0))
            ]
            .spacing(4)
        ).into()
    }

    fn create_text_input(&self, label: &str, value: &str, message: fn(String) -> PreferencesMessage) -> Element<PreferencesMessage> {
        container(
            row![
                text(label).width(Length::Fixed(120.0)),
                text_input("", value)
                    .on_input(message)
                    .width(Length::Fixed(200.0)),
            ]
            .align_items(Alignment::Center)
            .spacing(8)
        ).into()
    }

    fn prompt_tab(&self) -> Element<PreferencesMessage> {
        let prompt_styles = vec![PromptStyle::Warp, PromptStyle::Shell];
        let available_chips = vec![
            "cwd".to_string(),
            "git".to_string(),
            "kubernetes".to_string(),
            "pyenv".to_string(),
            "date".to_string(),
            "time".to_string(),
            // Add more as needed
        ];

        let context_chip_toggles: Vec<Element<PreferencesMessage>> = available_chips
            .into_iter()
            .map(|chip| {
                let is_enabled = self.prompt_settings.context_chips.contains(&chip);
                self.create_checkbox(&format!("{} chip", chip), is_enabled, move |checked| {
                    PreferencesMessage::ContextChipToggled(chip.clone(), checked)
                })
            })
            .collect();

        column![
            self.create_section("Prompt Style", vec![
                container(
                    row![
                        text("Prompt Type:").width(Length::Fixed(120.0)),
                        pick_list(
                            prompt_styles,
                            Some(self.prompt_settings.style.clone()),
                            PreferencesMessage::PromptStyleChanged
                        )
                    ]
                    .align_items(Alignment::Center)
                ).into(),
                self.create_checkbox("Same line prompt", self.prompt_settings.same_line_prompt, PreferencesMessage::SameLinePromptToggled),
            ]),
            self.create_section("Context Chips", context_chip_toggles),
        ]
        .spacing(16)
        .into()
    }
}

impl std::fmt::Display for CursorStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorStyle::Block => write!(f, "Block"),
            CursorStyle::Underline => write!(f, "Underline"),
            CursorStyle::Beam => write!(f, "Beam"),
        }
    }
}

impl std::fmt::Display for PromptStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromptStyle::Warp => write!(f, "Warp Prompt"),
            PromptStyle::Shell => write!(f, "Shell Prompt (PS1)"),
        }
    }
}
