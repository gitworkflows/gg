use iced::{
    widget::{column, container, scrollable, row, button, stack, Column, Row, Text},
    Element, Length, Command, Subscription, Theme, Color,
};
use iced::futures::{self, SinkExt, StreamExt};
use std::collections::VecDeque;
use tokio::sync::mpsc;
use uuid::Uuid;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Instant;

use crate::block::{Block, BlockContent, BlockMessage}; // Updated import
use crate::input::{Editor, EditorMessage}; // Updated import
use crate::shell::{Shell, ShellMessage, ShellOutput};
use crate::fuzzy::FuzzyMatcher;
use crate::collaboration::CollaborationManager;
use crate::config::yaml_theme_manager::YamlThemeManager; // Updated import
use crate::config::theme::WarpTheme; // Updated import
use crate::settings::{PreferencesWindow, PreferencesMessage}; // Updated import
use crate::settings::theme_editor::{ThemeEditor, ThemeEditorMessage}; // Updated import
use crate::config::{ConfigManager, UserPreferences, KeyBindings, PromptSettings}; // Updated import
use crate::profiles::{ProfileManager, UserProfile};
use crate::profile_manager_ui::{ProfileManagerUI, ProfileManagerMessage};
use crate::profile_switcher::{ProfileSwitcher, ProfileSwitcherMessage};
use crate::workflows::{WorkflowManager, Workflow, WorkflowExecutor, WorkflowUI}; // Updated imports
use crate::workflows::executor::WorkflowExecutorMessage; // Updated import
use crate::workflows::ui::WorkflowUIMessage; // Updated import
use crate::watcher::{FileWatcherService, FileWatcherEvent};
use crate::prompt::PromptRenderer;
use crate::command_palette::{CommandPalette, CommandPaletteMessage, CommandPaletteItem};
use crate::warp_drive_ui::{WarpDriveUI, WarpDriveMessage, WarpDriveItem};
use crate::config::preferences::PreferencesManager;
use crate::settings::keybinding_editor::{KeybindingEditor, KeybindingMessage};
use crate::settings::yaml_theme_ui::{YamlThemeUI, YamlThemeMessage};

pub struct WarpTerminal {
    // Core state
    blocks: VecDeque<Block>,
    current_block_id: Uuid,
    
    // Advanced features
    shell: Shell,
    fuzzy_matcher: FuzzyMatcher,
    collaboration: CollaborationManager,
    yaml_theme_manager: YamlThemeManager, // Renamed
    
    // UI state
    theme: WarpTheme,
    
    // Configuration and preferences
    config_manager: ConfigManager,
    preferences_window: PreferencesWindow,
    theme_editor: ThemeEditor, // Renamed
    preferences_manager: PreferencesManager,
    keybinding_editor: KeybindingEditor,
    yaml_theme_ui: YamlThemeUI,
    
    // Profile management
    profile_manager: ProfileManager,
    profile_manager_ui: ProfileManagerUI,
    profile_switcher: ProfileSwitcher,

    // Workflow system
    workflow_manager: WorkflowManager,
    workflow_ui: WorkflowUI, // Renamed
    workflow_executor: WorkflowExecutor,

    // Prompt rendering
    prompt_renderer: PromptRenderer,
    
    // Command Palette
    command_palette: CommandPalette,
    show_command_palette: bool,

    // Warp Drive
    warp_drive_ui: WarpDriveUI,
    show_warp_drive: bool,

    // Keybinding Editor
    show_keybinding_editor: bool,

    // Theme Editor
    show_theme_editor: bool,

    // YAML Theme UI
    show_yaml_theme_ui: bool,

    // Context menu state
    active_context_menu_block_id: Option<Uuid>, // New field to track active context menu

    // File watcher service instance (to keep it alive)
    _file_watcher_service: FileWatcherService,

    // Editor
    editor: Editor,
}

#[derive(Debug, Clone)]
pub enum TerminalMessage {
    ToggleWarpDrive,
    CommandPalette(crate::command_palette::CommandPaletteMessage),
    WarpDrive(WarpDriveMessage),
    KeybindingEditor(KeybindingMessage),
    ThemeEditor(ThemeEditorMessage),
    YamlThemeUI(YamlThemeMessage),
    // Add other terminal-wide messages
}

impl WarpTerminal {
    pub fn new(workflow_manager: WorkflowManager, yaml_theme_manager: YamlThemeManager) -> Self {
        let config_manager_result = ConfigManager::new();
        let profile_manager_result = ProfileManager::new();
        let workflow_manager_result = WorkflowManager::new();

        let (config_manager, config_cmd) = match config_manager_result {
            Ok(manager) => (manager, Command::none()),
            Err(e) => {
                eprintln!("Failed to load config: {}", e);
                (ConfigManager::default(), Command::perform(async { Err(e) }, |e| TerminalMessage::ConfigLoaded(Err(e))))
            }
        };

        let (mut profile_manager, profile_cmd) = match profile_manager_result {
            Ok(manager) => (manager, Command::none()),
            Err(e) => {
                eprintln!("Failed to load profiles: {}", e);
                (ProfileManager::default(), Command::none()) // No specific message for profile load error for now
            }
        };

        let (workflow_manager, workflow_cmd) = match workflow_manager_result {
            Ok(manager) => (manager, Command::none()),
            Err(e) => {
                eprintln!("Failed to load workflows: {}", e);
                (WorkflowManager::default(), Command::none()) // Fallback to default
            }
        };

        let mut yaml_theme_manager = yaml_theme_manager; // Renamed

        let initial_theme = WarpTheme::from_name(&config_manager.get_config().theme);
        let prompt_settings = config_manager.get_prompt_settings().clone();

        let mut blocks = VecDeque::new();
        let initial_block_id = Uuid::new_v4();
        blocks.push_back(Block::new_command(initial_block_id, "".to_string(), "".to_string())); // Initial empty block for prompt

        let initial_workflows: Vec<WarpDriveItem> = workflow_manager.get_all_workflows().into_iter()
            .map(|w| WarpDriveItem::Workflow { id: w.id, name: w.name.clone() })
            .collect();

        let initial_folders = vec![
            "Scripts".to_string(),
            "Notes".to_string(),
            "Configs".to_string(),
        ];

        let (mut file_watcher_service, file_watcher_event_receiver) = FileWatcherService::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize file watcher service: {}", e);
            let (tx, rx) = mpsc::unbounded_channel();
            (FileWatcherService::new_dummy(), rx)
        });

        let current_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        if let Err(e) = file_watcher_service.watch(&current_directory) {
            eprintln!("Failed to start watching current directory: {}", e);
        }

        let preferences = config_manager.get_preferences().clone();
        let config = config_manager.get_config();

        let preferences_window = PreferencesWindow::new(
            preferences,
            config.font_family.clone(),
            config.font_size,
            config.shell.clone(),
            config.keybindings.clone(),
            config.prompt.clone(),
            yaml_theme_manager.get_available_themes(), // Pass available themes
            yaml_theme_manager.get_current_theme().clone(), // Pass current theme
        );

        let theme_editor = ThemeEditor::new(initial_theme); // Renamed

        let mut profile_manager_ui = ProfileManagerUI::new();
        let mut profile_switcher = ProfileSwitcher::new();

        let profiles = profile_manager.get_all_profiles().into_iter().cloned().collect();
        let active_profile_id = profile_manager.get_active_profile().map(|p| p.id).unwrap_or_else(|| Uuid::new_v4());
        let quick_switch_profiles = profile_manager.get_quick_switch_profiles().into_iter().map(|p| p.id).collect();
        
        profile_manager_ui.update_profiles(profiles.clone(), active_profile_id, quick_switch_profiles);
        profile_switcher.update_profiles(
            profile_manager.get_quick_switch_profiles().into_iter().cloned().collect(),
            active_profile_id
        );

        let mut workflow_ui = WorkflowUI::new(); // Renamed
        workflow_ui.update_workflows(&workflow_manager);

        let preferences_manager = PreferencesManager::new();
        let initial_theme = WarpTheme::default_dark(); // Or load from preferences
        let keybinding_editor = KeybindingEditor::new(preferences_manager.clone());
        let yaml_theme_ui = YamlThemeUI::new(yaml_theme_manager.clone());

        WarpTerminal {
            blocks,
            current_block_id: initial_block_id,
            
            shell: Shell::new(),
            fuzzy_matcher: FuzzyMatcher::new(),
            collaboration: CollaborationManager::new(),
            yaml_theme_manager, // Renamed
            
            theme: initial_theme,
            
            config_manager,
            preferences_window,
            theme_editor, // Renamed
            
            profile_manager,
            profile_manager_ui,
            profile_switcher,

            workflow_manager,
            workflow_ui, // Renamed
            workflow_executor: WorkflowExecutor::new(),
            prompt_renderer: PromptRenderer::new(prompt_settings),
            command_palette: CommandPalette::new(),
            show_command_palette: false,
            warp_drive_ui: WarpDriveUI::new(initial_workflows, initial_folders),
            show_warp_drive: false,
            show_keybinding_editor: false,
            keybinding_editor,
            show_theme_editor: false,
            theme_editor,
            show_yaml_theme_ui: false,
            yaml_theme_ui,
            active_context_menu_block_id: None,
            _file_watcher_service: file_watcher_service,
            editor: Editor::new(),
        }
    }

    pub fn update(&mut self, message: TerminalMessage) {
        match message {
            TerminalMessage::ToggleWarpDrive => {
                self.show_warp_drive = !self.show_warp_drive;
                println!("Warp Drive visibility: {}", self.show_warp_drive);
            }
            TerminalMessage::CommandPalette(msg) => {
                self.command_palette.update(msg);
            }
            TerminalMessage::WarpDrive(msg) => {
                self.warp_drive_ui.update(msg);
            }
            TerminalMessage::KeybindingEditor(msg) => {
                self.keybinding_editor.update(msg);
            }
            TerminalMessage::ThemeEditor(msg) => {
                self.theme_editor.update(msg);
            }
            TerminalMessage::YamlThemeUI(msg) => {
                self.yaml_theme_ui.update(msg);
            }
        }
    }

    pub fn view(&self) -> Element<TerminalMessage> {
        let mut content = Column::new()
            .push(Text::new("Warp Terminal Main View").size(30))
            .spacing(20);

        if self.show_warp_drive {
            content = content.push(
                Container::new(self.warp_drive_ui.view().map(TerminalMessage::WarpDrive))
                    .width(iced::Length::Fill)
                    .height(iced::Length::FillPortion(0.5))
                    .center_x()
                    .center_y()
                    .style(iced::theme::Container::Box)
            );
        }

        if self.show_command_palette {
            content = content.push(
                Container::new(self.command_palette.view().map(TerminalMessage::CommandPalette))
                    .width(iced::Length::Fill)
                    .height(iced::Length::Shrink)
                    .center_x()
                    .style(iced::theme::Container::Box)
            );
        }

        if self.show_keybinding_editor {
            content = content.push(
                Container::new(self.keybinding_editor.view().map(TerminalMessage::KeybindingEditor))
                    .width(iced::Length::Fill)
                    .height(iced::Length::Shrink)
                    .center_x()
                    .style(iced::theme::Container::Box)
            );
        }

        if self.show_theme_editor {
            content = content.push(
                Container::new(self.theme_editor.view().map(TerminalMessage::ThemeEditor))
                    .width(iced::Length::Fill)
                    .height(iced::Length::Shrink)
                    .center_x()
                    .style(iced::theme::Container::Box)
            );
        }

        if self.show_yaml_theme_ui {
            content = content.push(
                Container::new(self.yaml_theme_ui.view().map(TerminalMessage::YamlThemeUI))
                    .width(iced::Length::Fill)
                    .height(iced::Length::Shrink)
                    .center_x()
                    .style(iced::theme::Container::Box)
            );
        }

        Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    pub fn preferences(&self) -> &PreferencesManager {
        &self.preferences_manager
    }

    pub fn key_bindings(&self) -> &KeyBindings {
        &self.preferences_manager.get_preferences().key_bindings
    }
}

impl FileWatcherService {
    fn new_dummy() -> Self {
        let (tx, _rx) = mpsc::unbounded_channel();
        FileWatcherService {
            watcher: notify::RecommendedWatcher::new(move |res| { /* do nothing */ }, notify::Config::default()).unwrap(),
            event_receiver: tx.subscribe(),
        }
    }
}
