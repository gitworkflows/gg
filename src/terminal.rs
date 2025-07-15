use iced::{executor, Application, Command, Element, Theme, Subscription};
use iced::widget::{column, container, scrollable};
use std::collections::VecDeque;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::blocks::{Block, BlockContent, BlockMessage};
use crate::editor::{EnhancedTextInput, EditorMessage};
use crate::shell::{ShellManager, ShellMessage};
use crate::fuzzy::FuzzyMatcher;
use crate::collaboration::CollaborationManager;

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
    
    // UI state
    scroll_offset: f32,
    theme: Theme,
    
    // Communication channels
    shell_sender: Option<mpsc::UnboundedSender<ShellMessage>>,
    shell_receiver: Option<mpsc::UnboundedReceiver<String>>,
    
    // Performance tracking
    frame_count: u64,
    last_render_time: std::time::Instant,
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
    
    // UI events
    ScrollChanged(f32),
    ThemeChanged(Theme),
    
    // System events
    Tick,
    FileChanged(PathBuf),
}

impl Application for WarpTerminal {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (shell_sender, shell_receiver) = mpsc::unbounded_channel();
        
        let terminal = WarpTerminal {
            blocks: VecDeque::new(),
            current_input: String::new(),
            input_history: VecDeque::new(),
            history_index: None,
            
            shell_manager: ShellManager::new(),
            fuzzy_matcher: FuzzyMatcher::new(),
            collaboration: CollaborationManager::new(),
            
            scroll_offset: 0.0,
            theme: Theme::Dark,
            
            shell_sender: Some(shell_sender),
            shell_receiver: Some(shell_receiver),
            
            frame_count: 0,
            last_render_time: std::time::Instant::now(),
        };

        let initial_command = Command::batch([
            Command::perform(async {}, |_| Message::Tick),
        ]);

        (terminal, initial_command)
    }

    fn title(&self) -> String {
        format!("Warp Terminal - {} blocks", self.blocks.len())
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
            
            Message::HistoryUp => {
                if !self.input_history.is_empty() {
                    let new_index = match self.history_index {
                        None => self.input_history.len() - 1,
                        Some(i) if i > 0 => i - 1,
                        Some(i) => i,
                    };
                    
                    self.history_index = Some(new_index);
                    self.current_input = self.input_history[new_index].clone();
                }
                Command::none()
            }
            
            Message::HistoryDown => {
                match self.history_index {
                    Some(i) if i < self.input_history.len() - 1 => {
                        self.history_index = Some(i + 1);
                        self.current_input = self.input_history[i + 1].clone();
                    }
                    Some(_) => {
                        self.history_index = None;
                        self.current_input.clear();
                    }
                    None => {}
                }
                Command::none()
            }
            
            Message::Tick => {
                self.frame_count += 1;
                Command::none()
            }
            
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let blocks_view = scrollable(
            column(
                self.blocks
                    .iter()
                    .map(|block| block.view())
                    .collect::<Vec<_>>()
            )
            .spacing(8)
        )
        .height(iced::Length::Fill);

        let input_view = self.create_input_view();

        column![
            blocks_view,
            input_view
        ]
        .spacing(4)
        .padding(8)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_millis(16))
            .map(|_| Message::Tick)
    }
}

impl WarpTerminal {
    fn create_input_view(&self) -> Element<Message> {
        let input = text_input("Enter command...", &self.current_input)
            .on_input(Message::InputChanged)
            .on_submit(Message::InputSubmitted)
            .padding(8)
            .size(16);

        container(input)
            .width(iced::Length::Fill)
            .padding(4)
            .into()
    }
}
