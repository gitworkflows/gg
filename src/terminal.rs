use iced::{executor, Application, Command, Element, Theme, Subscription};
use iced::widget::{column, container, scrollable, row, button, stack};
use std::collections::VecDeque;
use tokio::sync::mpsc;
use uuid::Uuid;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::blocks::{Block, BlockContent, BlockMessage};
use crate::editor::{EnhancedTextInput, EditorMessage, text_input};
use crate::shell::{ShellManager, ShellMessage};
use crate::fuzzy_match::FuzzyMatcher; // Updated import path
use crate::collaboration::CollaborationManager;
use crate::themes::{ThemeManager, WarpTheme};
use crate::preferences::{PreferencesWindow, PreferencesMessage};
use crate::theme_customizer::{ThemeCustomizer, ThemeCustomizerMessage};
use crate::config::{ConfigManager, UserPreferences, KeyBindings};
use crate::profiles::{ProfileManager, UserProfile};
use crate::profile_manager_ui::{ProfileManagerUI, ProfileManagerMessage};
use crate::profile_switcher::{ProfileSwitcher, ProfileSwitcherMessage};
use crate::workflows::{WorkflowManager, Workflow};
use crate::workflow_browser::{WorkflowBrowser, WorkflowBrowserMessage};
use crate::workflow_executor::{WorkflowExecutor, WorkflowExecutorMessage};
use crate::watcher::{FileWatcherService, FileWatcherEvent}; // New import

pub struct WarpTerminal {
    // Core state
    blocks: VecDeque<Block>,
    current_input: String,
    input_history: VecDeque<String>,
    history_index: Option<usize>,
    
    // Advanced features
    shell_manager: ShellManager,
    fuzzy_matcher: FuzzyMatcher,
    collaboration: CollaborationManager,
    theme_manager: ThemeManager,
    
    // UI state
    scroll_offset: f32,
    theme: Theme,
    
    // Communication channels
    shell_sender: Option<mpsc::UnboundedSender<ShellMessage>>,
    shell_receiver: Option<mpsc::UnboundedReceiver<String>>,
    file_watcher_event_receiver: Option<mpsc::UnboundedReceiver<FileWatcherEvent>>, // New field
    
    // Performance tracking
    frame_count: u64,
    last_render_time: std::time::Instant,

    // Configuration and preferences
    config_manager: ConfigManager,
    preferences_window: PreferencesWindow,
    theme_customizer: ThemeCustomizer,
    
    // Profile management
    profile_manager: ProfileManager,
    profile_manager_ui: ProfileManagerUI,
    profile_switcher: ProfileSwitcher,
    current_directory: PathBuf,

    // Workflow system
    workflow_manager: WorkflowManager,
    workflow_browser: WorkflowBrowser,
    workflow_executor: WorkflowExecutor,

    // File watcher service instance (to keep it alive)
    _file_watcher_service: FileWatcherService,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Input handling
    InputChanged(String),
    InputSubmitted,
    HistoryUp,
    HistoryDown,
    
    // Block management
    BlockAdded(Block),
    BlockUpdated(Uuid, BlockContent),
    BlockRemoved(Uuid),
    
    // Shell integration
    CommandExecuted(String),
    CommandOutput(Uuid, String),
    CommandCompleted(Uuid, i32),
    
    // Advanced features
    FuzzySearch(String),
    SuggestionSelected(String),
    
    // Collaboration
    CollaborationMessage(String),
    
    // Theme management
    ThemeChanged(String),
    ThemeReloaded,
    ThemeListRequested,
    
    // UI events
    ScrollChanged(f32),
    
    // System events
    Tick,
    FileChanged(PathBuf),
    DirectoryChanged(PathBuf),
    FileWatcherEvent(FileWatcherEvent), // New message variant for file watcher events

    // Preferences
    PreferencesOpened,
    PreferencesClosed,
    PreferencesMessage(PreferencesMessage),
    PreferencesSaved {
        preferences: UserPreferences,
        font_family: String,
        font_size: u16,
        shell: String,
        keybindings: KeyBindings,
    },

    // Theme customization
    ThemeCustomizerOpened,
    ThemeCustomizerClosed,
    ThemeCustomizerMessage(ThemeCustomizerMessage),
    ThemePreviewUpdated(WarpTheme),
    ThemePreviewDisabled,
    LoadBaseThemeForCustomization(String),
    SaveCustomTheme(String, WarpTheme),
    ExportTheme(WarpTheme),
    ImportTheme(String),

    // Profile management
    ProfileManagerOpened,
    ProfileManagerClosed,
    ProfileManagerMessage(ProfileManagerMessage),
    ProfileSwitcherMessage(ProfileSwitcherMessage),
    SwitchProfile(Uuid),
    CreateProfile(String, Option<String>),
    DuplicateProfile(Uuid),
    DeleteProfile(Uuid),
    SaveProfile(UserProfile),
    ExportProfile(Uuid),
    ImportProfile,
    AddToQuickSwitch(Uuid),
    RemoveFromQuickSwitch(Uuid),
    CheckAutoSwitchRules,

    // Workflow management
    WorkflowBrowserOpened,
    WorkflowBrowserClosed,
    WorkflowBrowserMessage(WorkflowBrowserMessage),
    WorkflowExecutorMessage(WorkflowExecutorMessage),
    ExecuteWorkflow(Uuid),
    ExecuteWorkflowWithArguments(Uuid, HashMap<String, String>),
    AddWorkflowToFavorites(Uuid),
    RemoveWorkflowFromFavorites(Uuid),
    EditWorkflow(Uuid),
    DeleteWorkflow(Uuid),
    CreateNewWorkflow,
    ImportWorkflow,
    ExportWorkflow(Uuid),
    RefreshWorkflows,
}

impl Application for WarpTerminal {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (shell_sender, shell_receiver) = mpsc::unbounded_channel();
        
        let theme_manager = ThemeManager::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize theme manager: {}", e);
            ThemeManager::new().unwrap()
        });

        let config_manager = ConfigManager::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize config manager: {}", e);
            ConfigManager::new().unwrap()
        });

        let mut profile_manager = ProfileManager::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize profile manager: {}", e);
            ProfileManager::new().unwrap()
        });

        let preferences = config_manager.get_preferences().clone();
        let config = config_manager.get_config();

        let preferences_window = PreferencesWindow::new(
            preferences,
            config.font_family.clone(),
            config.font_size,
            config.shell.clone(),
            config.keybindings.clone(),
        );

        let theme_customizer = ThemeCustomizer::new(
            theme_manager.get_available_themes(),
            theme_manager.get_current_theme().clone(),
        );

        let mut profile_manager_ui = ProfileManagerUI::new();
        let mut profile_switcher = ProfileSwitcher::new();

        // Update UI with current profile data
        let profiles = profile_manager.get_all_profiles().into_iter().cloned().collect();
        let active_profile_id = profile_manager.get_active_profile().map(|p| p.id).unwrap_or_else(|| Uuid::new_v4());
        let quick_switch_profiles = profile_manager.get_quick_switch_profiles().into_iter().map(|p| p.id).collect();
        
        profile_manager_ui.update_profiles(profiles.clone(), active_profile_id, quick_switch_profiles);
        profile_switcher.update_profiles(
            profile_manager.get_quick_switch_profiles().into_iter().cloned().collect(),
            active_profile_id
        );

        let current_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));

        let workflow_manager = WorkflowManager::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize workflow manager: {}", e);
            WorkflowManager::new().unwrap()
        });

        let mut workflow_browser = WorkflowBrowser::new();
        workflow_browser.update_workflows(&workflow_manager);

        let workflow_executor = WorkflowExecutor::new();

        // Initialize FileWatcherService
        let (mut file_watcher_service, file_watcher_event_receiver) = FileWatcherService::new().unwrap_or_else(|e| {
            eprintln!("Failed to initialize file watcher service: {}", e);
            // Fallback to a dummy service and receiver if initialization fails
            let (tx, rx) = mpsc::unbounded_channel();
            (FileWatcherService::new_dummy(), rx)
        });

        // Start watching the current directory (or home directory)
        if let Err(e) = file_watcher_service.watch(&current_directory) {
            eprintln!("Failed to start watching current directory: {}", e);
        }
        
        let terminal = WarpTerminal {
            blocks: VecDeque::new(),
            current_input: String::new(),
            input_history: VecDeque::new(),
            history_index: None,
            
            shell_manager: ShellManager::new(),
            fuzzy_matcher: FuzzyMatcher::new(),
            collaboration: CollaborationManager::new(),
            theme_manager,
            
            scroll_offset: 0.0,
            theme: Theme::Dark,
            
            shell_sender: Some(shell_sender),
            shell_receiver: Some(shell_receiver),
            file_watcher_event_receiver: Some(file_watcher_event_receiver), // Store the receiver
            
            frame_count: 0,
            last_render_time: std::time::Instant::now(),

            config_manager,
            preferences_window,
            theme_customizer,
            
            profile_manager,
            profile_manager_ui,
            profile_switcher,
            current_directory,

            workflow_manager,
            workflow_browser,
            workflow_executor,
            _file_watcher_service: file_watcher_service, // Store the service instance
        };

        let initial_command = Command::batch([
            Command::perform(async {}, |_| Message::Tick),
            Command::perform(async {}, |_| Message::CheckAutoSwitchRules),
        ]);

        (terminal, initial_command)
    }

    fn title(&self) -> String {
        let profile_name = self.profile_manager
            .get_active_profile()
            .map(|p| p.name.as_str())
            .unwrap_or("Unknown");
        format!("Warp Terminal [{}] - {} blocks", profile_name, self.blocks.len())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(input) => {
                self.current_input = input.clone();
                
                // Trigger fuzzy matching for suggestions
                let suggestions = self.fuzzy_matcher.get_suggestions(&input);
                
                Command::none()
            }
            
            Message::InputSubmitted => {
                if !self.current_input.trim().is_empty() {
                    let command = self.current_input.clone();
                    let block_id = Uuid::new_v4();
                    
                    // Add command to history
                    self.input_history.push_back(command.clone());
                    if self.input_history.len() > 1000 {
                        self.input_history.pop_front();
                    }
                    
                    // Create new command block
                    let block = Block::new_command(block_id, command.clone());
                    self.blocks.push_back(block);
                    
                    // Clear input
                    self.current_input.clear();
                    self.history_index = None;
                    
                    // Execute command
                    Command::perform(
                        self.shell_manager.execute_command(command),
                        move |output| Message::CommandOutput(block_id, output)
                    )
                } else {
                    Command::none()
                }
            }
            
            Message::CommandOutput(block_id, output) => {
                if let Some(block) = self.blocks.iter_mut().find(|b| b.id == block_id) {
                    block.append_output(output);
                }
                Command::none()
            }
            
            Message::DirectoryChanged(new_dir) => {
                self.current_directory = new_dir;
                Command::perform(async {}, |_| Message::CheckAutoSwitchRules)
            }

            Message::CheckAutoSwitchRules => {
                if let Some(target_profile_id) = self.profile_manager.check_auto_switch_rules(&self.current_directory) {
                    if target_profile_id != self.profile_manager.get_active_profile().map(|p| p.id).unwrap_or_else(|| Uuid::new_v4()) {
                        return self.update(Message::SwitchProfile(target_profile_id));
                    }
                }
                Command::none()
            }

            Message::SwitchProfile(profile_id) => {
                if let Err(e) = self.profile_manager.switch_profile(&profile_id) {
                    eprintln!("Failed to switch profile: {}", e);
                    return Command::none();
                }

                // Update theme based on new profile
                if let Some(profile) = self.profile_manager.get_active_profile() {
                    if let Err(e) = self.theme_manager.set_current_theme(&profile.config.theme) {
                        eprintln!("Failed to set theme for profile: {}", e);
                    }
                }

                // Update UI components
                self.update_profile_ui_components();
                
                Command::none()
            }

            Message::CreateProfile(name, description) => {
                match self.profile_manager.create_profile(name, description) {
                    Ok(profile_id) => {
                        self.update_profile_ui_components();
                        Command::none()
                    }
                    Err(e) => {
                        eprintln!("Failed to create profile: {}", e);
                        Command::none()
                    }
                }
            }

            Message::DuplicateProfile(source_id) => {
                match self.profile_manager.duplicate_profile(&source_id, format!("Copy of Profile")) {
                    Ok(_) => {
                        self.update_profile_ui_components();
                        Command::none()
                    }
                    Err(e) => {
                        eprintln!("Failed to duplicate profile: {}", e);
                        Command::none()
                    }
                }
            }

            Message::DeleteProfile(profile_id) => {
                match self.profile_manager.delete_profile(&profile_id) {
                    Ok(_) => {
                        self.update_profile_ui_components();
                        Command::none()
                    }
                    Err(e) => {
                        eprintln!("Failed to delete profile: {}", e);
                        Command::none()
                    }
                }
            }

            Message::SaveProfile(profile) => {
                match self.profile_manager.update_profile(&profile.id, profile) {
                    Ok(_) => {
                        self.update_profile_ui_components();
                        Command::none()
                    }
                    Err(e) => {
                        eprintln!("Failed to save profile: {}", e);
                        Command::none()
                    }
                }
            }

            Message::AddToQuickSwitch(profile_id) => {
                if let Err(e) = self.profile_manager.add_to_quick_switch(profile_id) {
                    eprintln!("Failed to add to quick switch: {}", e);
                }
                self.update_profile_ui_components();
                Command::none()
            }

            Message::RemoveFromQuickSwitch(profile_id) => {
                if let Err(e) = self.profile_manager.remove_from_quick_switch(&profile_id) {
                    eprintln!("Failed to remove from quick switch: {}", e);
                }
                self.update_profile_ui_components();
                Command::none()
            }

            Message::ProfileManagerOpened => {
                self.profile_manager_ui.show();
                Command::none()
            }

            Message::ProfileManagerClosed => {
                self.profile_manager_ui.hide();
                Command::none()
            }

            Message::ProfileManagerMessage(msg) => {
                if let Some(terminal_msg) = self.profile_manager_ui.update(msg) {
                    self.update(terminal_msg)
                } else {
                    Command::none()
                }
            }

            Message::ProfileSwitcherMessage(msg) => {
                match msg {
                    ProfileSwitcherMessage::SwitchProfile(id) => {
                        self.update(Message::SwitchProfile(id))
                    }
                    ProfileSwitcherMessage::OpenProfileManager => {
                        self.update(Message::ProfileManagerOpened)
                    }
                }
            }
            
            Message::ThemeChanged(theme_name) => {
                if let Err(e) = self.theme_manager.set_current_theme(&theme_name) {
                    eprintln!("Failed to set theme '{}': {}", theme_name, e);
                }
                Command::none()
            }

            Message::ThemeReloaded => {
                if let Err(e) = self.theme_manager.load_themes_from_directory() {
                    eprintln!("Failed to reload themes: {}", e);
                }
                Command::none()
            }

            Message::ThemeListRequested => {
                let themes = self.theme_manager.get_available_themes();
                println!("Available themes: {:?}", themes);
                Command::none()
            }

            Message::PreferencesOpened => {
                self.preferences_window.show();
                Command::none()
            }

            Message::PreferencesClosed => {
                self.preferences_window.hide();
                Command::none()
            }

            Message::PreferencesMessage(msg) => {
                if let Some(terminal_msg) = self.preferences_window.update(msg) {
                    self.update(terminal_msg)
                } else {
                    Command::none()
                }
            }

            Message::PreferencesSaved { preferences, font_family, font_size, shell, keybindings } => {
                // Update config
                let mut config = self.config_manager.get_config().clone();
                config.preferences = preferences;
                config.font_family = font_family;
                config.font_size = font_size;
                config.shell = shell;
                config.keybindings = keybindings;
                
                if let Err(e) = self.config_manager.update_config(config) {
                    eprintln!("Failed to save preferences: {}", e);
                }
                
                Command::none()
            }

            Message::ThemeCustomizerOpened => {
                self.theme_customizer.show();
                Command::none()
            }

            Message::ThemeCustomizerClosed => {
                self.theme_customizer.hide();
                Command::none()
            }

            Message::ThemeCustomizerMessage(msg) => {
                if let Some(terminal_msg) = self.theme_customizer.update(msg) {
                    self.update(terminal_msg)
                } else {
                    Command::none()
                }
            }

            Message::ThemePreviewUpdated(theme) => {
                // Temporarily apply theme for preview
                self.theme_customizer.set_theme(theme);
                Command::none()
            }

            Message::SaveCustomTheme(name, theme) => {
                if let Err(e) = self.theme_manager.save_theme(&name, &theme) {
                    eprintln!("Failed to save custom theme: {}", e);
                } else {
                    // Reload themes to include the new custom theme
                    if let Err(e) = self.theme_manager.load_themes_from_directory() {
                        eprintln!("Failed to reload themes: {}", e);
                    }
                }
                Command::none()
            }

            Message::WorkflowBrowserOpened => {
                self.workflow_browser.update_workflows(&self.workflow_manager);
                self.workflow_browser.show();
                Command::none()
            }

            Message::WorkflowBrowserClosed => {
                self.workflow_browser.hide();
                Command::none()
            }

            Message::WorkflowBrowserMessage(msg) => {
                if let Some(terminal_msg) = self.workflow_browser.update(msg) {
                    self.update(terminal_msg)
                } else {
                    Command::none()
                }
            }

            Message::WorkflowExecutorMessage(msg) => {
                if let Some(terminal_msg) = self.workflow_executor.update(msg) {
                    self.update(terminal_msg)
                } else {
                    Command::none()
                }
            }

            Message::ExecuteWorkflow(workflow_id) => {
                if let Some(workflow) = self.workflow_manager.get_workflow(&workflow_id) {
                    if workflow.arguments.is_some() && !workflow.arguments.as_ref().unwrap().is_empty() {
                        // Show executor dialog for workflows with arguments
                        self.workflow_executor.show_workflow(workflow.clone());
                        Command::none()
                    } else {
                        // Execute directly for workflows without arguments
                        match self.workflow_manager.execute_workflow(workflow_id, HashMap::new()) {
                            Ok(command) => {
                                self.update(Message::InputChanged(command.clone())).then(|| {
                                    self.update(Message::InputSubmitted)
                                })
                            }
                            Err(e) => {
                                eprintln!("Failed to execute workflow: {}", e);
                                Command::none()
                            }
                        }
                    }
                } else {
                    Command::none()
                }
            }

            Message::ExecuteWorkflowWithArguments(workflow_id, arguments) => {
                match self.workflow_manager.execute_workflow(workflow_id, arguments) {
                    Ok(command) => {
                        self.current_input = command;
                        self.update(Message::InputSubmitted)
                    }
                    Err(e) => {
                        eprintln!("Failed to execute workflow: {}", e);
                        Command::none()
                    }
                    _ => Command::none(), // Added to handle the case where execute_workflow returns an error
                }
            }

            Message::AddWorkflowToFavorites(workflow_id) => {
                self.workflow_manager.add_to_favorites(workflow_id);
                self.workflow_browser.update_workflows(&self.workflow_manager);
                Command::none()
            }

            Message::RemoveWorkflowFromFavorites(workflow_id) => {
                self.workflow_manager.remove_from_favorites(&workflow_id);
                self.workflow_browser.update_workflows(&self.workflow_manager);
                Command::none()
            }

            Message::RefreshWorkflows => {
                if let Err(e) = self.workflow_manager.load_workflows() {
                    eprintln!("Failed to refresh workflows: {}", e);
                }
                self.workflow_browser.update_workflows(&self.workflow_manager);
                Command::none()
            }

            Message::FileWatcherEvent(event) => {
                match event {
                    FileWatcherEvent::FileChanged(path) => {
                        println!("File changed: {:?}", path);
                        // TODO: Handle file change, e.g., reload config, update display
                    }
                    FileWatcherEvent::FileCreated(path) => {
                        println!("File created: {:?}", path);
                    }
                    FileWatcherEvent::FileDeleted(path) => {
                        println!("File deleted: {:?}", path);
                    }
                    FileWatcherEvent::DirectoryChanged(path) => {
                        println!("Directory changed: {:?}", path);
                        // Potentially re-evaluate auto-switch rules if directory changes
                        return self.update(Message::CheckAutoSwitchRules);
                    }
                    FileWatcherEvent::Error(e) => {
                        eprintln!("File watcher error: {}", e);
                    }
                }
                Command::none()
            }
            
            _ => Command::none(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let shell_subscription = self.shell_receiver.as_ref().map(|mut receiver| {
            iced::Subscription::batch(vec![
                iced::Subscription::run("shell_output_receiver", async move {
                    loop {
                        if let Some(output) = receiver.recv().await {
                            // This part needs to be adjusted as CommandOutput expects Uuid and String
                            // For now, just print to demonstrate
                            println!("Shell output received: {}", output);
                        }
                    }
                })
            ])
        }).unwrap_or(Subscription::none());

        let file_watcher_subscription = self.file_watcher_event_receiver.as_ref().map(|mut receiver| {
            iced::Subscription::batch(vec![
                iced::Subscription::run("file_watcher_events", async move {
                    loop {
                        if let Some(event) = receiver.recv().await {
                            yield Message::FileWatcherEvent(event);
                        }
                    }
                })
            ])
        }).unwrap_or(Subscription::none());

        Subscription::batch(vec![
            iced::time::every(std::time::Duration::from_millis(16))
                .map(|_| Message::Tick),
            shell_subscription,
            file_watcher_subscription,
        ])
    }
}

impl WarpTerminal {
    fn create_input_view(&self, theme: &WarpTheme) -> Element<Message> {
        let input = text_input("Enter command...", &self.current_input)
            .on_input(Message::InputChanged)
            .on_submit(Message::InputSubmitted)
            .padding(8)
            .size(16)
            .style(move |iced_theme: &iced::Theme, status| {
                text_input::Appearance {
                    background: iced::Background::Color(theme.get_block_background_color(theme.is_dark_theme())),
                    border: iced::Border {
                        color: theme.get_border_color(),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    icon_color: theme.get_foreground_color(),
                    placeholder_color: theme.get_terminal_color("white", false),
                    value_color: theme.get_foreground_color(),
                    selection_color: theme.get_accent_color(),
                }
            });

        container(input)
            .width(iced::Length::Fill)
            .padding(4)
            .into()
    }

    fn create_modal_overlay<T>(&self, content: Element<T>) -> Element<T> {
        container(content)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .style(|_theme: &iced::Theme| {
                container::Appearance {
                    background: Some(iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                    ..Default::default()
                }
            })
            .into()
    }

    fn update_profile_ui_components(&mut self) {
        let profiles = self.profile_manager.get_all_profiles().into_iter().cloned().collect();
        let active_profile_id = self.profile_manager.get_active_profile().map(|p| p.id).unwrap_or_else(|| Uuid::new_v4());
        let quick_switch_profiles = self.profile_manager.get_quick_switch_profiles().into_iter().map(|p| p.id).collect();
        
        self.profile_manager_ui.update_profiles(profiles, active_profile_id, quick_switch_profiles);
        self.profile_switcher.update_profiles(
            self.profile_manager.get_quick_switch_profiles().into_iter().cloned().collect(),
            active_profile_id
        );
    }
}

// Dummy implementation for FileWatcherService::new_dummy()
impl FileWatcherService {
    fn new_dummy() -> Self {
        // Create a dummy watcher that does nothing
        let (tx, _rx) = mpsc::unbounded_channel();
        FileWatcherService {
            watcher: notify::RecommendedWatcher::new(move |res| { /* do nothing */ }, notify::Config::default()).unwrap(),
            event_receiver: tx.subscribe(), // Use a broadcast channel for dummy
        }
    }
}
